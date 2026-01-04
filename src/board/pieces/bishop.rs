use std::fmt::Display;
use crate::board::pieces::{ChessPiece, Color};
use crate::board::{Board, Square};

#[derive(Debug, Clone, PartialEq)]
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

impl ChessPiece for Bishop {
    fn color(&self) -> Color {
        self.color.clone()
    }
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        if from_sq == to_sq {
            return None;
        }
        let here = board.square_as_ref(from_sq).ok()?;
        let to = board.square_as_ref(&to_sq).ok()?;
        if here.x.abs_diff(to.x) != here.y.abs_diff(to.y) {
            return None // not diag
        }
        let mut step_x: i8 = 1;
        if to.x<here.x { step_x = -1; }
        let mut step_y: i8 = 1;
        if to.y<here.y { step_y = -1; }
        let mut x: i8 = here.x.try_into().ok()?;
        let mut y: i8 = here.y.try_into().ok()?;
        let mut squares = vec![];
        while !(x == to.x.try_into().ok()? && y == to.y.try_into().ok()?) {
            x+=step_x;
            y+=step_y;
            let square = board.square_as_ref_from_coordinates(x.try_into().ok()?, y.try_into().ok()?).ok()?;
            if square.piece.is_some() && square.name != to_sq {
                return None;
            }
            squares.push(square);
        }
        Some(squares)
    }
}

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

