use crate::utils::*;
use crate::attacks::*;
use crate::types::*;

pub fn get_pawn_attacks_bitboard(board: &[u64; 12], white_pawns: bool) -> u64 {
    let pawns: u64 = if white_pawns { board[WP] } else { board[BP] };
    let mut attacks: u64 = 0;
    
    let mut pawns_copy: u64 = pawns;
    while pawns_copy != 0 {
        let square = get_lsb(pawns_copy).unwrap();
        clear_bit(&mut pawns_copy, square);
        
        if white_pawns {
            attacks |= WHITE_PAWN_ATTACKS.get().unwrap()[square as usize];
        } else {
            attacks |= BLACK_PAWN_ATTACKS.get().unwrap()[square as usize];
        }
    }
    attacks
}

pub fn get_knight_attacks_bitboard(board: &[u64; 12], white_knights: bool) -> u64 {
    let knights = if white_knights { board[WN] } else { board[BN] };
    let mut attacks = 0;
    
    let mut knights_copy = knights;
    while knights_copy != 0 {
        let square = get_lsb(knights_copy).unwrap();
        clear_bit(&mut knights_copy, square);
        attacks |= KNIGHT_ATTACKS.get().unwrap()[square as usize];
    }
    attacks
}

pub fn get_bishop_attacks_bitboard(board: &[u64; 12], white_bishops: bool) -> u64 {
    let bishops = if white_bishops { board[WB] } else { board[BB] };
    let all_occupied = get_all_occupied(*board);
    let mut attacks = 0;
    
    let mut bishops_copy = bishops;
    while bishops_copy != 0 {
        let square = get_lsb(bishops_copy).unwrap();
        clear_bit(&mut bishops_copy, square);
        
        attacks |= get_bishop_attacks(square, all_occupied);
    }
    attacks
}

pub fn get_rook_attacks_bitboard(board: &[u64; 12], white_rooks: bool) -> u64 {
    let rooks = if white_rooks { board[WR] } else { board[BR] };
    let all_occupied = get_all_occupied(*board);
    let mut attacks = 0;
    
    let mut rooks_copy = rooks;
    while rooks_copy != 0 {
        let square = get_lsb(rooks_copy).unwrap();
        clear_bit(&mut rooks_copy, square);
        
        attacks |= get_rook_attacks(square, all_occupied);
    }
    attacks
}

pub fn get_queen_attacks_bitboard(board: &[u64; 12], white_queens: bool) -> u64 {
    let queens = if white_queens { board[WQ] } else { board[BQ] };
    let all_occupied = get_all_occupied(*board);
    let mut attacks = 0;
    
    let mut queens_copy = queens;
    while queens_copy != 0 {
        let square = get_lsb(queens_copy).unwrap();
        clear_bit(&mut queens_copy, square);
        
        attacks |= get_queen_attacks(square, all_occupied);
    }
    attacks
}

pub fn get_king_attacks_bitboard(board: &[u64; 12], white_king: bool) -> u64 {
    let king = if white_king { board[WK] } else { board[BK] };
    let mut attacks = 0;
    
    if king != 0 {
        let square = get_lsb(king).unwrap();
        attacks |= KING_ATTACKS.get().unwrap()[square as usize];
    }
    attacks
}

pub fn complete_attacks_bitboard(board : &[u64; 12], white_attacking: bool) -> u64 {
    let mut attacks: u64 = 0;

    attacks |= get_pawn_attacks_bitboard(board, white_attacking);
    attacks |= get_knight_attacks_bitboard(board, white_attacking);
    attacks |= get_bishop_attacks_bitboard(board, white_attacking);
    attacks |= get_rook_attacks_bitboard(board, white_attacking);
    attacks |= get_queen_attacks_bitboard(board, white_attacking);
    attacks |= get_king_attacks_bitboard(board, white_attacking);

    attacks
}

pub fn is_check(board: [u64; 12], white_king: bool) -> bool {
    let attack_bitboard = if white_king {
        complete_attacks_bitboard(&board, false) // Black attacks white king
    } else {
        complete_attacks_bitboard(&board, true)  // White attacks black king
    };

    let king_square = if white_king {
        get_lsb(board[WK])
    } else {
        get_lsb(board[BK])
    };

    match king_square {
        Some(square) => get_bit(attack_bitboard, square),
        None => false,
    }
}

