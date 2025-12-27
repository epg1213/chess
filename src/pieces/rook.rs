use std::fmt::Display;
use crate::pieces::{ChessPiece, Color};

#[derive(Debug, Clone)]
pub struct Rook {
    color: Color
}

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "R"),
            Color::Black => write!(f, "r")
        }
    }
}

impl ChessPiece for Rook {}
impl Rook {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
    pub fn white() -> Self {
        Self::new(Color::White)
    }
    pub fn black() -> Self {
        Self::new(Color::Black)
    }
}

