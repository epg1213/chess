use std::fmt::Display;

use crate::pieces::{Piece, Color, PieceType};

#[derive(Debug)]
pub struct Square {
    piece: Option<Piece>,
    pawn_trace: bool,
    name: String,
    x: usize,
    y: usize
}

impl Square {
    pub fn new() -> Self {
        Self {
            piece: None, pawn_trace: false,
            name: String::new(), x: 0, y: 0
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.piece {
            Some(p) => write!(f, "{}", p),
            None => write!(f, "-")
        }
    }
}

#[derive(Debug)]
pub struct Board {
    squares: Vec<Square>
}

impl Board {
    pub fn new() -> Self {
        let mut squares = Vec::<Square>::new();
        let chars = "ABCDEFGH";
        let nums = "12345678";
        for x in 0..8 { for y in 0..8 {
            squares.push(Square{
                piece: None,
                pawn_trace: false,
                name: format!("{}{}",
                    chars.chars().nth(x).unwrap_or(' '),
                    nums.chars().nth(y).unwrap_or(' ')),
                x: x,
                y: y
            });
        }}
        Self { squares: squares }
    }

    pub fn square_mut(&mut self, square_name: impl AsRef<str>) -> Option<&mut Square> {
        for sq in self.squares.iter_mut() {
            if sq.name.as_str() == square_name.as_ref() {
                return Some(sq);
            }
        }
        None
    }
    pub fn square_as_ref(&self, square_name: impl AsRef<str>) -> Option<&Square> {
        for sq in self.squares.iter() {
            if sq.name.as_str() == square_name.as_ref() {
                return Some(sq);
            }
        }
        None
    }
    pub fn put(&mut self, square_name: impl AsRef<str>, piece: Piece) {
        match self.square_mut(square_name) {
            Some(sq) => sq.piece=Some(piece),
            None => {}
        }
    }
    pub fn default() -> Self {
        let mut board = Board::new();
        board.put("A2", Piece::new(Color::White, PieceType::Pawn));
        board.put("B2", Piece::new(Color::White, PieceType::Pawn));
        board.put("C2", Piece::new(Color::White, PieceType::Pawn));
        board.put("D2", Piece::new(Color::White, PieceType::Pawn));
        board.put("E2", Piece::new(Color::White, PieceType::Pawn));
        board.put("F2", Piece::new(Color::White, PieceType::Pawn));
        board.put("G2", Piece::new(Color::White, PieceType::Pawn));
        board.put("H2", Piece::new(Color::White, PieceType::Pawn));
        board.put("A7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("B7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("C7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("D7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("E7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("F7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("G7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("H7", Piece::new(Color::Black, PieceType::Pawn));
        board.put("A1", Piece::new(Color::White, PieceType::Rook));
        board.put("H1", Piece::new(Color::White, PieceType::Rook));
        board.put("A8", Piece::new(Color::Black, PieceType::Rook));
        board.put("H8", Piece::new(Color::Black, PieceType::Rook));
        board.put("B1", Piece::new(Color::White, PieceType::Knight));
        board.put("G1", Piece::new(Color::White, PieceType::Knight));
        board.put("B8", Piece::new(Color::Black, PieceType::Knight));
        board.put("G8", Piece::new(Color::Black, PieceType::Knight));
        board.put("C1", Piece::new(Color::White, PieceType::Bishop));
        board.put("F1", Piece::new(Color::White, PieceType::Bishop));
        board.put("C8", Piece::new(Color::Black, PieceType::Bishop));
        board.put("F8", Piece::new(Color::Black, PieceType::Bishop));
        board.put("D1", Piece::new(Color::White, PieceType::Queen));
        board.put("D8", Piece::new(Color::Black, PieceType::Queen));
        board.put("E1", Piece::new(Color::White, PieceType::King));
        board.put("E8", Piece::new(Color::Black, PieceType::King));
        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let chars = "ABCDEFGH";
        let nums = "12345678";
        for y in 0..8 {
            for x in 0..8 {
                s.push_str(format!("{}",
                    self.square_as_ref(format!("{}{}",
                    chars.chars().nth(x).unwrap_or(' '),
                    nums.chars().nth(7-y).unwrap_or(' ')))
                    .unwrap_or(&mut Square::new())).as_str());
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

