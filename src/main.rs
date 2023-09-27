use console::style;
use std::io::*;
use std::cmp::*;

// Color256 colur codes
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

#[derive(Copy, Clone, PartialEq, Debug)]
struct Square {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Input {
    src: Square,
    dest: Square,
}

fn main() {
    print_chessboard(new_chessboard());

    dbg!(parse_input());
}

fn new_chessboard() -> [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH] {
    let mut board = [[Piece::Empty; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH]; // create new 2D array of Piece enums

    for i in 0..CHESSBOARD_SIDE_LENGTH {
        board[WHITE_PAWN_ROW][i] = Piece::Pawn(Colour::White);
        board[BLACK_PAWN_ROW][i] = Piece::Pawn(Colour::Black);
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

fn print_chessboard(board: [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH]) {
    for row in 0..CHESSBOARD_SIDE_LENGTH {
        for column in 0..CHESSBOARD_SIDE_LENGTH {
            print!("{} ", board[row][column].format());
        }
        println!();
    }
}

fn parse_input() -> Input {
    let mut user_input = String::new();
    std::io::stdin()
            .read_line(&mut user_input)
            .expect("Error!");

    let mut user_input_as_vector: Vec<char> = user_input.trim().to_lowercase().chars().collect(); 

    if user_input_as_vector.len() == 5 {
        user_input_as_vector.remove(2); // removes the middle character in user_input_as_vector (eg. if it were a ' ' or a 'x')
    } else if user_input_as_vector.len() >= 6 {
        todo!("User input too long!");
    }

    Input {
        src: string_to_square(format!("{}{}", user_input_as_vector[0], user_input_as_vector[1])),
        dest: string_to_square(format!("{}{}", user_input_as_vector[2], user_input_as_vector[3])),
    }
}

fn get_legal_squares(board: [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH], square: Square) -> Vec<Square> {
    let legal_squares: Vec<Square> = vec![];

    let piece = board[square.row][square.col];

    legal_squares
}

fn string_to_square(chess_notation: String) -> Square { // eg. converts "c3" to (5, 2)
    let mut square = Square {row: 0, col: 0};
    
    let input: Vec<char> = chess_notation.trim().to_lowercase().chars().collect();

    match input[0] {
        'a' => square.col = 0, 
        'b' => square.col = 1, 
        'c' => square.col = 2, 
        'd' => square.col = 3, 
        'e' => square.col = 4, 
        'f' => square.col = 5, 
        'g' => square.col = 6, 
        'h' => square.col = 7, 
        _   => todo!("Handle invalid input!"),
    }

    match input[1] {
        '8' => square.row = 0, 
        '7' => square.row = 1, 
        '6' => square.row = 2, 
        '5' => square.row = 3, 
        '4' => square.row = 4, 
        '3' => square.row = 5, 
        '2' => square.row = 6, 
        '1' => square.row = 7, 
        _   => todo!("Handle invalid input!"),
    }
    square
}