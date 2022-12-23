use crate::direction::Direction;

use std::collections::HashSet;

/// A position is a (row, column) tuple.
pub(crate) type Position = (usize, usize);

const BOARD_WIDTH: usize = 6;
const END_CELL_POSITION: Position = (0, BOARD_WIDTH - 1);

/// Encapsulates both the value stored in that cell, as well as its position on the board.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Cell {
    value: i16,
    position: Position,
}

impl Cell {
    pub fn get_value(&self) -> i16 {
        self.value
    }

    pub fn is_end_cell(&self) -> bool {
        self.position == END_CELL_POSITION
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }
}

/// Holds a matrix of values of size BOARD_WIDTH x BOARD_WIDTH.
/// The start cell is the bottom-left cell, and the goal is to reach
/// the end cell, at the top-right.
pub(crate) struct Board {
    board: [[i16; BOARD_WIDTH]; BOARD_WIDTH],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [
                [57, 33, 132, 268, 492, 732],
                [81, 123, 240, 443, 353, 508],
                [186, 42, 195, 704, 452, 228],
                [-7, 2, 357, 452, 317, 395],
                [5, 23, -4, 592, 445, 620],
                [0, 77, 32, 403, 337, 452],
            ],
        }
    }

    /// Creates a new cell representing the value and position at the starting position.
    pub fn start_cell(&self) -> Cell {
        Cell {
            value: self.board[BOARD_WIDTH - 1][0],
            position: (BOARD_WIDTH - 1, 0),
        }
    }

    /// Tries to return the cell we land onto after moving from the current cell
    /// in the given direction.
    /// Returns `None` if that movement would be out of bounds.
    pub fn move_in(&self, curr_cell: &Cell, direction: Direction) -> Option<Cell> {
        let (row, col) = curr_cell.position;

        let moved_position = match direction {
            Direction::UP => {
                // Avoid a substraction overflow error.
                if row == 0 {
                    return None;
                }

                (row - 1, col)
            }
            Direction::RIGHT => (row, col + 1),
            Direction::DOWN => (row + 1, col),
            Direction::LEFT => {
                // Avoid a substraction overflow error.
                if col == 0 {
                    return None;
                }

                (row, col - 1)
            }
        };

        self.get_cell_at(moved_position)
    }

    fn get_cell_at(&self, position: Position) -> Option<Cell> {
        let value = *self.board.get(position.0)?.get(position.1)?;
        Some(Cell { value, position })
    }

    pub fn compute_sum_of_unvisited_cells(
        &self,
        unique_visited_positions: &HashSet<&Position>,
    ) -> i16 {
        let mut sum = 0;
        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                let position = (row, col);
                if !unique_visited_positions.contains(&position) {
                    sum += self.board[row][col];
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_moving_from_bottom_left() {
        let board = Board::new();
        let cell = Cell {
            value: board.board[BOARD_WIDTH - 1][0],
            position: (BOARD_WIDTH - 1, 0),
        };
        assert!(!cell.is_end_cell());
        assert_eq!(cell, board.start_cell());

        assert!(board.move_in(&cell, Direction::LEFT).is_none());
        assert!(board.move_in(&cell, Direction::DOWN).is_none());
        assert_eq!(
            board.move_in(&cell, Direction::UP),
            Some(Cell {
                value: 5,
                position: (4, 0)
            })
        );
        assert_eq!(
            board.move_in(&cell, Direction::RIGHT),
            Some(Cell {
                value: 77,
                position: (5, 1)
            })
        );
    }

    #[test]
    fn try_moving_from_top_left() {
        let board = Board::new();
        let cell = Cell {
            value: board.board[0][0],
            position: (0, 0),
        };
        assert!(!cell.is_end_cell());
        assert_ne!(cell, board.start_cell());

        assert!(board.move_in(&cell, Direction::LEFT).is_none());
        assert!(board.move_in(&cell, Direction::UP).is_none());
        assert_eq!(
            board.move_in(&cell, Direction::RIGHT),
            Some(Cell {
                value: 33,
                position: (0, 1)
            })
        );
        assert_eq!(
            board.move_in(&cell, Direction::DOWN),
            Some(Cell {
                value: 81,
                position: (1, 0)
            })
        );
    }

    #[test]
    fn try_moving_from_top_right() {
        let board = Board::new();
        let cell = Cell {
            value: board.board[0][BOARD_WIDTH - 1],
            position: (0, BOARD_WIDTH - 1),
        };
        assert!(cell.is_end_cell());
        assert_ne!(cell, board.start_cell());

        assert!(board.move_in(&cell, Direction::UP).is_none());
        assert!(board.move_in(&cell, Direction::RIGHT).is_none());
        assert_eq!(
            board.move_in(&cell, Direction::DOWN),
            Some(Cell {
                value: 508,
                position: (1, BOARD_WIDTH - 1)
            })
        );
        assert_eq!(
            board.move_in(&cell, Direction::LEFT),
            Some(Cell {
                value: 492,
                position: (0, BOARD_WIDTH - 2)
            })
        );
    }

    #[test]
    fn try_moving_from_bottom_right() {
        let board = Board::new();
        let cell = Cell {
            value: board.board[BOARD_WIDTH - 1][BOARD_WIDTH - 1],
            position: (BOARD_WIDTH - 1, BOARD_WIDTH - 1),
        };
        assert!(!cell.is_end_cell());
        assert_ne!(cell, board.start_cell());

        assert!(board.move_in(&cell, Direction::RIGHT).is_none());
        assert!(board.move_in(&cell, Direction::DOWN).is_none());
        assert_eq!(
            board.move_in(&cell, Direction::LEFT),
            Some(Cell {
                value: 337,
                position: (BOARD_WIDTH - 1, BOARD_WIDTH - 2)
            })
        );
        assert_eq!(
            board.move_in(&cell, Direction::UP),
            Some(Cell {
                value: 620,
                position: (BOARD_WIDTH - 2, BOARD_WIDTH - 1)
            })
        );
    }

    #[test]
    fn compute_sum_of_unvisited_cells_works() {
        let board = Board::new();

        let mut sum_of_all_cells = 0;
        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                sum_of_all_cells += board.board[row][col];
            }
        }

        let mut unique_visited_positions = HashSet::new();
        assert_eq!(
            board.compute_sum_of_unvisited_cells(&unique_visited_positions),
            sum_of_all_cells
        );

        let value_top_left_cell = board.board[0][0];
        let top_left_cell_position = (0, 0);
        unique_visited_positions.insert(&top_left_cell_position);
        assert_eq!(
            board.compute_sum_of_unvisited_cells(&unique_visited_positions),
            sum_of_all_cells - value_top_left_cell
        )
    }
}
