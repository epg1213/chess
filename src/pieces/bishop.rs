use std::fmt::Display;
use crate::pieces::{ChessPiece, Color};

#[derive(Debug, Clone)]
pub struct Bishop {
    color: Color
}

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "B"),
            Color::Black => write!(f, "b")
        }
    }
}

impl ChessPiece for Bishop {}
impl Bishop {
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

