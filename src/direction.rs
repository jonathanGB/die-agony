use strum_macros::EnumIter;

/// Enumerates the orthogonal movements a dice can do on the board.
#[derive(Clone, Copy, Debug, EnumIter)]
pub(crate) enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
