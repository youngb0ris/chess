use console::style;
use core::fmt;
use std::io::*;
use std::cmp::*;
use std::fmt::*;

// Color256 colour codes
const GREY_15: u8 = 235;
const GREY_39: u8 = 241;
const DARK_RED: u8 = 52;
const OLIVE: u8 = 94;

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
            Piece::Pawn(Colour::White) => style("p").color256(GREY_39).on_white().to_string(),
            Piece::Rook(Colour::White) => style("R").color256(GREY_39).on_white().to_string(),
            Piece::Knight(Colour::White) => style("N").color256(GREY_39).on_white().to_string(),
            Piece::Bishop(Colour::White) => style("B").color256(GREY_39).on_white().to_string(),
            Piece::Queen(Colour::White) => style("Q").color256(GREY_39).on_white().to_string(),
            Piece::King(Colour::White) => style("K").color256(GREY_39).on_white().to_string(),

            Piece::Pawn(Colour::Black) => style("p").color256(GREY_39).on_color256(GREY_15).to_string(),
            Piece::Rook(Colour::Black) => style("R").color256(GREY_39).on_color256(GREY_15).to_string(),
            Piece::Knight(Colour::Black) => style("N").color256(GREY_39).on_color256(GREY_15).to_string(),
            Piece::Bishop(Colour::Black) => style("B").color256(GREY_39).on_color256(GREY_15).to_string(),
            Piece::Queen(Colour::Black) => style("Q").color256(GREY_39).on_color256(GREY_15).to_string(),
            Piece::King(Colour::Black) => style("K").color256(GREY_39).on_color256(GREY_15).to_string(),

            Piece::Empty => "·".to_string(),
        }
    }

    fn colour(&self) -> Colour {
        match self {
            Piece::Pawn(Colour::White)   |
            Piece::Rook(Colour::White)   |
            Piece::Knight(Colour::White) |
            Piece::Bishop(Colour::White) |
            Piece::Queen(Colour::White)  |
            Piece::King(Colour::White) => Colour::White,

            Piece::Pawn(Colour::Black)   |
            Piece::Rook(Colour::Black)   |
            Piece::Knight(Colour::Black) |
            Piece::Bishop(Colour::Black) |
            Piece::Queen(Colour::Black)  |
            Piece::King(Colour::Black) => Colour::Black,

            Piece::Empty => panic!("Piece::Empty has no colour!"),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::Pawn(_) => write!(f, "p"),
            Piece::Rook(_) => write!(f, "R"),
            Piece::Knight(_) => write!(f, "N"),
            Piece::Bishop(_) => write!(f, "B"),
            Piece::Queen(_) => write!(f, "Q"),
            Piece::King(_) => write!(f, "K"),
            Piece::Empty => write!(f, "·"),
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


// TODO highlight previous moved piece src and destination
// TODO highlight legal moves if only a src square is inputted
fn main() {
    let mut chessboard = new_chessboard();

    
    print_chessboard(chessboard);
    
    chessboard = move_piece(chessboard, Square {row: 0, col: 3}, Square { row: 4, col: 2 }); // rook
    chessboard = move_piece(chessboard, Square {row: 0, col: 1}, Square { row: 4, col: 4 }); // knight

    chessboard = move_piece(chessboard, Square {row: 0, col: 4}, Square { row: 6, col: 0 }); 

    print_chessboard(chessboard);

    // dbg!(get_legal_squares(chessboard, Square { row: 4, col: 2 }));

    highlight_legal_squares(chessboard, Square { row: 6, col: 0 });

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
    println!("┌─────────────────┐");

    for row in 0..CHESSBOARD_SIDE_LENGTH {
        print!("│ ");

        for column in 0..CHESSBOARD_SIDE_LENGTH {
            print!("{} ", board[row][column].format());
        }

        print!("│ {}", style(8 - row).color256(GREY_39)); // prints the explicit chess notation rank number
        println!();
    }
    println!("└─────────────────┘");
    println!("{}", style("  a b c d e f g h").color256(GREY_39));
    println!();
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

fn get_legal_squares(board: [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH], src_square: Square) -> Vec<Square> {
    let mut legal_squares: Vec<Square> = vec![];
    let piece = board[src_square.row][src_square.col];

    match piece {
        Piece::Pawn(_) => {
            if piece.colour() == Colour::White {
                if src_square.row >= 1 && board[src_square.row - 1][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - 1, col: src_square.col });
                }
                if src_square.row == WHITE_PAWN_ROW && board[src_square.row - 2][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - 2, col: src_square.col });
                }
                if src_square.col >= 1 && board[src_square.row - 1][src_square.col - 1] != Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - 1, col: src_square.col - 1});
                } 
                if src_square.col <= 6 && board[src_square.row - 1][src_square.col + 1] != Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - 1, col: src_square.col + 1});
                }
            } else { // if piece is black
                if src_square.row <= 6 && board[src_square.row + 1][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + 1, col: src_square.col });
                }
                if src_square.row == BLACK_PAWN_ROW && board[src_square.row + 2][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + 2, col: src_square.col });
                }
                if src_square.col >= 1 && board[src_square.row + 1][src_square.col - 1] != Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + 1, col: src_square.col - 1});
                } 
                if src_square.col <= 6 && board[src_square.row + 1][src_square.col + 1] != Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + 1, col: src_square.col + 1});
                }
            }
            legal_squares
        },
        Piece::Rook(_) => {
            // check E squares
            let mut i = 1;
            while src_square.col + i < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row][src_square.col + i] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col + i });
                    i += 1;
                } else if board[src_square.row][src_square.col + i].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col + i });
                    break;
                } else {
                    break;
                }
            }

            // check W squares
            let mut j = 1;
            while j <= src_square.col { 
                if board[src_square.row][src_square.col - j] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col - j });
                    j += 1;
                } else if board[src_square.row][src_square.col - j].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col - j });
                    break;
                } else {
                    break;
                }
            }

            // check N squares
            let mut k = 1;
            while src_square.row + k < CHESSBOARD_SIDE_LENGTH {
                if board[src_square.row + k][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + k, col: src_square.col});
                    k += 1;
                } else if board[src_square.row + k][src_square.col].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + k, col: src_square.col });
                    break;
                } else {
                    break;
                }
            }

            // check S squares
            let mut l = 1;
            while l <= src_square.row { 
                if board[src_square.row - l][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - l, col: src_square.col});
                    l += 1;
                } else if board[src_square.row- l][src_square.col].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - l, col: src_square.col });
                    break;
                } else {
                    break;
                }
            }

            legal_squares
        },
        Piece::Knight(_) => {
            // check NNW square
            if src_square.row >= 2 && src_square.col >= 1 && board[src_square.row - 2][src_square.col - 1] == Piece::Empty || 
                    board[src_square.row - 2][src_square.col - 1].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row - 2, col: src_square.col - 1 })
            }

            // check NNE square
            if src_square.row >= 2 && src_square.col + 1 < CHESSBOARD_SIDE_LENGTH && board[src_square.row - 2][src_square.col + 1] == Piece::Empty || 
                    board[src_square.row - 2][src_square.col + 1].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row - 2, col: src_square.col + 1 })
            }

            // check ENE square
            if src_square.row >= 1 && src_square.col + 2 < CHESSBOARD_SIDE_LENGTH && board[src_square.row - 1][src_square.col + 2] == Piece::Empty ||
                    board[src_square.row - 1][src_square.col + 2].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row - 1, col: src_square.col + 2 })
            }

            // check ESE square
            if src_square.row + 1 < CHESSBOARD_SIDE_LENGTH && src_square.col + 2 < CHESSBOARD_SIDE_LENGTH && board[src_square.row + 1][src_square.col + 2] == Piece::Empty ||
                    board[src_square.row + 1][src_square.col + 2].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row + 1, col: src_square.col + 2 })
            }

            // check SSE square
            if src_square.row + 2 < CHESSBOARD_SIDE_LENGTH && src_square.col + 1 < CHESSBOARD_SIDE_LENGTH && board[src_square.row + 2][src_square.col + 1] == Piece::Empty || 
                    board[src_square.row + 2][src_square.col + 1].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row + 2, col: src_square.col + 1 })
            }

            // check SSW square
            if src_square.row + 2 < CHESSBOARD_SIDE_LENGTH && src_square.col >= 1 && board[src_square.row + 2][src_square.col - 1] == Piece::Empty || 
                    board[src_square.row + 2][src_square.col - 1].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row + 2, col: src_square.col - 1 })
            }

            // check WSW square
            if src_square.row + 1 < CHESSBOARD_SIDE_LENGTH && src_square.col >= 2 && board[src_square.row + 1][src_square.col - 2] == Piece::Empty || 
                    board[src_square.row + 1][src_square.col - 2].colour() != piece.colour() {       
                legal_squares.push(Square { row: src_square.row + 1, col: src_square.col - 2 })
            }

            // check WNW square
            if src_square.row >= 1 && src_square.col >= 2 && board[src_square.row - 1][src_square.col - 2] == Piece::Empty ||
                    board[src_square.row - 1][src_square.col - 2].colour() != piece.colour() {
                legal_squares.push(Square { row: src_square.row - 1, col: src_square.col - 2 })
            }

            legal_squares
        },
        Piece::Bishop(_) => {
            // check NE squares
            let mut i = 1;
            while i <= src_square.row && src_square.col + i < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row - i][src_square.col + i] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - i, col: src_square.col + i });
                    i += 1;
                } else if board[src_square.row - i][src_square.col + i].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - i, col: src_square.col + i });
                    break;
                } else {
                    break;
                }
            }

            // check SW squares
            let mut j = 1;
            while src_square.row + j < CHESSBOARD_SIDE_LENGTH && j <= src_square.col { 
                if board[src_square.row + j][src_square.col - j] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + j, col: src_square.col - j });
                    j += 1;
                } else if board[src_square.row + j][src_square.col - j].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + j, col: src_square.col - j });                    break;
                } else {
                    break;
                }
            }

            // check NW squares
            let mut k = 1;
            while k <= src_square.row && k <= src_square.col { 
                if board[src_square.row - k][src_square.col - k] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - k, col: src_square.col - k });
                    k += 1;
                } else if board[src_square.row - k][src_square.col - k].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - k, col: src_square.col - k });
                    break;
                } else {
                    break;
                }
            }

            // check SE squares
            let mut l = 1;
            while src_square.row + l < CHESSBOARD_SIDE_LENGTH && src_square.col + l < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row + l][src_square.col + l] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + l, col: src_square.col + l });
                    l += 1;
                } else if board[src_square.row + l][src_square.col + l].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + l, col: src_square.col + l });
                    break;
                } else {
                    break;
                }
            }

            legal_squares
        },
        Piece::Queen(_) => {
            // literally just copied bishop and rook code
            // check E squares
            let mut i = 1;
            while src_square.col + i < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row][src_square.col + i] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col + i });
                    i += 1;
                } else if board[src_square.row][src_square.col + i].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col + i });
                    break;
                } else {
                    break;
                }
            }

            // check W squares
            let mut j = 1;
            while j <= src_square.col { 
                if board[src_square.row][src_square.col - j] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col - j });
                    j += 1;
                } else if board[src_square.row][src_square.col - j].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row, col: src_square.col - j });
                    break;
                } else {
                    break;
                }
            }

            // check N squares
            let mut k = 1;
            while src_square.row + k < CHESSBOARD_SIDE_LENGTH {
                if board[src_square.row + k][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + k, col: src_square.col});
                    k += 1;
                } else if board[src_square.row + k][src_square.col].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + k, col: src_square.col });
                    break;
                } else {
                    break;
                }
            }

            // check S squares
            let mut l = 1;
            while l <= src_square.row { 
                if board[src_square.row - l][src_square.col] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - l, col: src_square.col});
                    l += 1;
                } else if board[src_square.row- l][src_square.col].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - l, col: src_square.col });
                    break;
                } else {
                    break;
                }
            }

            // check NE squares
            let mut m = 1;
            while m <= src_square.row && src_square.col + m < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row - m][src_square.col + m] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - m, col: src_square.col + m });
                    m += 1;
                } else if board[src_square.row - m][src_square.col + m].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - m, col: src_square.col + m });
                    break;
                } else {
                    break;
                }
            }

            // check SW squares
            let mut n = 1;
            while src_square.row + n < CHESSBOARD_SIDE_LENGTH && n <= src_square.col { 
                if board[src_square.row + n][src_square.col - n] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + n, col: src_square.col - n });
                    n += 1;
                } else if board[src_square.row + n][src_square.col - n].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + n, col: src_square.col - n });                    break;
                } else {
                    break;
                }
            }

            // check NW squares
            let mut o = 1;
            while o <= src_square.row && o <= src_square.col { 
                if board[src_square.row - o][src_square.col - o] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row - o, col: src_square.col - o });
                    o += 1;
                } else if board[src_square.row - o][src_square.col - o].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row - o, col: src_square.col - o });
                    break;
                } else {
                    break;
                }
            }

            // check SE squares
            let mut p = 1;
            while src_square.row + p < CHESSBOARD_SIDE_LENGTH && src_square.col + p < CHESSBOARD_SIDE_LENGTH { 
                if board[src_square.row + p][src_square.col + p] == Piece::Empty {
                    legal_squares.push(Square { row: src_square.row + p, col: src_square.col + p });
                    p += 1;
                } else if board[src_square.row + p][src_square.col + p].colour() != piece.colour() {
                    legal_squares.push(Square { row: src_square.row + p, col: src_square.col + p });
                    break;
                } else {
                    break;
                }
            }

            legal_squares
        },
        Piece::King(_) => {
            // check N square
            if src_square.row >= 1 && 
                    (board[src_square.row - 1][src_square.col] == Piece::Empty || board[src_square.row - 1][src_square.col].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row - 1, col: src_square.col });
            } 

            // check NE square
            if src_square.row >= 1 && src_square.col <= 6 && 
                    (board[src_square.row - 1][src_square.col + 1] == Piece::Empty || board[src_square.row - 1][src_square.col + 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row - 1, col: src_square.col + 1 });
            } 

            // check E square
            if src_square.col <= 6 && 
                    (board[src_square.row][src_square.col + 1] == Piece::Empty || board[src_square.row][src_square.col + 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row, col: src_square.col + 1 });
            } 

            // check SE square
            if src_square.row <= 6 && src_square.col <= 6 && 
                    (board[src_square.row + 1][src_square.col + 1] == Piece::Empty || board[src_square.row + 1][src_square.col + 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row + 1, col: src_square.col + 1 });
            } 
            
            // check S square
            if src_square.row <= 6 && 
                    (board[src_square.row + 1][src_square.col] == Piece::Empty || board[src_square.row + 1][src_square.col].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row + 1, col: src_square.col});
            } 

            // check SW square
            if src_square.row <= 6 && src_square.col >= 1 && 
                    (board[src_square.row + 1][src_square.col - 1] == Piece::Empty || board[src_square.row + 1][src_square.col - 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row + 1, col: src_square.col - 1});
            } 

            // check W square
            if src_square.col >= 1 && 
                    (board[src_square.row][src_square.col - 1] == Piece::Empty || board[src_square.row][src_square.col - 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row, col: src_square.col - 1});
            }

            // check NW square
            if src_square.col >= 1 && src_square.row >= 1 && 
                    (board[src_square.row - 1][src_square.col - 1] == Piece::Empty || board[src_square.row - 1][src_square.col - 1].colour() != piece.colour()) {
                legal_squares.push(Square { row: src_square.row - 1, col: src_square.col - 1});
            } 

            legal_squares
        },
        Piece::Empty => legal_squares,
    }
}

fn highlight_legal_squares(board: [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH], src_square: Square) { 
    let legal_squares = get_legal_squares(board, src_square);

    println!("┌─────────────────┐");

    for r in 0..CHESSBOARD_SIDE_LENGTH {
        print!("│ ");

        for c in 0..CHESSBOARD_SIDE_LENGTH {
            let sq = Square {row: r, col: c};

            if legal_squares.contains(&sq) {
                if board[r][c] == Piece::Empty {
                    print!("{} ", style("⬤").yellow());
                } else if board[r][c].colour() == Colour::White {
                    print!("{} ", style(board[r][c]).white().on_red());
                } else {
                    print!("{} ", style(board[r][c]).black().on_red());
                }
            } else if sq == src_square {
                if board[r][c].colour() == Colour::White {
                    print!("{} ", style(board[r][c]).color256(OLIVE).on_white());
                } else {
                    print!("{} ", style(board[r][c]).color256(OLIVE).on_color256(GREY_15));
                }
            } else {
                print!("{} ", board[r][c].format());
            }
        }
        print!("│ {}", style(8 - r).color256(GREY_39)); // prints the explicit chess notation rank number
        println!();
    }
    println!("└─────────────────┘");
    println!("{}", style("  a b c d e f g h").color256(GREY_39));
    println!();
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

fn move_piece(mut board: [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH], src: Square, dest: Square) -> [[Piece; CHESSBOARD_SIDE_LENGTH]; CHESSBOARD_SIDE_LENGTH] {
    let src_piece = board[src.row][src.col];

    board[dest.row][dest.col] = src_piece;
    board[src.row][src.col] = Piece::Empty;

    board
}