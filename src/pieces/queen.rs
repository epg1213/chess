use std::fmt::Display;
use crate::pieces::{ChessPiece, Color};

#[derive(Debug, Clone)]
pub struct Queen {
    color: Color
}

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "Q"),
            Color::Black => write!(f, "q")
        }
    }
}

impl ChessPiece for Queen {}
impl Queen {
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

