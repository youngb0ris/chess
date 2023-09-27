use console::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Piece {
    Pawn(Colour), 
    Rook(Colour),
    Knight(Colour),
    Bishop(Colour),
    Queen(Colour),
    King(Colour),
    Empty,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Colour {
    Black, 
    White,
}

fn main() {
    new_game_board();
    println!("Hello, world!");

    dbg!(new_game_board());
}

fn new_game_board() -> [[Piece; 8]; 8] {
    let mut board = [[Piece::Empty; 8]; 8]; // create new 2D array of Piece enums

    for i in 0..7 {
        board[1][i] = Piece::Pawn(Colour::Black);
        board[6][i] = Piece::Pawn(Colour::White);
    }

    for colour in [Colour::White, Colour::Black] {
        for piece in [Piece::Rook(colour), Piece::Knight(colour), Piece::Bishop(colour), 
        Piece::Queen(colour),  Piece::King(colour)] {
            let mut row = 0;
            if colour == Colour::White {
                row = 7;
            } else {
                row = 0;
            }
            match piece {
                Piece::Rook(_) => {
                    board[row][0] = piece;
                    board[row][7] = piece;
                },
                Piece::Knight(_) => {
                    board[row][1] = piece;
                    board[row][6] = piece;
                },
                Piece::Bishop(_) => {
                    board[row][2] = piece;
                    board[row][5] = piece;
                },
                Piece::Queen(_) => board[row][3] = piece,
                Piece::King(_) => board[row][4] = piece,

                Piece::Pawn(_) => panic!("This should be impossible!"),
                Piece::Empty => panic!("This should be impossible!"), 
            }
        }
    }
    board
}
