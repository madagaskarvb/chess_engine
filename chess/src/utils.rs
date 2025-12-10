use crate::types::{WP, WN, WB, WR, WQ, WK, BP, BN, BB, BR, BQ, BK};

#[inline]
pub fn get_bit(bitboard: u64, square: u8) -> bool {
    return (bitboard >> square) & 1 != 0;
}

#[inline]
pub fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1 << square;
}

#[inline]
pub fn clear_bit(bitboard: &mut u64, square: u8) {
    *bitboard &= !(1 << square);
}

pub fn count_bits(bitboard: u64) -> u32 {
    bitboard.count_ones()
}

#[inline]
pub fn get_lsb(bitboard: u64) -> Option<u8> {
    if bitboard == 0 {
        None
    } else {
        Some(bitboard.trailing_zeros() as u8)
    }
}

pub fn get_all_white(board : [u64; 12]) -> u64 {
    board[WP] | board[WN] | board[WB] | board[WR] | board[WQ] | board[WK]
}

pub fn get_all_black(board : [u64; 12]) -> u64 {
    board[BP] | board[BN] | board[BB] | board[BR] | board[BQ] | board[BK]
}

pub fn get_all_occupied(board: [u64; 12]) -> u64 {
    get_all_black(board) | get_all_white(board)
}

pub fn get_all_empty(board: [u64; 12]) -> u64 {
    !(get_all_black(board) | get_all_white(board))
}

pub fn is_square_occupied(board: &[u64; 12], square: u8) -> bool {
    get_bit(get_all_occupied(*board), square)
}

pub fn get_piece_at_square(board: &[u64; 12], square: u8) -> Option<usize> {
    for piece_type in 0..12 {
        if get_bit(board[piece_type], square) {
            return Some(piece_type);
        }
    }
    None
}

pub fn get_piece_value(piece_type: usize) -> i32 {
    match piece_type {
        WP | BP => 100,
        WN | BN => 300,
        WB | BB => 300,
        WR | BR => 500,
        WQ | BQ => 900,
        WK | BK => 0, // Kings aren't counted in material
        _ => 0,
    }
}

