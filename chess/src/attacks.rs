use std::sync::OnceLock;
use crate::utils::{get_bit, set_bit};

pub static KNIGHT_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();
pub static KING_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();
pub static WHITE_PAWN_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();
pub static BLACK_PAWN_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();

pub fn precompute_knight_attacks() {
    let knight_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2)
    ];
    
    let mut attacks = [0u64; 64];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut attack_mask: u64 = 0;
        
        for &(dr, df) in &knight_moves {
            let new_rank = rank as i32 + dr;
            let new_file = file as i32 + df;
            
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as u8;
                set_bit(&mut attack_mask, target_square);
            }
        }
        
        attacks[square as usize] = attack_mask;
    }
    
    let _ = KNIGHT_ATTACKS.set(attacks);
}

pub fn precompute_king_attacks() {
    let king_moves: [(i32, i32); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1)
    ];
    
    let mut attacks = [0u64; 64];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut attack_mask = 0;
        
        for &(dr, df) in &king_moves {
            let new_rank = rank as i32 + dr;
            let new_file = file as i32 + df;
            
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as u8;
                set_bit(&mut attack_mask, target_square);
            }
        }
        
        attacks[square as usize] = attack_mask;
    }
    
    let _ = KING_ATTACKS.set(attacks);
}

pub fn precompute_pawn_attacks() {
    let mut white_attacks = [0u64; 64];
    let mut black_attacks = [0u64; 64];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        
        // White pawns
        if rank < 7 {
            if file > 0 { set_bit(&mut white_attacks[square as usize], square + 7); } // up-left
            if file < 7 { set_bit(&mut white_attacks[square as usize], square + 9); } // up-right
        }
        
        // Black pawns
        if rank > 0 {
            if file > 0 { set_bit(&mut black_attacks[square as usize], square - 9); } // down-left
            if file < 7 { set_bit(&mut black_attacks[square as usize], square - 7); } // down-right
        }
    }
    
    let _ = WHITE_PAWN_ATTACKS.set(white_attacks);
    let _ = BLACK_PAWN_ATTACKS.set(black_attacks);
}

pub fn get_bishop_attacks(square: u8, blockers: u64) -> u64 {
    let mut attacks = 0;
    let (rank, file) = (square / 8, square % 8);
    
    // Diagonal: up-right
    let (mut r, mut f) = (rank as i32 - 1, file as i32 + 1);
    while r >= 0 && f < 8 {
        let target = (r * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r -= 1;
        f += 1;
    }
    
    // Diagonal: up-left
    let (mut r, mut f) = (rank as i32 - 1, file as i32 - 1);
    while r >= 0 && f >= 0 {
        let target = (r * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r -= 1;
        f -= 1;
    }
    
    // Diagonal: down-right
    let (mut r, mut f) = (rank as i32 + 1, file as i32 + 1);
    while r < 8 && f < 8 {
        let target = (r * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r += 1;
        f += 1;
    }
    
    // Diagonal: down-left
    let (mut r, mut f) = (rank as i32 + 1, file as i32 - 1);
    while r < 8 && f >= 0 {
        let target = (r * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r += 1;
        f -= 1;
    }
    
    attacks
}

pub fn get_rook_attacks(square: u8, blockers: u64) -> u64 {
    let mut attacks = 0;
    let (rank, file) = (square / 8, square % 8);
    
    // Up
    let mut r = rank as i32 - 1;
    while r >= 0 {
        let target = (r * 8 + file as i32) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r -= 1;
    }
    
    // Down
    let mut r = rank as i32 + 1;
    while r < 8 {
        let target = (r * 8 + file as i32) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        r += 1;
    }
    
    // Right
    let mut f = file as i32 + 1;
    while f < 8 {
        let target = (rank as i32 * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        f += 1;
    }
    
    // Left
    let mut f = file as i32 - 1;
    while f >= 0 {
        let target = (rank as i32 * 8 + f) as u8;
        set_bit(&mut attacks, target);
        if get_bit(blockers, target) { break; }
        f -= 1;
    }
    
    attacks
}

pub fn get_queen_attacks(square: u8, blockers: u64) -> u64 {
    get_bishop_attacks(square, blockers) | get_rook_attacks(square, blockers)
}

