use std::fmt::{Debug, Display};
use crate::board::{Board, Square};

pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;

pub trait ChessPiece: Display+Debug{
    fn is_pawn(&self) -> bool {
        false
    }
    fn color(&self) -> Color;
    fn path_eat<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>>;
    fn path_move<'bo>(&self, from_sq: String, to_sq: String, board: &'bo Board) -> Option<Vec<&'bo Square>> {
        self.path_eat(from_sq, to_sq, board)
    }
    fn is_king(&self) -> bool {
        false
    }
    fn is_rook(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

