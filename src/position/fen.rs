use crate::defs::FEN_NUM_PARTS;
use crate::piece::Piece;
use crate::position::Position;
use crate::position::Squares;
use crate::Sides;
use crate::Board;

use super::CastleRights;

const WHITE_SIDE:Sides = Sides::White;
const BLACK_SIDE:Sides = Sides::Black;

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = fen.split_whitespace().collect();

        if parts.len() != FEN_NUM_PARTS {
            return Err(format!("FEN must contain {FEN_NUM_PARTS} parts, got {}", parts.len()));
        }
        Ok(Position { 
            board: parse_board(parts[0])?,
            colour_to_move: parse_colour_to_move(parts[1])?,
            castling_rights: (),
            en_passant_square: (),
            half_move_clock: parts[4].parse().unwrap(),
            full_move_counter: parts[5].parse().unwrap() 
        })

    }
}

fn parse_board(str: &str) -> Result<Board, String> {

    let row_count = str.matches('/').count() + 1;

    if row_count != 8 {
        return Err(format!("board must contain 8 rows, got {}", row_count));
    }
    let mut board = Board::empty();
    let mut square_index = Squares::a8 as u8;

    for char in str.chars() {
        if char == '/' {
            square_index -= 16;
            continue;
        }

        if char.is_ascii_digit() {
            square_index += char as u8 - b'0';
            continue;
        }

        let piece = match char {
            'P' => Piece::pawn(WHITE_SIDE),
            'N' => Piece::knight(WHITE_SIDE),
            'B' => Piece::bishop(WHITE_SIDE),
            'R' => Piece::rook(WHITE_SIDE),
            'Q' => Piece::queen(WHITE_SIDE),
            'K' => Piece::king(WHITE_SIDE),
            'p' => Piece::pawn(BLACK_SIDE),
            'n' => Piece::knight(BLACK_SIDE),
            'b' => Piece::bishop(BLACK_SIDE),
            'r' => Piece::rook(BLACK_SIDE),
            'q' => Piece::queen(BLACK_SIDE),
            'k' => Piece::king(BLACK_SIDE),
            _ => return Err(format!("invalid piece '{char}'")),
        };

        board.put_piece(piece, Squares::from_u8(square_index));
        square_index += 1;
    }

    if square_index != 8 {
        return Err("board must contain 64 squares".to_string());
    }

    Ok(board)
}

fn parse_colour_to_move(colour: &str) -> Result<Sides, String> {
    match colour {
        "w" => Ok(Sides::White),
        "b" => Ok(Sides::Black),
        _ => Err(format!("invalid colour to move '{colour}'")),
    }
}

fn parse_castling_rights(str: &str) -> Result<CastleRights, String> {
    if str == "-" {
        return Ok(CastleRights::none());
    }

    let mut rights = CastleRights::none();

    for char in str.chars() {
        rights.add(match char {
            'K' => CastleRights::WhiteKing,
            'Q' => CastleRights::WhiteQueen,
            'k' => CastleRights::BlackKing,
            'q' => CastleRights::BlackQueen,
            _ => return Err("invalid castling rights".to_string()),
        });
    }

    Ok(rights)
}


fn parse_en_passant_square(square: &str) -> Result<Option<Square>, String> {
    if square == "-" {
        return Ok(None);
    }

    let result = square.parse::<Square>();

    if result.is_err() {
        return Err("invalid en passant square".to_string());
    }

    let square = result.unwrap();

    if square.rank() != 2 && square.rank() != 5 {
        return Err("invalid en passant square".to_string());
    }

    Ok(Some(square))
}
