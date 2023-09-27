use console::style;

const GREY15: u8 = 235;
const GREY39: u8 = 241;

const WHITE_PAWN_ROW: usize = 6;
const WHITE_NON_PAWN_ROW: usize = 7;

const BLACK_PAWN_ROW: usize = 1;
const BLACK_NON_PAWN_ROW: usize = 0;

const CHESSBOARD_SIDE_LENGTH: usize = 8;

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

impl Piece {
    fn format(&self) -> String {
        match self {
            Piece::Pawn(Colour::White) => style("p").color256(GREY39).on_white().to_string(),
            Piece::Rook(Colour::White) => style("R").color256(GREY39).on_white().to_string(),
            Piece::Knight(Colour::White) => style("N").color256(GREY39).on_white().to_string(),
            Piece::Bishop(Colour::White) => style("B").color256(GREY39).on_white().to_string(),
            Piece::Queen(Colour::White) => style("Q").color256(GREY39).on_white().to_string(),
            Piece::King(Colour::White) => style("K").color256(GREY39).on_white().to_string(),

            Piece::Pawn(Colour::Black) => style("p").color256(GREY39).on_color256(GREY15).to_string(),
            Piece::Rook(Colour::Black) => style("R").color256(GREY39).on_color256(GREY15).to_string(),
            Piece::Knight(Colour::Black) => style("N").color256(GREY39).on_color256(GREY15).to_string(),
            Piece::Bishop(Colour::Black) => style("B").color256(GREY39).on_color256(GREY15).to_string(),
            Piece::Queen(Colour::Black) => style("Q").color256(GREY39).on_color256(GREY15).to_string(),
            Piece::King(Colour::Black) => style("K").color256(GREY39).on_color256(GREY15).to_string(),

            Piece::Empty => "·".to_string(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Colour {
    Black, 
    White,
}

fn main() {
    print_board(new_game_board());
}

fn new_game_board() -> [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH] {
    let mut board = [[Piece::Empty; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH]; // create new 2D array of Piece enums

    for i in 0..CHESSBOARD_SIDE_LENGTH {
        board[1][i] = Piece::Pawn(Colour::Black);
        board[6][i] = Piece::Pawn(Colour::White);
    }

    for colour in [Colour::White, Colour::Black] {
        for piece in [Piece::Rook(colour), Piece::Knight(colour), Piece::Bishop(colour), 
        Piece::Queen(colour),  Piece::King(colour)] {
            let mut row = 0;
            if colour == Colour::White {
                row = WHITE_NON_PAWN_ROW;
            } else {
                row = BLACK_NON_PAWN_ROW;
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

fn print_board(board: [[Piece; 8]; 8]) {
    for row in 0..8 {
        for column in 0..8 {
            print!("{} ", board[row][column].format());
        }
        println!();
    }
}