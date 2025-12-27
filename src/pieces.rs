use std::fmt::{Debug, Display};

pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;

pub trait ChessPiece: Display+Debug{}

#[derive(Debug, Clone)]
pub enum Color {
    White,
    Black
}

