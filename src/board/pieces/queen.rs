use std::fmt::Display;
use crate::board::pieces::{ChessPiece, Color, bishop::Bishop, rook::Rook};
use crate::board::{Board, Square};

#[derive(Debug, Clone, PartialEq)]
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

impl ChessPiece for Queen {
    fn color(&self) -> Color {
        self.color.clone()
    }
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        match Bishop::new(self.color.clone()).path_eat(from_sq.clone(), to_sq.clone(), board) {
            Some(b) => return Some(b),
            None => Rook::new(self.color.clone()).path_eat(from_sq, to_sq, board)
        }
    }
}

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

