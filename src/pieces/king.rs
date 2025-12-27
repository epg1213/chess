use std::fmt::Display;
use crate::pieces::{ChessPiece, Color};

#[derive(Debug, Clone)]
pub struct King {
    color: Color
}

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "K"),
            Color::Black => write!(f, "k")
        }
    }
}

impl ChessPiece for King {}
impl King {
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

