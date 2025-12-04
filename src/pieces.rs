use std::fmt::Display;


#[derive(Debug)]
pub enum Color {
    White,
    Black
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug)]
pub struct Piece {
    color: Color,
    piece_type: PieceType
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self {
            color: color,
            piece_type: piece_type
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.piece_type {
            PieceType::Pawn => {
                match self.color {
                    Color::White => write!(f, "P"),
                    Color::Black => write!(f, "p")
                }
            },
            PieceType::Knight => {
                match self.color {
                    Color::White => write!(f, "N"),
                    Color::Black => write!(f, "n")
                }
            },
            PieceType::Bishop => {
                match self.color {
                    Color::White => write!(f, "B"),
                    Color::Black => write!(f, "b")
                }
            },
            PieceType::Rook => {
                match self.color {
                    Color::White => write!(f, "R"),
                    Color::Black => write!(f, "r")
                }
            },
            PieceType::Queen => {
                match self.color {
                    Color::White => write!(f, "Q"),
                    Color::Black => write!(f, "q")
                }
            },
            PieceType::King => {
                match self.color {
                    Color::White => write!(f, "K"),
                    Color::Black => write!(f, "k")
                }
            },
        }
    }
}

