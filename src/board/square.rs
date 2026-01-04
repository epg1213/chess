use std::fmt::Display;
use std::sync::Arc;
use crate::board::pieces::ChessPiece;
use crate::board::{Board, BoardError};

#[derive(Debug, Clone)]
pub struct Square {
    pub piece: Option<Arc<dyn ChessPiece>>,
    pub pawn_trace: bool,
    pub name: String,
    pub x: usize,
    pub y: usize,
}

impl Square {
    pub fn new() -> Self {
        Self {
            piece: None, pawn_trace: false,
            name: String::new(), x: 0, y: 0,
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.piece {
            Some(p) => write!(f, "{}", p),
            None => if self.pawn_trace {
                write!(f, ".")
            } else {
                write!(f, "_")
            }
        }
    }
}

impl Board {
    pub fn square_mut(&mut self, square_name: impl AsRef<str>) -> Result<&mut Square, BoardError> {
        for sq in self.squares.iter_mut() {
            if sq.name.as_str() == square_name.as_ref() {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
    pub fn square_mut_from_coordinates(&mut self, x: usize, y: usize) -> Result<&mut Square, BoardError> {
        for sq in self.squares.iter_mut() {
            if sq.x==x && sq.y==y {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
    pub fn square_as_ref(&self, square_name: impl AsRef<str>) -> Result<&Square, BoardError> {
        for sq in self.squares.iter() {
            if sq.name.as_str() == square_name.as_ref() {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
    pub fn square_as_ref_from_coordinates(&self, x: usize, y: usize) -> Result<&Square, BoardError> {
        for sq in self.squares.iter() {
            if sq.x==x && sq.y==y {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
}
