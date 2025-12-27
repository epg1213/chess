use std::fmt::Display;
use crate::pieces::{ChessPiece, Color};

#[derive(Debug, Clone)]
pub struct Knight {
    color: Color
}

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "N"),
            Color::Black => write!(f, "n")
        }
    }
}

impl ChessPiece for Knight {}
impl Knight {
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

