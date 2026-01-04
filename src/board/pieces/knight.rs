use std::fmt::Display;
use crate::board::pieces::{ChessPiece, Color};
use crate::board::{Board, Square};

#[derive(Debug, Clone, PartialEq)]
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

impl ChessPiece for Knight {
    fn color(&self) -> Color {
        self.color.clone()
    }
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        let here = board.square_as_ref(from_sq).ok()?;
        let to = board.square_as_ref(to_sq).ok()?;
        if (here.x.abs_diff(to.x)==1 && here.y.abs_diff(to.y)==2) || (here.y.abs_diff(to.y) == 1 && here.x.abs_diff(to.x) == 2) {
            return Some(vec![to]);
        }
        None
    }
}

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

