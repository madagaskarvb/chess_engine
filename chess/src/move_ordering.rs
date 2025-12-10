use std::sync::OnceLock;
use std::sync::atomic::{AtomicI32, Ordering};
use std::cell::RefCell;
use std::thread_local;
use crate::utils::*;
use crate::types::*;
use crate::board_state::BoardState;
use crate::zobrist::compute_board_hash;
use crate::transposition_table::{get_tt_move, ThreadSafeHistoryTable};
use std::sync::OnceLock;

thread_local! {
    static KILLER_MOVES: RefCell<[[Option<(u8, u8)>; 2]; 64]> = RefCell::new([[None; 2]; 64]);
}

static HISTORY_TABLE: OnceLock<ThreadSafeHistoryTable> = OnceLock::new();

pub fn init_history_table() {
    let _ = HISTORY_TABLE.set(ThreadSafeHistoryTable::new());
}

pub fn order_moves(board: &BoardState, moves: &[(u8, u8)]) -> Vec<(i32, u8, u8)> {
    let mut scored_moves = Vec::with_capacity(moves.len());
    
    // Try to get TT move first
    let hash = compute_board_hash(board);
    let tt_move = get_tt_move(hash);
    
    for &(from, to) in moves {
        let mut score = 0;
        
        // TT move gets highest priority
        if Some((from, to)) == tt_move {
            score = 1_000_000;
        }
        // Captures: MVV-LVA
        else if let Some(captured_piece) = get_piece_at_square(&board.bitboards, to) {
            let victim_value = get_piece_value(captured_piece);
            let aggressor_value = if let Some(aggressor) = get_piece_at_square(&board.bitboards, from) {
                get_piece_value(aggressor)
            } else { 0 };
            
            // MVV-LVA: victim*100 - aggressor
            score = 100_000 + victim_value * 100 - aggressor_value;
            
            // Add bonus for capturing with less valuable pieces
            if aggressor_value < victim_value {
                score += 500; // Good trade
            }
        }
        // Promotions
        else if let Some(piece) = get_piece_at_square(&board.bitboards, from) {
            if (piece == WP && to < 8) || (piece == BP && to >= 56) {
                score = 90_000; // Promotion
            }
        }
        // Killer moves
        else if is_killer_move(from, to, board.white_to_move) {
            score = 80_000;
        }
        // History heuristic
        else {
            score = get_history_score(from, to, board.white_to_move);
        }
        
        // Add some positional bonuses
        // Center control for knights
        if let Some(piece) = get_piece_at_square(&board.bitboards, from) {
            if piece == WN || piece == BN {
                let center_squares = [27, 28, 35, 36]; // d4, e4, d5, e5
                if center_squares.contains(&(to as usize)) {
                    score += 50;
                }
            }
        }
        
        scored_moves.push((score, from, to));
    }
    
    // Sort descending (best moves first)
    scored_moves.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    scored_moves
}

pub fn is_killer_move(from: u8, to: u8, _white_to_move: bool) -> bool {
    KILLER_MOVES.with(|km| {
        let km_ref = km.borrow();
        for ply in 0..64 {
            for slot in 0..2 {
                if let Some((kfrom, kto)) = km_ref[ply][slot] {
                    if kfrom == from && kto == to {
                        return true;
                    }
                }
            }
        }
        false
    })
}

pub fn add_killer_move(ply: usize, from: u8, to: u8) {
    if ply >= 64 {
        return;
    }
    
    KILLER_MOVES.with(|km| {
        let mut km_mut = km.borrow_mut();
        
        // Check if move already exists
        for slot in 0..2 {
            if let Some((kfrom, kto)) = km_mut[ply][slot] {
                if kfrom == from && kto == to {
                    return;
                }
            }
        }
        
        // Shift and add
        km_mut[ply][1] = km_mut[ply][0];
        km_mut[ply][0] = Some((from, to));
    });
}

pub fn get_history_score(from: u8, to: u8, _white_to_move: bool) -> i32 {
    HISTORY_TABLE.get().unwrap().get(from, to)
}

pub fn update_history_score(from: u8, to: u8, depth: i32) {
    HISTORY_TABLE.get().unwrap().update(from, to, depth);
}

