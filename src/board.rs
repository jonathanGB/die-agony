type Position = (usize, usize);

const BOARD_WIDTH: usize = 6;
const END_CELL_POSITION: Position = (0, BOARD_WIDTH - 1);

struct Board {
    board: [[i16; BOARD_WIDTH]; BOARD_WIDTH],
}

#[derive(Debug, PartialEq)]
pub struct Cell {
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

    pub fn start_cell(&self) -> Cell {
        Cell {
            value: self.board[BOARD_WIDTH - 1][0],
            position: (BOARD_WIDTH - 1, 0),
        }
    }

    pub fn move_up(&self, curr_cell: &Cell) -> Option<Cell> {
        let (curr_row, curr_col) = curr_cell.position;
        if curr_row > 0 {
            let row_up = curr_row - 1;
            Some(Cell {
                value: self.board[row_up][curr_col],
                position: (row_up, curr_col),
            })
        } else {
            None
        }
    }

    pub fn move_down(&self, curr_cell: &Cell) -> Option<Cell> {
        let (curr_row, curr_col) = curr_cell.position;
        if curr_row < BOARD_WIDTH - 1 {
            let row_down = curr_row + 1;
            Some(Cell {
                value: self.board[row_down][curr_col],
                position: (row_down, curr_col),
            })
        } else {
            None
        }
    }

    pub fn move_left(&self, curr_cell: &Cell) -> Option<Cell> {
        let (curr_row, curr_col) = curr_cell.position;
        if curr_col > 0 {
            let col_left = curr_col - 1;
            Some(Cell {
                value: self.board[curr_row][col_left],
                position: (curr_row, col_left),
            })
        } else {
            None
        }
    }

    pub fn move_right(&self, curr_cell: &Cell) -> Option<Cell> {
        let (curr_row, curr_col) = curr_cell.position;
        if curr_col < BOARD_WIDTH - 1 {
            let col_right = curr_col + 1;
            Some(Cell {
                value: self.board[curr_row][col_right],
                position: (curr_row, col_right),
            })
        } else {
            None
        }
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

        assert!(board.move_left(&cell).is_none());
        assert!(board.move_down(&cell).is_none());
        assert_eq!(
            board.move_up(&cell),
            Some(Cell {
                value: 5,
                position: (4, 0)
            })
        );
        assert_eq!(
            board.move_right(&cell),
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

        assert!(board.move_left(&cell).is_none());
        assert!(board.move_up(&cell).is_none());
        assert_eq!(
            board.move_right(&cell),
            Some(Cell {
                value: 33,
                position: (0, 1)
            })
        );
        assert_eq!(
            board.move_down(&cell),
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

        assert!(board.move_up(&cell).is_none());
        assert!(board.move_right(&cell).is_none());
        assert_eq!(
            board.move_down(&cell),
            Some(Cell {
                value: 508,
                position: (1, BOARD_WIDTH - 1)
            })
        );
        assert_eq!(
            board.move_left(&cell),
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

        assert!(board.move_right(&cell).is_none());
        assert!(board.move_down(&cell).is_none());
        assert_eq!(
            board.move_left(&cell),
            Some(Cell {
                value: 337,
                position: (BOARD_WIDTH - 1, BOARD_WIDTH - 2)
            })
        );
        assert_eq!(
            board.move_up(&cell),
            Some(Cell {
                value: 620,
                position: (BOARD_WIDTH - 2, BOARD_WIDTH - 1)
            })
        );
    }
}
