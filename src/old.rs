
enum Color {
    White,
    Black
}

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

struct Piece {
    piece_type: PieceType,
    color: Color
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type: piece_type,
            color: color
        }
    }
}

enum SquareType {

}


fn main() {
    println!("Hello, world!");
}
