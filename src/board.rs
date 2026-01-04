use std::fmt::Display;
mod pieces;
use pieces::*;
use std::sync::Arc;
mod castling;
mod square;
use square::Square;

#[derive(Debug, Clone)]
pub struct Board {
    squares: Vec<Square>,
    current_color: Color,
    half_moves: usize,
    full_moves: usize,
    pub white_short_allowed: bool,
    pub white_long_allowed: bool,
    pub black_short_allowed: bool,
    pub black_long_allowed: bool,
}

#[derive(Debug)]
pub enum BoardError {
    OutOfBoardSquare
}
fn fen_names() -> Vec<String> {
    let mut names = vec![];
    for x in 0..8 { for y in 0..8 {
        let name = format!("{}{}", "abcdefgh".chars().nth(y).unwrap_or(' '), "12345678".chars().nth(7-x).unwrap_or(' '));
        names.push(name);
    }}
    names
}

impl Board {
    fn new() -> Self {
        let mut squares = vec![];
        for x in 0..8 { for y in 0..8 {
            let name = format!("{}{}", "abcdefgh".chars().nth(x).unwrap_or(' '), "12345678".chars().nth(y).unwrap_or(' '));
            squares.push(Square{ piece: None, pawn_trace: false, x, y, name });
        }}
        //squares.sort_by(|a, b|b.name.as_str().chars().rev().cmp(a.name.as_str().chars().rev()));
        Self { squares,
            current_color: Color::White,
            half_moves: 0,
            full_moves: 1,
            white_short_allowed: true,
            white_long_allowed: true,
            black_short_allowed: true,
            black_long_allowed: true,
        }
    }
    pub fn to_fen(&self) -> String {
        let mut res = String::new();
        let mut empty_count = 0;
        let names = fen_names();
        let mut pawn_trace = "-";
        for (i, sq) in names.iter().enumerate() {
            let square=self.square_as_ref(sq).unwrap();
            match square.piece {
                Some(ref piece) => {
                    if empty_count!=0{
                        res.push_str(empty_count.to_string().as_str());
                    }
                    empty_count=0;
                    res.push_str(piece.to_string().as_str());
                },
                None => {
                    if square.pawn_trace {
                        pawn_trace=sq.as_str();
                    }
                    empty_count+=1;
                }
            }
            if i%8==7 && i!=63 {
                if empty_count!=0{
                    res.push_str(empty_count.to_string().as_str());
                }
                empty_count=0;
                res.push_str("/");
            }
        }
        if empty_count!=0{
            res.push_str(empty_count.to_string().as_str());
        }
        match self.current_color {
            Color::White => res.push_str(" w "),
            Color::Black => res.push_str(" b "),
        }
        if self.white_short_allowed { res.push_str("K"); }
        if self.white_long_allowed { res.push_str("Q"); }
        if self.black_short_allowed { res.push_str("k"); }
        if self.black_long_allowed { res.push_str("q"); }
        res.push_str(format!(" {} {} {}", pawn_trace, self.half_moves, self.full_moves).as_str());
        res
    }
    fn switch_player(&mut self) {
        self.current_color=self.current_color.other();
    }

    fn get_piece(&self, square_name: impl AsRef<str>) -> Result<Option<Arc<dyn ChessPiece>>, BoardError> {
        Ok(self.square_as_ref(square_name)?.piece.clone())
    }
    fn put(&mut self, square_name: impl AsRef<str>, piece: Option<Arc<dyn ChessPiece>>) -> Result<(), BoardError> {
        self.square_mut(square_name)?.piece=piece;
        Ok(())
    }

    fn remove_traced_pawns(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> Result<(), BoardError> {
        let from_sq = self.square_as_ref(from_sq_name.as_ref())?;
        let to_sq = self.square_as_ref(to_sq_name.as_ref())?;
        if to_sq.pawn_trace {
            if to_sq.y>from_sq.y { // moving up
                self.square_mut_from_coordinates(to_sq.x, to_sq.y-1)?.piece=None;
            } else if from_sq.y>to_sq.y { //moving down
                self.square_mut_from_coordinates(to_sq.x, to_sq.y+1)?.piece=None;
            }
        }
        Ok(())
    }
    fn remove_pawn_traces(&mut self) {
        for sq in self.squares.iter_mut() {
            sq.pawn_trace=false;
        }
    }
    fn place_pawn_traces(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> Result<(), BoardError> {
        let from_sq = self.square_as_ref(from_sq_name.as_ref())?;
        let to_sq = self.square_as_ref(to_sq_name.as_ref())?;
        if to_sq.y>from_sq.y+1 { // moving up fast
            self.square_mut_from_coordinates(to_sq.x, to_sq.y-1)?.pawn_trace=true;
        } else if from_sq.y>to_sq.y+1 { // moving down fast
            self.square_mut_from_coordinates(to_sq.x, to_sq.y+1)?.pawn_trace=true;
        }
        Ok(())
    }

    fn controls(&self, color: Color, square: impl AsRef<str>) -> bool {
        for sq in self.squares.iter() {
            if sq.piece.clone().is_some_and(|p|
                p.color()==color && p.path_eat(sq.name.to_string(), square.as_ref().to_string(), self).is_some())
                { return true }
        }
        false
    }
    fn end_turn(&mut self, is_pawn: bool, occupied: bool, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> Result<(), BoardError> {
        if is_pawn { // pawn trace for en passant
            self.remove_traced_pawns(from_sq_name.as_ref(), to_sq_name.as_ref())?;
            self.remove_pawn_traces();
            self.place_pawn_traces(from_sq_name.as_ref(), to_sq_name.as_ref())?;
        } else {
            self.remove_pawn_traces();
        }
        self.half_moves+=1;
        if is_pawn || occupied {
            self.half_moves = 0;
        }
        if self.current_color==Color::Black {
            self.full_moves+=1;
        }
        self.switch_player();
        Ok(())
    }
    fn is_checked(&self, color: Color) -> bool {
        for square in self.squares.iter() {
            if square.piece.clone().is_some_and(|p|p.is_king()&&p.color()==color) {
                return self.controls(color.other(), square.name.clone());
            }
        }
        false
    }
    pub fn is_legal(&self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> bool {
        let mut cloned = self.clone();
        if let Ok(res) = cloned.move_piece(from_sq_name, to_sq_name) {
            return res && !cloned.is_checked(self.current_color.clone());
        }
        false
    }
    pub fn make_move(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> bool {
        if self.is_legal(from_sq_name.as_ref(), to_sq_name.as_ref()) {
            if let Ok(res) = self.move_piece(from_sq_name, to_sq_name) {
                return res;
            }
        }
        false
    }
    fn move_piece(&mut self, from_sq_name: impl AsRef<str>, to_sq_name: impl AsRef<str>) -> Result<bool, BoardError> {
        let piece = match self.get_piece(from_sq_name.as_ref())? {
            Some(p) => p,
            None => return Ok(false) // can move from square only if there is a piece on it
        };
        if piece.color() != self.current_color {
            // cant move opponent piece
            return Ok(false);
        }
        if self.get_piece(to_sq_name.as_ref())?.is_some_and(|p|p.color()==self.current_color) {
            // cant take your own piece
            return Ok(false);
        }
        if self.try_castle(to_sq_name.as_ref(), piece.clone())? {
            self.end_turn(false, false, from_sq_name, to_sq_name)?; // if castling works end turn
            return Ok(true);
        }
        let occupied = self.get_piece(to_sq_name.as_ref())?.is_some();
        if !occupied &&
        piece.clone().path_move(from_sq_name.as_ref().to_string(), to_sq_name.as_ref().to_string(), self).is_none() {
            return Ok(false); // no moving path
        }
        if ( occupied ||
        (self.square_as_ref(to_sq_name.as_ref())?.pawn_trace && piece.clone().is_pawn())) &&
        piece.clone().path_eat(from_sq_name.as_ref().to_string(), to_sq_name.as_ref().to_string(), self).is_none() {
            return Ok(false); // no eating path
        }
        
        self.put(to_sq_name.as_ref(), Some(piece.clone()))?;
        self.put(from_sq_name.as_ref(), None)?;
        self.remove_castling_rights(from_sq_name.as_ref(), to_sq_name.as_ref());
        self.end_turn(piece.is_pawn(), occupied, from_sq_name, to_sq_name)?;
        Ok(true)
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
        let chars = "abcdefgh";
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

