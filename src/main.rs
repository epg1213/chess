mod board;
use board::Board;

pub fn get_input() -> String {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let _ = stdin.read_line(&mut buffer);
    let mut chars = buffer.chars();
    chars.next_back();
    chars.as_str().to_string()
}

fn main() {
    let mut board = Board::default();
    loop {
        println!("{}", board.to_fen());
        println!("{}", board);
        println!("{:?}", board.make_move(get_input(), get_input()));
    }
}

