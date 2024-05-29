use std::{fmt::{Display, self}, ops::RangeInclusive};

use super::board::{Bitboard, Squares};

pub struct About;
impl About {
    pub const ENGINE: &'static str = "Rustic Alpha";
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const AUTHOR: &'static str = "Marcel Vanthoor";
    pub const EMAIL: &'static str = "mail@marcelvanthoor.nl";
    pub const WEBSITE: &'static str = "https://rustic-chess.org/";
}

pub type Piece = usize;
pub type Side = usize;
pub type Square = usize;

#[derive(Copy, Clone, PartialEq)]
pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const FEN_KIWIPETE_POSITION: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

pub struct NrOf;
impl NrOf {
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_PERMISSIONS: usize = 16; // 0-15
    pub const SQUARES: usize = 64;
    pub const FILES: usize = 8;
    pub const RANKS: usize = 8;
}

pub struct Castling;
impl Castling {
    pub const WK: u8 = 1;
    pub const WQ: u8 = 2;
    pub const BK: u8 = 4;
    pub const BQ: u8 = 8;
    pub const ALL: u8 = 15;
}

pub const EMPTY: u64 = 0;
pub const MAX_GAME_MOVES: usize = 2048;
pub const MAX_LEGAL_MOVES: u8 = 255;
pub const MAX_PLY: i8 = 125;
pub const MAX_MOVE_RULE: u8 = 100; // 50/75 move rule

// Define errors
pub type EngineRunResult = Result<(), u8>;
pub const ENGINE_RUN_ERRORS: [&str; 8] = [
    "FEN: Must have six parts",
    "FEN: Pieces and squares incorrect",
    "FEN: Color selection incorrect",
    "FEN: Castling permissions incorrect",
    "FEN: En-passant square incorrect",
    "FEN: Half-move clock incorrect",
    "FEN: Full-move number incorrect",
    "XBoard not yet implemented.",
];


const NR_OF_FEN_PARTS: usize = 6;
const SHORT_FEN_PARTS: usize = 4;
const LIST_OF_PIECES: &str = "kqrbnpKQRBNP";
const EP_SQUARES_WHITE: RangeInclusive<Squares> = Squares::a3..=Squares::h3;
const EP_SQUARES_BLACK: RangeInclusive<Squares> = Squares::a6..=Squares::h6;
const WHITE_OR_BLACK: &str = "wb";
const SPLITTER: char = '/';
const DASH: char = '-';
const EM_DASH: char = '–';
const SPACE: char = ' ';

#[derive(Debug)]
pub enum FenError {
    IncorrectLength,
    Part1,
    Part2,
    Part3,
    Part4,
    Part5,
    Part6,
}

impl Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::IncorrectLength => "Error in FEN string: Must be 6 parts",
            Self::Part1 => "Error in FEN Part 1: Pieces or squares",
            Self::Part2 => "Error in FEN Part 2: Colors",
            Self::Part3 => "Error in FEN Part 3: Castling rights",
            Self::Part4 => "Error in FEN Part 4: En passant field",
            Self::Part5 => "Error in FEN Part 5: Half-move clock",
            Self::Part6 => "Error in FEN Part 6: Full-move number",
        };
        write!(f, "{error}")
    }
}
type FenPartParser = fn(board: &mut Board, part: &str) -> bool;
type FenResult = Result<(), u8>;

impl Board{
    // This function reads a provided FEN-string or uses the default position.
    pub fn fen_read(&mut self, fen_string: Option<&str>) -> FenResult {
        // Split the string into parts. There should be 6 parts.
        let mut fen_parts: Vec<String> = match fen_string {
            Some(f) => f,
            None => FEN_START_POSITION,
        }
        .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
        .split(SPACE)
        .map(|s| s.to_string())
        .collect();

        if fen_parts.len() == SHORT_FEN_PARTS {
            fen_parts.append(&mut vec![String::from("0"), String::from("1")]);
        }

        // Check the number of fen parts.
        let nr_of_parts_ok = fen_parts.len() == NR_OF_FEN_PARTS;

        // Set the initial result.
        let mut result: FenResult = if nr_of_parts_ok { Ok(()) } else { Err(0) };

        if nr_of_parts_ok {
            // Create an array of function pointers; one parsing function per part.
            let fen_parsers: [FenPartParser; 6] = [pieces, color, castling, ep, hmc, fmn];

            // Create a new board so we don't destroy the original.
            let mut new_board = self.clone();
            new_board.reset();

            // Parse all the parts and check if each one succeeds.
            let mut i: usize = 0;
            while i < NR_OF_FEN_PARTS && result == Ok(()) {
                let parser = &fen_parsers[i];
                let part = &fen_parts[i];
                let part_ok = parser(&mut new_board, part);
                result = if part_ok { Ok(()) } else { Err(i as u8 + 1) };
                i += 1;
            }

            // Replace original board with new one if setup was successful.
            if result == Ok(()) {
                new_board.init();
                *self = new_board;
            }
        }

        result
    }
}

// ===== Private functions =====

// Part 1: Parsing piece setup. Put each piece into its respective bitboard.
fn pieces(board: &mut Board, part: &str) -> bool {
    let mut rank = Ranks::R8 as u8;
    let mut file = Files::A as u8;

    // Assume parsing succeeds.
    let mut result = true;

    // Parse each character; it should be a piece, square count, or splitter.
    for c in part.chars() {
        let square = ((rank * 8) + file) as usize;
        match c {
            'k' => board.bb_pieces[Sides::BLACK][Pieces::KING] |= BB_SQUARES[square],
            'q' => board.bb_pieces[Sides::BLACK][Pieces::QUEEN] |= BB_SQUARES[square],
            'r' => board.bb_pieces[Sides::BLACK][Pieces::ROOK] |= BB_SQUARES[square],
            'b' => board.bb_pieces[Sides::BLACK][Pieces::BISHOP] |= BB_SQUARES[square],
            'n' => board.bb_pieces[Sides::BLACK][Pieces::KNIGHT] |= BB_SQUARES[square],
            'p' => board.bb_pieces[Sides::BLACK][Pieces::PAWN] |= BB_SQUARES[square],
            'K' => board.bb_pieces[Sides::WHITE][Pieces::KING] |= BB_SQUARES[square],
            'Q' => board.bb_pieces[Sides::WHITE][Pieces::QUEEN] |= BB_SQUARES[square],
            'R' => board.bb_pieces[Sides::WHITE][Pieces::ROOK] |= BB_SQUARES[square],
            'B' => board.bb_pieces[Sides::WHITE][Pieces::BISHOP] |= BB_SQUARES[square],
            'N' => board.bb_pieces[Sides::WHITE][Pieces::KNIGHT] |= BB_SQUARES[square],
            'P' => board.bb_pieces[Sides::WHITE][Pieces::PAWN] |= BB_SQUARES[square],
            '1'..='8' => {
                if let Some(x) = c.to_digit(10) {
                    file += x as u8;
                }
            }
            SPLITTER => {
                result = file == 8;
                rank -= 1;
                file = 0;
            }
            // Unknown character: result becomes false.
            _ => result = false,
        }

        // If piece found, advance to the next file.
        if LIST_OF_PIECES.contains(c) {
            file += 1;
        }

        // As soon as something is wrong, stop parsing.
        if !result {
            break;
        }
    }

    result
}

// Part 2: Parse color to move: White or Black
fn color(board: &mut Board, part: &str) -> bool {
    // Assume parsing fails.
    let mut result = false;

    // Length should be 1, and the character should be 'w' or 'b'.
    if_chain! {
        if part.len() == 1;
        if let Some(x) = part.chars().next();
        if WHITE_OR_BLACK.contains(x);
        then {
            match x {
                'w' => board.game_state.active_color = Sides::WHITE as u8,
                'b' => board.game_state.active_color = Sides::BLACK as u8,
                _ => (),
            }

            // If everything is correct, set the result to true;
            result = true;
        }
    }

    result
}

// Part 3: Parse castling rights.
fn castling(board: &mut Board, part: &str) -> bool {
    let length = part.len();
    let mut char_ok = 0;

    // There should be 1 to 4 castling rights. If no player has castling
    // rights, the character is '-'.
    if (1..=4).contains(&length) {
        // Accepts "-" for no castling rights in addition to leaving out letters.
        for c in part.chars() {
            if CASTLING_RIGHTS.contains(c) {
                // Count correct characters
                char_ok += 1;
                match c {
                    'K' => board.game_state.castling |= Castling::WK,
                    'Q' => board.game_state.castling |= Castling::WQ,
                    'k' => board.game_state.castling |= Castling::BK,
                    'q' => board.game_state.castling |= Castling::BQ,
                    _ => (),
                }
            }
        }
    }

    // Counted correct characters should be at least 1, and equal to the
    // length of the part.
    (length >= 1) && (char_ok == length)
}

// Part 4: Parse the en passant square
fn ep(board: &mut Board, part: &str) -> bool {
    let length = part.len();
    let mut char_ok = 0;

    // No en-passant square if length is 1. The character should be a DASH.
    if_chain! {
        if length == 1;
        if let Some(x) = part.chars().next();
        if x == DASH;
        then {
            char_ok += 1
        }
    }

    // If length is 2, try to parse the part to a square number.
    if length == 2 {
        let square = parse::algebraic_square_to_number(part);

        match square {
            Some(s) if EP_SQUARES_WHITE.contains(&s) || EP_SQUARES_BLACK.contains(&s) => {
                board.game_state.en_passant = Some(s as u8);
                char_ok += 2;
            }
            Some(_) | None => (),
        }
    }

    // The length of this part should either be 1 or 2, and the counted
    // correct characters should be equal to the part length.
    (length == 1 || length == 2) && (length == char_ok)
}

// Part 5: Half-move clock: parse number of moves since last capture or pawn push.
fn hmc(board: &mut Board, part: &str) -> bool {
    let length = part.len();
    let mut result = false;

    if_chain! {
        if length == 1 || length == 2;
        if let Ok(x) = part.parse::<u8>();
        if x <= MAX_MOVE_RULE;
        then {
            board.game_state.halfmove_clock = x;
            result = true;
        }
    }

    result
}

// Part 6: Parse full move number.
fn fmn(board: &mut Board, part: &str) -> bool {
    let length = part.len();
    let mut result = false;

    if_chain! {
        if length >= 1 || length <= 4;
        if let Ok(x) = part.parse::<u16>();
        if x <= (MAX_GAME_MOVES as u16);
        then {
            board.game_state.fullmove_number = x;
            result = true;
        }
    }

    result
}


#[cfg(test)]
mod fen_tests{
    use super::*;

    #[test]
    fn all_empty() {
        // Represents the empty board with no pieces
        let fen = "8/8/8/8/8/8/8/8 w - - ";
    }
        
    #[test]
    fn start() {
        // Represents the Starting board position
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
    }

    #[test]
    fn tricky() {
        //Represents some random mid game position
        let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
    }

    #[test]
    fn capture() {
        //Again a mid game capturing position
        let fen = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
    }

    #[test]
    fn random() {
        let fen = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9";
    }
}
