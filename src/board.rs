use std::fmt::Display;
use crate::pieces::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Square {
    piece: Option<Arc<dyn ChessPiece>>,
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

pub enum BoardError {
    OutOfBoardSquare
}

impl Board {
    pub fn new() -> Self {
        let mut squares = vec![];
        for x in 0..8 { for y in 0..8 {
            let name = format!("{}{}", "ABCDEFGH".chars().nth(x).unwrap_or(' '),
                                       "12345678".chars().nth(y).unwrap_or(' '));
            squares.push(Square{ piece: None, pawn_trace: false, x, y, name });
        }}
        Self { squares }
    }

    pub fn square_mut(&mut self, square_name: impl AsRef<str>) -> Result<&mut Square, BoardError> {
        for sq in self.squares.iter_mut() {
            if sq.name.as_str() == square_name.as_ref() {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
    pub fn square_as_ref(&self, square_name: impl AsRef<str>) -> Result<&Square, BoardError> {
        for sq in self.squares.iter() {
            if sq.name.as_str() == square_name.as_ref() {
                return Ok(sq);
            }
        }
        Err(BoardError::OutOfBoardSquare)
    }
    pub fn get_piece(&self, square_name: impl AsRef<str>) -> Result<Option<Arc<dyn ChessPiece>>, BoardError> {
        Ok(self.square_as_ref(square_name)?.piece.clone())
    }
    pub fn put(&mut self, square_name: impl AsRef<str>, piece: Option<Arc<dyn ChessPiece>>) -> Result<(), BoardError> {
        self.square_mut(square_name)?.piece=piece;
        Ok(())
    }
    pub fn move_piece(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> Result<(), BoardError> {
        self.put(to_sq_name, self.get_piece(from_sq_name.as_ref())?)?;
        self.put(from_sq_name, None)?;
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();
        let _ = board.put("A2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("B2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("C2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("D2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("E2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("F2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("G2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("H2", Some(Arc::new(pawn::Pawn::white())));
        let _ = board.put("A7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("B7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("C7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("D7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("E7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("F7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("G7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("H7", Some(Arc::new(pawn::Pawn::black())));
        let _ = board.put("A1", Some(Arc::new(rook::Rook::white())));
        let _ = board.put("H1", Some(Arc::new(rook::Rook::white())));
        let _ = board.put("A8", Some(Arc::new(rook::Rook::black())));
        let _ = board.put("H8", Some(Arc::new(rook::Rook::black())));
        let _ = board.put("B1", Some(Arc::new(knight::Knight::white())));
        let _ = board.put("G1", Some(Arc::new(knight::Knight::white())));
        let _ = board.put("B8", Some(Arc::new(knight::Knight::black())));
        let _ = board.put("G8", Some(Arc::new(knight::Knight::black())));
        let _ = board.put("C1", Some(Arc::new(bishop::Bishop::white())));
        let _ = board.put("F1", Some(Arc::new(bishop::Bishop::white())));
        let _ = board.put("C8", Some(Arc::new(bishop::Bishop::black())));
        let _ = board.put("F8", Some(Arc::new(bishop::Bishop::black())));
        let _ = board.put("D1", Some(Arc::new(queen::Queen::white())));
        let _ = board.put("D8", Some(Arc::new(queen::Queen::black())));
        let _ = board.put("E1", Some(Arc::new(king::King::white())));
        let _ = board.put("E8", Some(Arc::new(king::King::black())));
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

