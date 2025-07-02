use crate::defs::FEN_NUM_PARTS;
use crate::piece::Piece;
use crate::position::{Bitboard, Position};
use crate::position::Squares;
use crate::{set_bit, Board};
use crate::Sides;

use super::castling::Castle;
use super::CastleRights;

const WHITE_SIDE: Sides = Sides::White;
const BLACK_SIDE: Sides = Sides::Black;

impl Position {
    pub fn new() -> Self {
        Position {
            boards: Board::empty_bitboards(),
            colour_to_move: WHITE_SIDE,
            castling_rights: CastleRights(Castle::all()),
            en_passant_square: None,
            half_move_clock: 0,
            full_move_counter: 1,
            material_score : [0.0;2]
        }
    }

    pub fn render_fancy(&self) {
        let all_pieces = Piece::gen_all();

        for rank in 0..8 {
            print!(" {} ", 8 - rank);

            for file in 0..8 {
                let square = rank * 8 + file;
                let square_mask = 1u64 << square;

                let piece_char = self.boards
                    .iter()
                    .enumerate()
                    .find(|(_, &bitboard)| bitboard & square_mask != 0)
                    .map(|(idx, _)| all_pieces[idx].fancy_char())
                    .unwrap_or(".");

                print!(" {}", piece_char);
            }
            println!();
        }
        println!("    a b c d e f g h\n");
        println!("Castling: {:?}",self.castling_rights.bits());
        println!("En passant: {:?}",self.en_passant_square);
        println!("Half move clock: {}",self.half_move_clock);
        println!("Full move counter: {}",self.full_move_counter);
        println!("Material score: {:?}",self.material_score);
        println!("Colour to move: {:?}",self.colour_to_move);
    }

    fn parse_bitboards(str: &str) -> Result<[Bitboard;12], String> {
        let row_count = str.matches('/').count() + 1;

        if row_count != 8 {
            return Err(format!("board must contain 8 rows, got {}", row_count));
        }

        let mut boards = Board::empty_bitboards();
        let mut square_index = Squares::a8 as u8;

        for char in str.chars() {
            if char == '/' {
                continue;
            }

            if char.is_digit(10) {
                square_index += char.to_digit(10).unwrap() as u8;
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

            let square = Squares::from_u8(square_index);
            match square {
                Some(square) => {
                    set_bit!(square, boards[piece.index()]);
                }
                None => {
                    eprintln!("put_piece: Imagine being so retard: Square is out of bounds")
                }
            };
            square_index += 1;
        }

        if square_index != 64 {
            return Err("board must contain exactly 64 squares".to_string());
        }

        Ok(boards)
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
            return Ok(CastleRights(Castle::NA));
        }

        let mut rights = CastleRights(Castle::NA);

        for char in str.chars() {
            match char {
                'K' => rights.0 |= Castle::WK,
                'Q' => rights.0 |= Castle::WQ,
                'k' => rights.0 |= Castle::BK,
                'q' => rights.0 |= Castle::BQ,
                _ => return Err("invalid castling rights".to_string()),
            };
        }
        Ok(rights)
    }

    fn parse_en_passant_square(square: &str) -> Result<Option<Squares>, String> {
        if square == "-" {
            return Ok(None);
        }

        let result = square.parse::<Squares>();

        if result.is_err() {
            return Err("invalid en passant square".to_string());
        }

        let square = result?;

        println!("{:?}", square.rank());
        if square.rank() != 2 || square.rank() != 5 {
            return Err("invalid en passant square".to_string());
        }

        Ok(Some(square))
    }

    fn calculate_score() -> [f64;2] {
        [0.0,0.0]
    }
}

impl std::str::FromStr for Position {
    type Err = String;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        if parts.len() != FEN_NUM_PARTS {
            return Err(format!(
                "FEN must contain {FEN_NUM_PARTS} parts, got {}",
                parts.len()
            ));
        }

        let score = Self::calculate_score();

        Ok(Position {
            boards: Self::parse_bitboards(parts[0])?,
            colour_to_move: Self::parse_colour_to_move(parts[1])?,
            castling_rights: Self::parse_castling_rights(parts[2])?,
            en_passant_square: Self::parse_en_passant_square(parts[3])?,
            half_move_clock: parts[4].parse().unwrap(),
            full_move_counter: parts[5].parse().unwrap(),
            material_score : score
        })
    }


}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_error_with_wrong_number_of_parts() {
        assert_parse_error("w - - 0 1", "FEN must contain 6 parts, got 5");
        assert_parse_error(
            "8/8/8/8/8/8/8/8 w - - 0 1 extra",
            "FEN must contain 6 parts, got 7",
        );
    }

    #[test]
    fn parse_error_with_wrong_number_of_rows() {
        assert_parse_error("8/8 w - - 0 1", "board must contain 8 rows, got 2");
        assert_parse_error(
            "8/8/8/8/8/8/8/8/1 w - - 0 1",
            "board must contain 8 rows, got 9",
        );
    }

    #[test]
    fn parse_error_with_wrong_number_of_squares() {
        assert_parse_error("8/8/8/8/8/8/8/7 w - - 0 1", "board must contain exactly 64 squares");
        assert_parse_error("8/8/8/8/8/8/8/9 w - - 0 1", "board must contain exactly 64 squares");
    }

    #[test]
    fn parse_error_with_invalid_piece() {
        assert_parse_error("8/8/8/8/8/8/8/4a3 w - - 0 1", "invalid piece 'a'");
    }

    #[test]
    fn parse_with_white_to_move() {
        let parse = "8/8/8/8/8/8/8/8 w - - 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().colour_to_move, Sides::White);
    }

    #[test]
    fn parse_with_black_to_move() {
        let parse = "8/8/8/8/8/8/8/8 b - - 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().colour_to_move, Sides::Black);
    }

    #[test]
    fn parse_error_with_invalid_colour_to_move() {
        assert_parse_error("8/8/8/8/8/8/8/8 W - - 0 1", "invalid colour to move 'W'");
    }

    #[test]
    fn parse_with_no_castling_rights() {
        let parse = "8/8/8/8/8/8/8/8 w - - 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().castling_rights.0, Castle::none());
    }

    #[test]
    fn parse_with_partial_castling_rights() {
        let parse = "8/8/8/8/8/8/8/8 w Kq - 0 1".parse::<Position>();

        assert!(parse.is_ok());

        assert_eq!(
            parse.unwrap().castling_rights,
            CastleRights(Castle::from(&[Castle::WK, Castle::BQ]))
        );
    }

    #[test]
    fn parse_with_all_castling_rights() {
        let parse = "8/8/8/8/8/8/8/8 w KQkq - 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().castling_rights.0, Castle::all());
    }

    #[test]
    fn parse_error_with_invalid_castling_rights() {
        assert_parse_error("8/8/8/8/8/8/8/8 w K- - 0 1", "invalid castling rights");
    }

    #[test]
    fn parse_with_no_en_passant_square() {
        let parse = "8/8/8/8/8/8/8/8 w - - 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().en_passant_square, None);
    }

    #[test]
    fn parse_with_en_passant_square_3rd_rank() {
        let parse = "8/8/8/8/8/8/8/8 w - f3 0 1".parse::<Position>();

        assert!(parse.is_ok());
        assert_eq!(parse.unwrap().en_passant_square, Some(parse_square("f3")));
    }

    #[test]
    fn parse_with_en_passant_square_6th_rank() {
        let parse = "rnbqkbnr/ppp1pppp/8/4pp2/8/8/PPP1PPPP/RNBQKBNR w KQkq d6 0 2".parse::<Position>();

        assert_eq!(parse.unwrap().en_passant_square, Some(parse_square("d6")));
    }

    #[test]
    fn parse_error_with_invalid_en_passant_square() {
        assert_parse_error("8/8/8/8/8/8/8/8 w - f4 0 1", "invalid en passant square");
    }

    #[test]
    fn parse_with_move_counters() {
        let parse = "8/8/8/8/8/8/8/8 w - - 10 20".parse::<Position>();

        assert!(parse.is_ok());

        let pos = parse.unwrap();
        assert_eq!(pos.half_move_clock, 10);
        assert_eq!(pos.full_move_counter, 20);
    }

    fn parse_square(str: &str) -> Squares {
        let square = str.parse();
        assert!(square.is_ok());

        square.unwrap()
    }

    fn assert_parse_error(fen: &str, err: &str) {
        let parse = fen.parse::<Position>();
        assert!(parse.is_err());
        assert_eq!(parse.unwrap_err(), err.to_string());
    }
}
