#[derive(Copy, Clone)]
enum Piece {
    Pawn(Colour), 
    Knight(Colour),
    Bishop(Colour),
    Rook(Colour),
    Queen(Colour),
    King(Colour),
    Empty,
}

#[derive(Copy, Clone)]
enum Colour {
    Black, 
    White,
}

fn main() {
    board_setup();
    println!("Hello, world!");
}

fn board_setup() -> [[Piece; 8]; 8] {
    let mut board = [[Piece::Empty; 8]; 8]; 
    board
}
