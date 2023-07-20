use super::errors::*;

pub type ChessResult<T> = core::result::Result<T, ChessError>;

pub fn is_valid(idx: usize) -> ChessResult<usize> {
    if idx & 0x88 == 0 && idx < 256 {
        return Ok(idx);
    } else {
        return Err(ChessError::InvalidIndex(idx));
    }
}
