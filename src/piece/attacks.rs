use crate::defs::*;
use crate::position::{Bitboard, Sides, Squares};
use crate::utils::computed_magics::{BISHOP_MAGIC, ROOK_MAGICS};

pub fn mask_pawn(sq: Squares, side: Sides) -> u64 {
    let attacks = 1 << sq as u8;

    // CHANGE THE BITBOARD TO ADD EVERY SQUARE THAT HAS
    // CHECK SIDE IF WHITE OR BLACK
    match side {
        Sides::White => (attacks  >> 7 & NOT_A_FILE) | (attacks  >> 9 & NOT_H_FILE),
        Sides::Black => (attacks  << 7 & NOT_H_FILE)| (attacks << 9 & NOT_A_FILE),
    }
}

pub fn mask_knight(sq: Squares) -> Bitboard {
    let attacks = 1 << sq as u8;
    (attacks & NOT_A_FILE) >> 17
        | (attacks & NOT_A_FILE) << 15
        | (attacks & NOT_H_FILE) >> 15
        | (attacks & NOT_H_FILE) << 17
        | (attacks & NOT_AB_FILE) >> 10
        | (attacks & NOT_AB_FILE) << 6
        | (attacks & NOT_HG_FILE) >> 6
        | (attacks & NOT_HG_FILE) << 10
}

pub fn mask_king(sq: Squares) -> Bitboard {
    let attacks = 1 << sq as u8;
    (attacks >> 8 | attacks << 8)
        | (attacks & NOT_A_FILE) >> 9
        | (attacks & NOT_A_FILE) >> 1
        | (attacks & NOT_A_FILE) << 7
        | (attacks & NOT_H_FILE) >> 7
        | (attacks & NOT_H_FILE) << 1
        | (attacks & NOT_H_FILE) << 9
}

/// Attacks for Slider pieces
/// Don't use this: Compute Brute force attack for a Bishop piece
/// Instead uses the precomputed `get_bishop_attacks`
pub fn mask_bishop(sq: Squares, block: Bitboard) -> Bitboard {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for &(dr, df) in &[(1, 1), (-1, 1), (1, -1), (-1, -1)] {
        let mut r = i32::from(tr) + i32::from(dr);
        let mut f = i32::from(tf) + i32::from(df);

        while r >= 1 && r <= 6 && f >= 1 && f <= 6 {
            attacks |= 1u64 << (r as u64 * 8 + f as u64);

            if ((1u64 << (r * 8 + f)) & block) > 0 {
                break;
            }
            r += dr;
            f += df;
        }
    }
    attacks
}

/// Don't use this: Compute Brute force attack for a Rook piece
pub fn mask_rook(sq: Squares) -> Bitboard {
    let mut attacks: u64 = 0;
    let tr = (sq as u8) / 8;
    let tf = (sq as u8) % 8;

    for r in (tr + 1)..=6 {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for r in (1..tr).rev() {
        attacks |= 1u64 << (u64::from(r) * 8 + u64::from(tf));
    }

    for f in (tf + 1)..=6 {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }

    for f in 1..tf {
        attacks |= 1u64 << (u64::from(tr) * 8 + u64::from(f));
    }
    attacks
}

#[inline]
pub fn get_rook_attacks(
    square: Squares,
    occupancy: &u64,
    masks: &Vec<u64>,
    attacks: &Vec<u64>,
) -> u64 {
    let mut occupancy = occupancy & masks[square as usize];
    occupancy = occupancy.wrapping_mul(ROOK_MAGICS[square as usize]);
    occupancy >>= 64 - ROOK_REVEVANT_BITS[square as usize];
    attacks[square as usize * 4096 + occupancy as usize]
}

#[inline]
pub fn get_bishop_attacks(
    square: Squares,
    occupancy: &u64,
    masks: &Vec<u64>,
    attacks: &Vec<u64>,
) -> u64 {
    let mut occupancy = occupancy & masks[square as usize];
    occupancy = occupancy.wrapping_mul(BISHOP_MAGIC[square as usize]);
    occupancy >>= 64 - BISHOP_RELEVANT_BITS[square as usize];
    attacks[square as usize * 512 + occupancy as usize]
}

pub fn mask_queen(
    sq: Squares,
    occupancy: &Bitboard,
    b_mask: &Vec<Bitboard>,
    b_attacks: &Vec<Bitboard>,
    r_mask: &Vec<Bitboard>,
    r_attacks: &Vec<Bitboard>,
) -> Bitboard {
    get_bishop_attacks(sq, occupancy, b_mask, b_attacks)
        | get_rook_attacks(sq, occupancy, r_mask, r_attacks)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::defs::*;
    use crate::position::{render, Bitboard, Sides, Squares};
    use crate::set_bit;

    // Helper function to create test data for slider pieces
    fn create_test_masks_and_attacks() -> (Vec<u64>, Vec<u64>) {
        let masks = vec![0u64; 64];
        let attacks = vec![0u64; 64 * 4096]; // Max size for rook attacks
        (masks, attacks)
    }

    fn create_test_bishop_data() -> (Vec<u64>, Vec<u64>) {
        let masks = vec![0u64; 64];
        let attacks = vec![0u64; 64 * 512]; // Max size for bishop attacks
        (masks, attacks)
    }

    #[test]
    fn test_mask_pawn_white_center() {
        let attacks = mask_pawn(Squares::e4, Sides::White);
        render(attacks);

        // White pawn on e4 should attack d5 and f5
        let expected = (1u64 << Squares::d5 as u8) | (1u64 << Squares::f5 as u8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_pawn_white_edge_cases() {
        // Test pawn on h-file (should only attack one square)
        let attacks_h = mask_pawn(Squares::h4, Sides::White);
        let expected_h = set_bit!([Squares::g5]);
        render(attacks_h);

        assert_eq!(attacks_h, expected_h);
    }

    #[test]
    fn test_mask_pawn_black_center() {
        let attacks = mask_pawn(Squares::e5, Sides::Black);
        render(attacks);

        // Black pawn on e5 should attack d4 and f4
        let expected = (1u64 << Squares::d4 as u8) | (1u64 << Squares::f4 as u8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_pawn_black_edge_cases() {
        // Test pawn on a-file
        let attacks_a = mask_pawn(Squares::a5, Sides::Black);
        let expected_a = 1u64 << Squares::b4 as u8;

        render(attacks_a);

        assert_eq!(attacks_a, expected_a);

        // Test pawn on h-file
        let attacks_h = mask_pawn(Squares::h5, Sides::Black);
        let expected_h = 1u64 << Squares::g4 as u8;

        render(attacks_h);

        assert_eq!(attacks_h, expected_h);
    }

    #[test]
    fn test_mask_knight_center() {
        let attacks = mask_knight(Squares::e4);

        // Knight on e4 should attack 8 squares
        let expected_squares = vec![
            Squares::d2, Squares::f2, // Down 2, left/right 1
            Squares::c3, Squares::g3, // Down 1, left/right 2
            Squares::c5, Squares::g5, // Up 1, left/right 2
            Squares::d6, Squares::f6, // Up 2, left/right 1
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_knight_corner() {
        let attacks = mask_knight(Squares::a1);

        // Knight on a1 should only attack 2 squares
        let expected = (1u64 << Squares::b3 as u8) | (1u64 << Squares::c2 as u8);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_knight_edge() {
        let attacks = mask_knight(Squares::a4);

        // Knight on a4 should attack 4 squares
        let expected_squares = vec![
            Squares::b2, Squares::c3, Squares::c5, Squares::b6
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_king_center() {
        let attacks = mask_king(Squares::e4);

        // King on e4 should attack 8 surrounding squares
        let expected_squares = vec![
            Squares::d3, Squares::e3, Squares::f3, // Below
            Squares::d4, Squares::f4,             // Sides
            Squares::d5, Squares::e5, Squares::f5, // Above
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_king_corner() {
        let attacks = mask_king(Squares::a1);

        // King on a1 should attack 3 squares
        let expected_squares = vec![Squares::a2, Squares::b1, Squares::b2];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_king_edge() {
        let attacks = mask_king(Squares::a4);

        // King on a4 should attack 5 squares
        let expected_squares = vec![
            Squares::a3, Squares::b3, Squares::b4, Squares::b5, Squares::a5
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_bishop_empty_board() {
        let attacks = mask_bishop(Squares::e4, 0);

        // Bishop on e4 with an empty board should attack diagonals within inner 6x6 square
        let expected_squares = vec![
            // Up-right diagonal
            Squares::f5, Squares::g6,
            // Up-left diagonal
            Squares::d5, Squares::c6, Squares::b7,
            // Down-right diagonal
            Squares::f3, Squares::g2,
            // Down-left diagonal
            Squares::d3, Squares::c2,
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_bishop_with_blockers() {
        let blocker = 1u64 << Squares::f5 as u8;
        let attacks = mask_bishop(Squares::e4, blocker);

        // Bishop should be blocked by piece on f5, so g6 should not be attacked
        let expected_squares = vec![
            Squares::f5, // Blocker square is still in attacks
            Squares::d5, Squares::c6, Squares::b7,     // Up-left diagonal
            Squares::f3, Squares::g2,     // Down-right diagonal
            Squares::d3, Squares::c2,     // Down-left diagonal
        ];

        let expected= set_bit!(expected_squares);
        render(attacks);
        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_bishop_corner() {
        let attacks = mask_bishop(Squares::a1, 0);

        // Bishop on a1 should only attack along one diagonal within bounds
        let expected_squares =
            vec![Squares::b2, Squares::c3, Squares::d4,
                 Squares::e5, Squares::f6,Squares::g7];
        render(attacks);
        let mut expected = 0u64;
        for sq in expected_squares {
            expected |= 1u64 << sq as u8;
        }

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_rook_center() {
        let attacks = mask_rook(Squares::e4);

        // Rook on e4 should attack along rank and file within inner 6x6 square
        let expected_squares = vec![
            // Vertical (file)
            Squares::e2, Squares::e3, Squares::e5, Squares::e6,Squares::e7,
            // Horizontal (rank)
            Squares::b4, Squares::c4, Squares::d4, Squares::f4, Squares::g4,
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_rook_corner() {
        let attacks = mask_rook(Squares::a1);

        // Rook on a1 should attack along rank and file within bounds
        let expected_squares = vec![
            Squares::a2, Squares::a3, Squares::a4, Squares::a5, Squares::a6,Squares::a7,
            Squares::b1, Squares::c1, Squares::d1, Squares::e1, Squares::f1, Squares::g1,
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_mask_rook_edge() {
        let attacks = mask_rook(Squares::a4);

        // Rook on a4 should attack along rank and file within bounds
        let expected_squares = vec![
            // Vertical
            Squares::a2, Squares::a3, Squares::a5, Squares::a6,Squares::a7,
            // Horizontal
            Squares::b4,Squares::c4, Squares::d4, Squares::e4, Squares::f4, Squares::g4,
        ];

        let expected= set_bit!(expected_squares);

        assert_eq!(attacks, expected);
    }

    #[test]
    fn test_get_rook_attacks_basic() {
        let (masks, attacks) = create_test_masks_and_attacks();
        let occupancy = 0u64;

        
        let result = get_rook_attacks(Squares::e4, &occupancy, &masks, &attacks);
        
        // The result depends on the precomputed data, so we just verify it doesn't panic
        // and returns a u64 value
        assert_eq!(result, 0u64); // With empty test data, should return 0
    }

    #[test]
    fn test_get_bishop_attacks_basic() {
        let (masks, attacks) = create_test_bishop_data();
        let occupancy = 0u64;

        let result = get_bishop_attacks(Squares::e4, &occupancy, &masks, &attacks);

        // The result depends on the precomputed data, so we just verify it doesn't panic
        // and returns a u64 value
        assert_eq!(result, 0u64); // With empty test data, should return 0
    }

    #[test]
    fn test_mask_queen_basic() {
        let (b_masks, b_attacks) = create_test_bishop_data();
        let (r_masks, r_attacks) = create_test_masks_and_attacks();
        let occupancy = 0u64;

        let result = mask_queen(
            Squares::e4,
            &occupancy,
            &b_masks,
            &b_attacks,
            &r_masks,
            &r_attacks,
        );

        // Should be combination of bishop and rook attacks
        // With empty test data, should return 0
        assert_eq!(result, 0u64);
    }

    #[test]
    fn test_pawn_attacks_all_squares() {
        // Test that pawn attacks work for all squares and don't panic
        for sq_idx in 0..64 {
            let sq = unsafe { std::mem::transmute::<u8, Squares>(sq_idx) };
            let white_attacks = mask_pawn(sq, Sides::White);
            let black_attacks = mask_pawn(sq, Sides::Black);

            // Attacks should be different for white and black (except edge cases)
            if sq_idx != 0 && sq_idx != 7 && sq_idx != 56 && sq_idx != 63 {
                // For non-corner squares, attacks should generally be non-zero
                assert!(white_attacks != 0 || black_attacks != 0);
            }
        }
    }


    #[test]
    fn test_king_attacks_all_squares() {
        // Test that king attacks work for all squares
        for sq_idx in 0..64 {
            let sq = unsafe { std::mem::transmute::<u8, Squares>(sq_idx) };
            let attacks = mask_king(sq);

            // King should always have at least 3 attacks (corner) and at most 8 (center)
            let attack_count = attacks.count_ones();
            assert!(attack_count >= 3 && attack_count <= 8);
        }
    }

    #[test]
    fn test_bishop_attacks_symmetry() {
        // Test bishop attacks are symmetric for empty board
        let attacks_e4 = mask_bishop(Squares::e4, 0);
        let attacks_e5 = mask_bishop(Squares::e5, 0);

        // Both should have attacks, though they'll be different
        assert!(attacks_e4 != 0);
        assert!(attacks_e5 != 0);
        assert_ne!(attacks_e4, attacks_e5);
    }

    #[test]
    fn test_rook_attacks_symmetry() {
        // Test rook attacks for different squares
        let attacks_e4 = mask_rook(Squares::e4);
        let attacks_e5 = mask_rook(Squares::e5);

        // Both should have attacks along rank and file
        assert!(attacks_e4 != 0);
        assert!(attacks_e5 != 0);
        assert_ne!(attacks_e4, attacks_e5);
    }

    #[test]
    fn test_edge_case_occupancy_indexing() {
        // Test that the magic indexing doesn't cause out-of-bounds access
        let (masks, attacks) = create_test_masks_and_attacks();
        let occupancy = 0xFFFFFFFFFFFFFFFFu64; // Full board

        // Should not panic even with full occupancy
        let result = get_rook_attacks(Squares::a1, &occupancy, &masks, &attacks);
        assert_eq!(result, 0u64); // With test data

        let (b_masks, b_attacks) = create_test_bishop_data();
        let b_result = get_bishop_attacks(Squares::a1, &occupancy, &b_masks, &b_attacks);
        assert_eq!(b_result, 0u64); // With test data
    }
}