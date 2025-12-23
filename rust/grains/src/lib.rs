#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidSquare,
}

/// Return the number of grains on square `s` (1-based).
pub fn square(_s: u32) -> Result<u64, Error> {
    unimplemented!("compute grains on a given square");
}

/// Return the total number of grains on the chessboard.
pub fn total() -> u64 {
    unimplemented!("compute total grains on the board");
}
