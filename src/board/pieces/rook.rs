use std::fmt::Display;
use crate::board::pieces::{ChessPiece, Color};
use crate::board::{Board, Square};

#[derive(Debug, Clone, PartialEq)]
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

impl ChessPiece for Rook {
    fn color(&self) -> Color {
        self.color.clone()
    }
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        if from_sq == to_sq {
            return None;
        }
        let here = board.square_as_ref(from_sq).ok()?;
        let to = board.square_as_ref(&to_sq).ok()?;
        let mut squares = vec![];
        if here.x == to.x {
            let mut step: i8 = 1;
            if to.y<here.y { step = -1; }
            let mut y: i8 = here.y.try_into().ok()?;
            while !(y == to.y.try_into().ok()?) {
                y+=step;
                let square = board.square_as_ref_from_coordinates(here.x.try_into().ok()?, y.try_into().ok()?).ok()?;
                if square.piece.is_some() && square.name != to_sq {
                    return None;
                }
                squares.push(square);
            }
            return Some(squares);
        }
        if here.y == to.y {
            let mut step: i8 = 1;
            if to.x<here.x { step = -1; }
            let mut x: i8 = here.x.try_into().ok()?;
            while !(x == to.x.try_into().ok()?) {
                x+=step;
                let square = board.square_as_ref_from_coordinates(x.try_into().ok()?, here.y.try_into().ok()?).ok()?;
                if square.piece.is_some() && square.name != to_sq {
                    return None;
                }
                squares.push(square);
            }
            return Some(squares);
        }
        None
    }
    fn is_rook(&self) -> bool {
        true
    }
}

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

