use crate::{
    board::{Board, Cell},
    dice::Dice,
    direction::Direction,
};

use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;

/// Represents a candidate solution to the puzzle. The candidate might not have reached the end cell,
/// and might not have not a full knowledge of what values the dice has. Each valid movement
/// creates a new distinct journey, from which we can then potentially create other journeys.
#[derive(Debug)]
struct Journey {
    /// Current disposition of the dice, which values might be partially known.
    dice: Dice,
    /// How many rotations have been applied to the dice throughout this journey.
    turn: i16,
    /// The visited cells are ordered movement by movement, the last of which being
    /// the current cell being visited. This list can never be empty!
    visited_cells: Vec<Cell>,
}

impl Journey {
    pub fn get_last_visited_cell(&self) -> &Cell {
        self.visited_cells
            .last()
            .expect("A journey must have visited at least one cell.")
    }

    fn explain(&self) -> String {
        // First, go backwards through the visited cells. This will help us list the dice movements,
        // and figure out the initial configuration of the dice.
        let mut last_visited_cell = self.get_last_visited_cell();
        let mut dice_movements = Vec::new();
        let mut dice = self.dice.clone();
        for second_to_last_visited_cell in self.visited_cells.iter().rev().skip(1) {
            let (second_to_last_row, second_to_last_col) =
                second_to_last_visited_cell.get_position();
            let (last_row, last_col) = last_visited_cell.get_position();

            // Here, we note the dice movement going from cell n-1 to cell n, and apply the opposite
            // roll to that movement, such that we move the dice from cell n to cell n-1.
            if second_to_last_row < last_row && second_to_last_col == last_col {
                dice_movements.push(Direction::DOWN);
                dice = dice.roll_up();
            } else if second_to_last_row > last_row && second_to_last_col == last_col {
                dice_movements.push(Direction::UP);
                dice = dice.roll_down();
            } else if second_to_last_col < last_col && second_to_last_row == last_row {
                dice_movements.push(Direction::RIGHT);
                dice = dice.roll_left();
            } else if second_to_last_col > last_col && second_to_last_row == last_row {
                dice_movements.push(Direction::LEFT);
                dice = dice.roll_right();
            } else {
                panic!(
                    "dice has to move orthogonally, but got ({},{}) ({},{})",
                    second_to_last_row, second_to_last_col, last_row, last_col
                );
            }

            last_visited_cell = second_to_last_visited_cell;
        }

        let mut explanation = Vec::new();
        explanation.push(format!("We started with the following dice: {:?}", dice));

        // Now that we have made back it the start cell, explain the movements applied from start to end.
        dice_movements.reverse();
        let mut score = 0;
        for (turn, dice_movement) in dice_movements.into_iter().enumerate() {
            dice = dice.roll_in(dice_movement);

            let dice_top = dice.get_top().unwrap();
            let new_score = score + (turn as i16 + 1) * dice_top;
            explanation.push(
            format!(
                "Turn {} we rolled the dice {:?} (top={}). Score was {}, now is `{} + ({} x {}) = {}` (cell value = {}).",
                turn+1,
                dice_movement,
                dice_top, score, score, turn+1, dice_top, new_score, self.visited_cells[turn+1].get_value()
            ));

            score = new_score;
        }

        explanation.join("\n")
    }
}

/// Enumerates the possible outcomes when trying to roll a dice to an orthogonal cell.
enum MovementOutcome {
    /// A journey that made it all the way to the end cell.
    SolutionJourney(Journey),
    /// A journey that is valid, but has not reached the end cell.
    ValidJourney(Journey),
    /// A journey that is invalid, per the puzzle rules.
    Invalid,
}

/// Enumerates the possible outcomes when solving the puzzle.
pub enum Solution {
    /// If found, this holds the sum of unvisited cells, as well as an explanation message.
    Found(i16, String),
    /// No solutions found.
    NotFound,
}

/// Solves the puzzle by using a BFS traversal.
pub struct Solver {
    board: Board,
    /// Keeps track of a FIFO list of all the candidate journeys, one of which should eventually be
    /// a solution to the puzzle.  
    journeys: VecDeque<Journey>,
}

impl Solver {
    /// Initializes a solver.
    pub fn new() -> Self {
        let board = Board::new();
        let first_journey = Journey {
            dice: Dice::default(),
            turn: 0,
            visited_cells: vec![board.start_cell()],
        };

        Self {
            board,
            journeys: VecDeque::from([first_journey]),
        }
    }

    /// Solves the puzzle, which consumes the solver.
    pub fn solve(mut self) -> Solution {
        match self.find_solution_journey() {
            Some(solution_journey) => {
                let sum = self.compute_sum_of_unvisited_cells(&solution_journey);
                Solution::Found(sum, solution_journey.explain())
            }
            None => Solution::NotFound,
        }
    }

    fn compute_sum_of_unvisited_cells(&self, solution_journey: &Journey) -> i16 {
        let unique_visited_positions: HashSet<_> = solution_journey
            .visited_cells
            .iter()
            .map(|cell| cell.get_position())
            .collect();

        self.board
            .compute_sum_of_unvisited_cells(&unique_visited_positions)
    }

    // This is where we actually run the BFS traversal. For each candidate journey popped,
    // we will check whether we can roll the dice up, right, down, and left. If a movement is
    // valid, we push it to the back of the list of candidate journeys, unless the movement
    // leads to the end cell, in which case we return the solution journey.
    fn find_solution_journey(&mut self) -> Option<Journey> {
        while let Some(journey) = self.journeys.pop_front() {
            let last_visited_cell = journey.get_last_visited_cell();
            let new_turn = journey.turn + 1;

            for direction in Direction::iter() {
                // Confirm that a movement in this direction yiels a cell (i.e. not outbounds).
                if let Some(cell) = self.board.move_in(last_visited_cell, direction) {
                    // If we are inbounds after this movement, confirm that moving there is valid,
                    // per the puzzle rules.
                    let rolled_dice = journey.dice.roll_in(direction);
                    match Solver::try_dice_movement(
                        rolled_dice,
                        last_visited_cell.get_value(),
                        new_turn,
                        cell,
                        &journey.visited_cells,
                    ) {
                        MovementOutcome::SolutionJourney(journey) => return Some(journey),
                        MovementOutcome::ValidJourney(journey) => self.journeys.push_back(journey),
                        MovementOutcome::Invalid => {}
                    }
                }
            }
        }

        // Oops, no solution found.
        None
    }

    fn try_dice_movement(
        dice: Dice,
        score: i16,
        new_turn: i16,
        cell: Cell,
        visited_cells: &Vec<Cell>,
    ) -> MovementOutcome {
        // There are two main scenarios when rolling a dice onto a cell:
        //  1. The top value on the dice is known.
        //     In this case, we validate that the new score matches the value of the cell.
        //     If it does not, then we return an INVALID outcome.
        //  2. The top value on the dice is yet unknown.
        //     In this case, we infer an integral top value on the dice resulting in a score matching the
        //     value of the new cell.
        //     If no such integral value exists, then we return an INVALID outcome.
        // If the movement is valid, we finish by checking whether the journey has reached the end cell.
        // If it does, we annotate it as a solution, otherwise as a valid journey worth further traversing.
        let valid_journey = match dice.get_top() {
            Some(dice_top) => {
                let new_score = score + new_turn * dice_top;
                if new_score != cell.get_value() {
                    return MovementOutcome::Invalid;
                }

                let mut new_visited_cells = visited_cells.clone();
                new_visited_cells.push(cell);
                Journey {
                    dice,
                    turn: new_turn,
                    visited_cells: new_visited_cells,
                }
            }
            None => {
                let new_score = cell.get_value();
                let score_diff = new_score - score;
                if score_diff % new_turn != 0 {
                    return MovementOutcome::Invalid;
                }

                let mut new_visited_cells = visited_cells.clone();
                new_visited_cells.push(cell);
                let new_dice_top = score_diff / new_turn;
                Journey {
                    dice: dice.set_top(new_dice_top),
                    turn: new_turn,
                    visited_cells: new_visited_cells,
                }
            }
        };

        if valid_journey.get_last_visited_cell().is_end_cell() {
            MovementOutcome::SolutionJourney(valid_journey)
        } else {
            MovementOutcome::ValidJourney(valid_journey)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Solution {
        /// Return true if a solution was found.
        fn found_solution(&self) -> bool {
            matches!(self, Solution::Found(..))
        }

        /// Returns the sum+explanation tuple contained in the `Found` value, consuming itself.
        ///
        /// # Panics
        /// Panics if no solution was found.
        fn unwrap(self) -> (i16, String) {
            match self {
                Solution::Found(sum, explanation) => (sum, explanation),
                Solution::NotFound => panic!("called `Solution::unwrap()` on a `NotFound` value"),
            }
        }
    }

    fn create_default_journey() -> Journey {
        Journey {
            dice: Dice::default(),
            turn: 0,
            visited_cells: Vec::new(),
        }
    }

    #[test]
    fn compute_sum_of_unvisited_cells_works() {
        let solver = Solver::new();
        let mut journey = create_default_journey();

        assert_eq!(
            solver.compute_sum_of_unvisited_cells(&journey),
            solver.board.compute_sum_of_unvisited_cells(&HashSet::new())
        );

        let visited_position = (3, 2);
        let visited_cell = solver.board.get_cell_at(visited_position.clone()).unwrap();
        journey.visited_cells = vec![visited_cell];

        assert_eq!(
            solver.compute_sum_of_unvisited_cells(&journey),
            solver
                .board
                .compute_sum_of_unvisited_cells(&HashSet::from([&visited_position]))
        );
    }

    #[test]
    fn solver_finds_right_solution() {
        let solution = Solver::new().solve();

        assert!(solution.found_solution());
        let (sum_unvisited_cells, _) = solution.unwrap();
        assert_eq!(sum_unvisited_cells, 1935);
    }
}
