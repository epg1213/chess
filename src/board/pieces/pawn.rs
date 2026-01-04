use std::fmt::Display;
use crate::board::pieces::{ChessPiece, Color};
use crate::board::{Board, Square};

#[derive(Debug, Clone, PartialEq)]
pub struct Pawn {
    color: Color
}

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::White => write!(f, "P"),
            Color::Black => write!(f, "p")
        }
    }
}

impl ChessPiece for Pawn {
    fn is_pawn(&self) -> bool {
        true
    }
    fn color(&self) -> Color {
        self.color.clone()
    }
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        let here = board.square_as_ref(from_sq).ok()?;
        let to = board.square_as_ref(to_sq).ok()?;
        if here.x.abs_diff(to.x) != 1 {return None}
        if (to.y == here.y + 1 && self.color==Color::White) || (here.y == to.y + 1 && self.color==Color::Black) {
            return Some(vec![to]);
        }
        None
    }

    fn path_move<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        let here = board.square_as_ref(from_sq).ok()?;
        let to = board.square_as_ref(to_sq).ok()?;
        if self.color() == Color::White && to.x==here.x && to.y == here.y+1 && to.piece.is_none() {
            return Some(vec![to]);
        }
        if self.color() == Color::Black && to.x==here.x && to.y+1 == here.y && to.piece.is_none() {
            return Some(vec![to]);
        }
        if self.color() == Color::White && to.x==here.x && to.y == here.y+2 && to.piece.is_none()
        && board.square_as_ref_from_coordinates(here.x, here.y+1).ok()?.piece.is_none() {
            return Some(vec![to]);
        }
        if self.color() == Color::Black && to.x==here.x && to.y+2 == here.y && to.piece.is_none()
        && board.square_as_ref_from_coordinates(here.x, to.y+1).ok()?.piece.is_none() {
            return Some(vec![to]);
        }
        None
    }
}

impl Pawn {
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

