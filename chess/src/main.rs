use std::sync::OnceLock;
use rand::Rng;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicI32, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use std::io::{self, BufRead, Write};


const WP: usize = 0;
const WN: usize = 1;
const WB: usize = 2;
const WR: usize = 3;
const WQ: usize = 4;
const WK: usize = 5;
const BP: usize = 6;
const BN: usize = 7;
const BB: usize = 8;
const BR: usize = 9;
const BQ: usize = 10;
const BK: usize = 11;


const PAWN_TABLE: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
    5,   5,  10,  25,  25,  10,   5,   5,
    0,   0,   0,  20,  20,   0,   0,   0,
    5,  -5, -10,   0,   0, -10,  -5,   5,
    5,  10,  10, -20, -20,  10,  10,   5,
    0,   0,   0,   0,   0,   0,   0,   0,
];

// Knight piece-square table
const KNIGHT_TABLE: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

// Bishop piece-square table
const BISHOP_TABLE: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

// Rook piece-square table
const ROOK_TABLE: [i32; 64] = [
    0,   0,   0,   0,   0,   0,   0,   0,
    5,  10,  10,  10,  10,  10,  10,   5,
   -5,   0,   0,   0,   0,   0,   0,  -5,
   -5,   0,   0,   0,   0,   0,   0,  -5,
   -5,   0,   0,   0,   0,   0,   0,  -5,
   -5,   0,   0,   0,   0,   0,   0,  -5,
   -5,   0,   0,   0,   0,   0,   0,  -5,
    0,   0,   0,   5,   5,   0,   0,   0,
];

// Queen piece-square table
const QUEEN_TABLE: [i32; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,   5,   5,   5,   0, -10,
     -5,   0,   5,   5,   5,   5,   0,  -5,
      0,   0,   5,   5,   5,   5,   0,  -5,
    -10,   5,   5,   5,   5,   5,   0, -10,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

// King piece-square table (middle game)
const KING_TABLE: [i32; 64] = [
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -10, -20, -20, -20, -20, -20, -20, -10,
     20,  20,   0,   0,   0,   0,  20,  20,
     20,  30,  10,   0,   0,  10,  30,  20,
];


fn create_board() -> [u64; 12] {
    let mut board_arr: [u64; 12] = [0; 12];
    
    // Populate WP
    for i in 48..56 {
        board_arr[WP] |= 1u64 << i;
    }

    // Populate WN
    board_arr[WN] |= 1u64 << 57;
    board_arr[WN] |= 1u64 << 62;

    // Populate WB
    board_arr[WB] |= 1u64 << 58;
    board_arr[WB] |= 1u64 << 61;

    // Populate WR
    board_arr[WR] |= 1u64 << 56;
    board_arr[WR] |= 1u64 << 63;

    // Populate WQ
    board_arr[WQ] |= 1u64 << 59;
    
    // Populate WK
    board_arr[WK] |= 1u64 << 60;

    // Populate BP
    for i in 8..16 {
        board_arr[BP] |= 1u64 << i;
    }

    // Populate BN 
    board_arr[BN] |= 1u64 << 1;
    board_arr[BN] |= 1u64 << 6;

    // Populate BB
    board_arr[BB] |= 1u64 << 2;
    board_arr[BB] |= 1u64 << 5;

    // Populate BR
    board_arr[BR] |= 1u64 << 0;
    board_arr[BR] |= 1u64 << 7;

    // Populate BQ
    board_arr[BQ] |= 1u64 << 3;

    // Populate BK
    board_arr[BK] |= 1u64 << 4;

    board_arr
}


//UTILS-----------------------------------------------------------------------------------------------------

#[inline]
fn get_bit(bitboard: u64, square: u8) -> bool {
    return (bitboard >> square) & 1 != 0;
}


#[inline]
fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1 << square;
}


#[inline]
fn clear_bit(bitboard: &mut u64, square: u8) {
    *bitboard &= !(1 << square);
}


fn count_bits(bitboard: u64) -> u32 {
    bitboard.count_ones()
}

#[inline]
fn get_lsb(bitboard: u64) -> Option<u8> {
    if bitboard == 0 {
        None
    } else {
        Some(bitboard.trailing_zeros() as u8)
    }
}


fn get_all_white(board : [u64; 12]) -> u64 {
    board[WP] | board[WN] | board[WB] | board[WR] | board[WQ] | board[WK]
}


fn get_all_black(board : [u64; 12]) -> u64 {
    board[BP] | board[BN] | board[BB] | board[BR] | board[BQ] | board[BK]
}


fn get_all_occupied(board: [u64; 12]) -> u64 {
    get_all_black(board) | get_all_white(board)
}


fn get_all_empty(board: [u64; 12]) -> u64 {
    !(get_all_black(board) | get_all_white(board))
}


fn is_square_occupied(board: &[u64; 12], square: u8) -> bool {
    get_bit(get_all_occupied(*board), square)
}


fn get_piece_at_square(board: &[u64; 12], square: u8) -> Option<usize> {
    for piece_type in 0..12 {
        if get_bit(board[piece_type], square) {
            return Some(piece_type);
        }
    }
    None
}

fn get_piece_value(piece_type: usize) -> i32 {
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

//END OF UTILS---------------------------------------------------------------------------------------------


// COMPUTE ATTACKS-----------------------------------------------------------------------------------------

static KNIGHT_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();

fn precompute_knight_attacks() {
    let knight_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2)
    ];
    
    let mut attacks = [0u64; 64];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut attack_mask = 0;
        
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


static KING_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();

fn precompute_king_attacks() {
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


static WHITE_PAWN_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();
static BLACK_PAWN_ATTACKS: OnceLock<[u64; 64]> = OnceLock::new();

fn precompute_pawn_attacks() {
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


fn get_bishop_attacks(square: u8, blockers: u64) -> u64 {
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


fn get_rook_attacks(square: u8, blockers: u64) -> u64 {
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


fn get_queen_attacks(square: u8, blockers: u64) -> u64 {
    get_bishop_attacks(square, blockers) | get_rook_attacks(square, blockers)
}
//END OF ATTACKS-------------------------------------------------------------------------------------------


//MOVE GENERATION------------------------------------------------------------------------------------------

fn generate_pawn_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let pawns = if white { board[WP] } else { board[BP] };
    let enemy_occupied = if white { get_all_black(*board) } else { get_all_white(*board) };
    let all_occupied = get_all_occupied(*board);
    let empty = get_all_empty(*board);
    
    let mut pawns_copy = pawns;
    while pawns_copy != 0 {
        let from_square = get_lsb(pawns_copy).unwrap();
        clear_bit(&mut pawns_copy, from_square);
        
        let rank = from_square / 8;
        let file = from_square % 8;
        
        if white {
            // Single push forward
            let single_push = from_square - 8;
            if single_push >= 0 && single_push < 64 && get_bit(empty, single_push) {
                moves.push((from_square, single_push));
                
                // Double push from starting rank
                if rank == 6 {  // White pawns start on rank 6 (48-55)
                    let double_push = from_square - 16;
                    if double_push >= 0 && double_push < 64 && get_bit(empty, double_push) {
                        moves.push((from_square, double_push));
                    }
                }
            }
            
            // Captures
            
            let attacks = WHITE_PAWN_ATTACKS.get().unwrap()[from_square as usize];
            let mut attacks_copy = attacks;
            while attacks_copy != 0 {
                let target_square = get_lsb(attacks_copy).unwrap();
                clear_bit(&mut attacks_copy, target_square);
                
                if get_bit(enemy_occupied, target_square) {
                    moves.push((from_square, target_square));
                }
            }
            
        } else { // Black pawns (moving downward)
            // Single push forward
            let single_push = from_square + 8;
            if single_push >= 0 && single_push < 64 && get_bit(empty, single_push) {
                moves.push((from_square, single_push));
                
                // Double push from starting rank  
                if rank == 1 {  // Black pawns start on rank 1 (8-15)
                    let double_push = from_square + 16;
                    if double_push >= 0 && double_push < 64 && get_bit(empty, double_push) {
                        moves.push((from_square, double_push));
                    }
                }
            }
            
            // Captures
            unsafe {
                let attacks = BLACK_PAWN_ATTACKS.get().unwrap()[from_square as usize];
                let mut attacks_copy = attacks;
                while attacks_copy != 0 {
                    let target_square = get_lsb(attacks_copy).unwrap();
                    clear_bit(&mut attacks_copy, target_square);
                    
                    if get_bit(enemy_occupied, target_square) {
                        moves.push((from_square, target_square));
                    }
                }
            }
        }
    }
}


fn generate_knight_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let knights = if white { board[WN] } else { board[BN] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    
    let mut knights_copy = knights;
    while knights_copy != 0 {
        let from_square = get_lsb(knights_copy).unwrap();
        clear_bit(&mut knights_copy, from_square);
        
        let attacks = KNIGHT_ATTACKS.get().unwrap()[from_square as usize];
        let mut attacks_copy = attacks;
        while attacks_copy != 0 {
            let target_square = get_lsb(attacks_copy).unwrap();
            clear_bit(&mut attacks_copy, target_square);
            
            // Only move to empty squares or enemy pieces
            if !get_bit(friendly_occupied, target_square) {
                moves.push((from_square, target_square));
            }
        }
    }
}


fn generate_king_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool, board_state: &BoardState) {
    let king = if white { board[WK] } else { board[BK] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    
    if king != 0 {
        let from_square = get_lsb(king).unwrap();
        let attacks = KING_ATTACKS.get().unwrap()[from_square as usize];
        let mut attacks_copy = attacks;
        while attacks_copy != 0 {
            let target_square = get_lsb(attacks_copy).unwrap();
            clear_bit(&mut attacks_copy, target_square);
            
            if !get_bit(friendly_occupied, target_square) {
                moves.push((from_square, target_square));
            }
        }
        
        // Fix: Pass all required parameters including the king square
        generate_castling_moves(board, moves, white, board_state, from_square);
    }
}


fn generate_castling_moves(
    board: &[u64; 12], 
    moves: &mut Vec<(u8, u8)>, 
    white: bool, 
    board_state: &BoardState, 
    _king_square: u8  // Add underscore since it's unused
) {
    let all_occupied = get_all_occupied(*board);
    let enemy_attacks = complete_attacks_bitboard(board, !white);
    
    if white {
        // White kingside castling (O-O)
        if board_state.white_kingside_castle {
            // Check if squares between king and rook are empty
            let empty_squares = !get_bit(all_occupied, 61) && !get_bit(all_occupied, 62);
            
            // Check if king is not in check and doesn't pass through attacked squares
            let safe_squares = !get_bit(enemy_attacks, 60) &&
                             !get_bit(enemy_attacks, 61) &&
                             !get_bit(enemy_attacks, 62);
            
            if empty_squares && safe_squares {
                moves.push((60, 62));
            }
        }
        
        // White queenside castling (O-O-O)
        if board_state.white_queenside_castle {
            let empty_squares = !get_bit(all_occupied, 59) &&
                             !get_bit(all_occupied, 58) &&
                             !get_bit(all_occupied, 57);
            
            let safe_squares = !get_bit(enemy_attacks, 60) &&
                             !get_bit(enemy_attacks, 59) &&
                             !get_bit(enemy_attacks, 58);
            
            if empty_squares && safe_squares {
                moves.push((60, 58));
            }
        }
    } else {
        // Black kingside castling (O-O)
        if board_state.black_kingside_castle {
            let empty_squares = !get_bit(all_occupied, 5) && !get_bit(all_occupied, 6);
            
            let safe_squares = !get_bit(enemy_attacks, 4) &&
                             !get_bit(enemy_attacks, 5) &&
                             !get_bit(enemy_attacks, 6);
            
            if empty_squares && safe_squares {
                moves.push((4, 6));
            }
        }
        
        // Black queenside castling (O-O-O)
        if board_state.black_queenside_castle {
            let empty_squares = !get_bit(all_occupied, 3) &&
                             !get_bit(all_occupied, 2) &&
                             !get_bit(all_occupied, 1);
            
            let safe_squares = !get_bit(enemy_attacks, 4) &&
                             !get_bit(enemy_attacks, 3) &&
                             !get_bit(enemy_attacks, 2);
            
            if empty_squares && safe_squares {
                moves.push((4, 2));
            }
        }
    }
}



fn generate_bishop_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let bishops = if white { board[WB] } else { board[BB] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut bishops_copy = bishops;
    while bishops_copy != 0 {
        let from_square = get_lsb(bishops_copy).unwrap();
        clear_bit(&mut bishops_copy, from_square);
        
        let attacks = get_bishop_attacks(from_square, all_occupied);
        let mut attacks_copy = attacks;
        while attacks_copy != 0 {
            let target_square = get_lsb(attacks_copy).unwrap();
            clear_bit(&mut attacks_copy, target_square);
            
            if !get_bit(friendly_occupied, target_square) {
                moves.push((from_square, target_square));
            }
        }
    }
}


fn generate_rook_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let rooks = if white { board[WR] } else { board[BR] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut rooks_copy = rooks;
    while rooks_copy != 0 {
        let from_square = get_lsb(rooks_copy).unwrap();
        clear_bit(&mut rooks_copy, from_square);
        
        let attacks = get_rook_attacks(from_square, all_occupied);
        let mut attacks_copy = attacks;
        while attacks_copy != 0 {
            let target_square = get_lsb(attacks_copy).unwrap();
            clear_bit(&mut attacks_copy, target_square);
            
            if !get_bit(friendly_occupied, target_square) {
                moves.push((from_square, target_square));
            }
        }
    }
}


fn generate_queen_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let queens = if white { board[WQ] } else { board[BQ] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut queens_copy = queens;
    while queens_copy != 0 {
        let from_square = get_lsb(queens_copy).unwrap();
        clear_bit(&mut queens_copy, from_square);
        
        let attacks = get_queen_attacks(from_square, all_occupied);
        let mut attacks_copy = attacks;
        while attacks_copy != 0 {
            let target_square = get_lsb(attacks_copy).unwrap();
            clear_bit(&mut attacks_copy, target_square);
            
            if !get_bit(friendly_occupied, target_square) {
                moves.push((from_square, target_square));
            }
        }
    }
}


fn generate_moves(board: [u64; 12], white_move: bool, board_state: &BoardState) -> Vec<(u8, u8)> {
    let mut moves: Vec<(u8, u8)> = Vec::new();
    
    if white_move {
        generate_pawn_moves(&board, &mut moves, true);
        generate_knight_moves(&board, &mut moves, true);
        generate_bishop_moves(&board, &mut moves, true);
        generate_rook_moves(&board, &mut moves, true);
        generate_queen_moves(&board, &mut moves, true);
        generate_king_moves(&board, &mut moves, true, board_state);
    } else {
        generate_pawn_moves(&board, &mut moves, false);
        generate_knight_moves(&board, &mut moves, false);
        generate_bishop_moves(&board, &mut moves, false);
        generate_rook_moves(&board, &mut moves, false);
        generate_queen_moves(&board, &mut moves, false);
        generate_king_moves(&board, &mut moves, false, board_state);
    }
    
    moves
}

//END OF MOVE GENERATION-----------------------------------------------------------------------------------


//ATTACK BITBOARDS-----------------------------------------------------------------------------------------

fn get_pawn_attacks_bitboard(board: &[u64; 12], white_pawns: bool) -> u64 {
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

fn get_knight_attacks_bitboard(board: &[u64; 12], white_knights: bool) -> u64 {
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

fn get_bishop_attacks_bitboard(board: &[u64; 12], white_bishops: bool) -> u64 {
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

fn get_rook_attacks_bitboard(board: &[u64; 12], white_rooks: bool) -> u64 {
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

fn get_queen_attacks_bitboard(board: &[u64; 12], white_queens: bool) -> u64 {
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

fn get_king_attacks_bitboard(board: &[u64; 12], white_king: bool) -> u64 {
    let king = if white_king { board[WK] } else { board[BK] };
    let mut attacks = 0;
    
    if king != 0 {
        let square = get_lsb(king).unwrap();
        attacks |= KING_ATTACKS.get().unwrap()[square as usize];
    }
    attacks
}

fn complete_attacks_bitboard(board : &[u64; 12], white_attacking: bool) -> u64 {
    let mut attacks: u64 = 0;

    attacks |= get_pawn_attacks_bitboard(board, white_attacking);
    attacks |= get_knight_attacks_bitboard(board, white_attacking);
    attacks |= get_bishop_attacks_bitboard(board, white_attacking);
    attacks |= get_rook_attacks_bitboard(board, white_attacking);
    attacks |= get_queen_attacks_bitboard(board, white_attacking);
    attacks |= get_king_attacks_bitboard(board, white_attacking);

    attacks
}


fn is_check(board: [u64; 12], white_king: bool) -> bool {
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


//END OF ATTACK BITBOARDS----------------------------------------------------------------------------------


//PRINTING FUNCTIONS---------------------------------------------------------------------------------------

fn square_to_coordinates(square: u8) -> String {
    let file: u8 = (square % 8) as u8;
    let rank: u8 = (square / 8) as u8;
    let file_char: char = (b'a' + file) as char;
    let rank_char: char = (b'8' - rank) as char; // 0=a8, 63=h1 in your system
    format!("{}{}", file_char, rank_char)
}

fn coordinates_to_square(coords: &str) -> Option<u8> {
    if coords.len() < 2 {
        return None;
    }
    let mut chars = coords.chars();
    let file_char = chars.next()?;
    let rank_char = chars.next()?;
    
    if file_char < 'a' || file_char > 'h' || rank_char < '1' || rank_char > '8' {
        return None;
    }
    
    let file = (file_char as u8) - b'a';
    let rank = (rank_char as u8) - b'1';
    
    // Convert to our square representation (0=a8, 63=h1)
    Some((7 - rank) * 8 + file)
}

fn move_to_uci(from: u8, to: u8) -> String {
    format!("{}{}", square_to_coordinates(from), square_to_coordinates(to))
}

fn uci_to_move(uci_move: &str) -> Option<(u8, u8)> {
    // Handle UCI move format: e2e4 or e7e8q (with promotion)
    // We'll ignore promotion for now as the engine doesn't fully support it yet
    if uci_move.len() < 4 {
        return None;
    }
    let from_str = &uci_move[0..2];
    let to_str = &uci_move[2..4];
    
    let from = coordinates_to_square(from_str)?;
    let to = coordinates_to_square(to_str)?;
    
    Some((from, to))
}


fn print_moves(moves: &[(u8, u8)]) {
    for &(from, to) in moves {
        println!("{} -> {}", square_to_coordinates(from), square_to_coordinates(to));
    }
}


fn print_board(board: &BoardState) {
    println!("\n  a b c d e f g h");
    for rank in 0..8 {
        print!("{} ", 8 - rank);
        for file in 0..8 {
            let square = rank * 8 + file;
            let mut piece_char = '.';
            
            for (piece_type, symbol) in [
                (WP, 'P'), (WN, 'N'), (WB, 'B'), (WR, 'R'), (WQ, 'Q'), (WK, 'K'),
                (BP, 'p'), (BN, 'n'), (BB, 'b'), (BR, 'r'), (BQ, 'q'), (BK, 'k')
            ].iter() {
                if get_bit(board.bitboards[*piece_type], square as u8) {
                    piece_char = *symbol;
                    break;
                }
            }
            
            print!("{} ", piece_char);
        }
        println!();
    }
    println!("Side to move: {}", if board.white_to_move { "white" } else { "black" });
}

//END OF PRINTING FUNCTIONS--------------------------------------------------------------------------------


//MOVE EXECUTION-------------------------------------------------------------------------------------------

struct Move {
    from : u8,
    to : u8,
    piece : usize,
    captured_piece : Option<usize>,
    promotion : Option <usize>,
    castling_move: bool,
    en_passant: bool,
    previous_castling_rights : (bool, bool, bool, bool),
    previous_en_passant_target: Option<u8>,
}

fn make_move(board: &mut BoardState, from: u8, to: u8) -> Option<Move> {
    let previous_castling: (bool, bool, bool, bool) = (
        board.white_kingside_castle,
        board.white_queenside_castle,
        board.black_kingside_castle,
        board.black_queenside_castle,
    );

    let mut moving_piece: Option<usize> = None; 
    let start_range: usize = if board.white_to_move { 0 } else { 6 };
    let end_range: usize = if board.white_to_move { 6 } else { 12 };
    
    for piece_type in start_range..end_range {
        if get_bit(board.bitboards[piece_type], from) {
            moving_piece = Some(piece_type);
            break;
        }
    }
    
    let moving_piece: usize = match moving_piece {
        Some(p) => p,
        None => return None,  // Changed from false to None
    };
    
    // Handle castling moves
    if (moving_piece == WK || moving_piece == BK) && (from as i32 - to as i32).abs() == 2 {
        return make_castling_move(board, from, to, moving_piece == WK);
    }
    
    // Handle captures
    let mut captured_piece: Option<usize> = None;
    let enemy_start: usize = if board.white_to_move { 6 } else { 0 };
    let enemy_end: usize = if board.white_to_move { 12 } else { 6 };
    
    for piece_type in enemy_start..enemy_end {
        if get_bit(board.bitboards[piece_type], to) {
            captured_piece = Some(piece_type);
            break;
        }
    }
    
    // Update bitboards
    clear_bit(&mut board.bitboards[moving_piece], from);
    
    if let Some(captured) = captured_piece {
        clear_bit(&mut board.bitboards[captured], to);
    }
    
    set_bit(&mut board.bitboards[moving_piece], to);
    
    // Update castling rights
    if moving_piece == WK || moving_piece == BK {
        board.king_moved(moving_piece == WK);
    } else if moving_piece == WR || moving_piece == BR {
        board.rook_moved(from, moving_piece == WR);
    }
    
    // Switch sides
    board.white_to_move = !board.white_to_move;
    
    // Update check status
    board.update_check_status();
    
    Some(Move {
        from,
        to,
        piece: moving_piece,
        captured_piece,
        promotion: None,
        castling_move: false,
        en_passant: false,
        previous_castling_rights: previous_castling,
        previous_en_passant_target: None,
    })
}

fn unmake_move(board: &mut BoardState, mv: &Move) {
    board.white_to_move = !board.white_to_move;
    
    clear_bit(&mut board.bitboards[mv.piece], mv.to);
    set_bit(&mut board.bitboards[mv.piece], mv.from);
    
    if let Some(captured_piece) = mv.captured_piece {
        if mv.en_passant {
            let captured_square: u8 = if board.white_to_move {
                mv.to + 8
            } else {
                mv.to - 8
            };
            set_bit(&mut board.bitboards[captured_piece], captured_square);
        } else {
            set_bit(&mut board.bitboards[captured_piece], mv.to);
        }
    }
    
    if mv.castling_move {
        if mv.to == 62 { 
            clear_bit(&mut board.bitboards[WR], 61);
            set_bit(&mut board.bitboards[WR], 63);
        } else if mv.to == 58 { 
            clear_bit(&mut board.bitboards[WR], 59);
            set_bit(&mut board.bitboards[WR], 56);
        } else if mv.to == 6 {
            clear_bit(&mut board.bitboards[BR], 5);
            set_bit(&mut board.bitboards[BR], 7);
        } else if mv.to == 2 {
            clear_bit(&mut board.bitboards[BR], 3);
            set_bit(&mut board.bitboards[BR], 0);
        }
    }
    
    board.white_kingside_castle = mv.previous_castling_rights.0;
    board.white_queenside_castle = mv.previous_castling_rights.1;
    board.black_kingside_castle = mv.previous_castling_rights.2;
    board.black_queenside_castle = mv.previous_castling_rights.3;
    
    board.en_passant_target = mv.previous_en_passant_target;
    
    board.update_check_status();
}

fn make_castling_move(board: &mut BoardState, from: u8, to: u8, white: bool) -> Option<Move> {
    let previous_castling = (
        board.white_kingside_castle,
        board.white_queenside_castle,
        board.black_kingside_castle,
        board.black_queenside_castle,
    );
    
    let (king_from, king_to, rook_from, rook_to) = if white {
        if to == 62 { // Kingside
            (60, 62, 63, 61)
        } else { // Queenside
            (60, 58, 56, 59)
        }
    } else {
        if to == 6 { // Kingside
            (4, 6, 7, 5)
        } else { // Queenside
            (4, 2, 0, 3)
        }
    };
    
    clear_bit(&mut board.bitboards[if white { WK } else { BK }], king_from);
    set_bit(&mut board.bitboards[if white { WK } else { BK }], king_to);
    
    clear_bit(&mut board.bitboards[if white { WR } else { BR }], rook_from);
    set_bit(&mut board.bitboards[if white { WR } else { BR }], rook_to);
    
    if white {
        board.white_kingside_castle = false;
        board.white_queenside_castle = false;
    } else {
        board.black_kingside_castle = false;
        board.black_queenside_castle = false;
    }
    
    board.white_to_move = !board.white_to_move;
    
    board.update_check_status();
    
    Some(Move {
        from,
        to,
        piece: if white { WK } else { BK },
        captured_piece: None,
        promotion: None,
        castling_move: true,
        en_passant: false,
        previous_castling_rights: previous_castling,
        previous_en_passant_target: None,
    })
}

//END OF MOVE EXECUTION-----------------------------------------------------------------------------------


//MOVE HISTORY--------------------------------------------------------------------------------------------

struct SearchState {
    move_history: Vec<Move>,
    board: BoardState,
}

impl SearchState {
    fn make_move(&mut self, from: u8, to: u8) -> bool {
        if let Some(mv) = make_move(&mut self.board, from, to) {
            self.move_history.push(mv);
            true
        } else {
            false
        }
    }
    
    fn unmake_move(&mut self) -> bool {
        if let Some(mv) = self.move_history.pop() {
            unmake_move(&mut self.board, &mv);
            true
        } else {
            false
        }
    }
}

//END OF MOVE HISTORY-------------------------------------------------------------------------------------


//EVALUATION----------------------------------------------------------------------------------------------

fn evaluate_board_fast(board: &BoardState) -> i32 {
    // Use incremental evaluation
    let mut score = 0;
    
    // Material evaluation using bit counting
    const PIECE_VALUES: [i32; 6] = [100, 300, 300, 500, 900, 0]; // P, N, B, R, Q, K
    
    for piece in 0..6 {
        let white_count = count_bits(board.bitboards[piece]) as i32;
        let black_count = count_bits(board.bitboards[piece + 6]) as i32;
        score += PIECE_VALUES[piece] * (white_count - black_count);
    }
    
    // Piece-square tables using precomputed values
    static mut WHITE_PST: [i32; 64] = [0; 64];
    static mut BLACK_PST: [i32; 64] = [0; 64];
    
    unsafe {
        // Precompute these once
        for square in 0..64 {
            let table_index = 63 - square as usize;
            WHITE_PST[square] = PAWN_TABLE[table_index] + KNIGHT_TABLE[table_index] 
                + BISHOP_TABLE[table_index] + ROOK_TABLE[table_index]
                + QUEEN_TABLE[table_index] + KING_TABLE[table_index];
            BLACK_PST[square] = -(PAWN_TABLE[square] + KNIGHT_TABLE[square] 
                + BISHOP_TABLE[square] + ROOK_TABLE[square]
                + QUEEN_TABLE[square] + KING_TABLE[square]);
        }
    }
    
    // Fast piece-square evaluation
    for square in 0..64 {
        let bit = 1u64 << square;
        for piece in 0..6 {
            if (board.bitboards[piece] & bit) != 0 {
                unsafe { score += WHITE_PST[square]; }
            }
            if (board.bitboards[piece + 6] & bit) != 0 {
                unsafe { score += BLACK_PST[square]; }
            }
        }
    }
    
    score
}

fn evaluate_board_advanced(board: &BoardState) -> i32 {
    let mut score = 0;
    
    // Material values
    const PIECE_VALUES: [i32; 12] = [
        100,  // WP - Pawn
        300,  // WN - Knight
        300,  // WB - Bishop
        500,  // WR - Rook
        900,  // WQ - Queen
        0,    // WK - King (not used in material count)
        -100, // BP - Pawn
        -300, // BN - Knight
        -300, // BB - Bishop
        -500, // BR - Rook
        -900, // BQ - Queen
        0,    // BK - King (not used in material count)
    ];
    
    // Count material for each piece type 
    for (piece_type, &value) in PIECE_VALUES.iter().enumerate() {
        if value != 0 {
            let count = count_bits(board.bitboards[piece_type]) as i32;
            score += value * count;
        }
    }
    
    // Piece-square table evaluation
    for square in 0..64 {
        // White pieces (positive values)
        if get_bit(board.bitboards[WP], square) {
            // For white pawns, we need to flip the square index because the table is from white's perspective but our board representation has a8=0, h1=63
            let table_index = 63 - square as usize;
            score += PAWN_TABLE[table_index];
        }
        if get_bit(board.bitboards[WN], square) {
            let table_index = 63 - square as usize;
            score += KNIGHT_TABLE[table_index];
        }
        if get_bit(board.bitboards[WB], square) {
            let table_index = 63 - square as usize;
            score += BISHOP_TABLE[table_index];
        }
        if get_bit(board.bitboards[WR], square) {
            let table_index = 63 - square as usize;
            score += ROOK_TABLE[table_index];
        }
        if get_bit(board.bitboards[WQ], square) {
            let table_index = 63 - square as usize;
            score += QUEEN_TABLE[table_index];
        }
        if get_bit(board.bitboards[WK], square) {
            let table_index = 63 - square as usize;
            score += KING_TABLE[table_index];
        }
        
        // Black pieces (negative values)
        if get_bit(board.bitboards[BP], square) {
            // Black wants the opposite of what white wants
            score -= PAWN_TABLE[square as usize];
        }
        if get_bit(board.bitboards[BN], square) {
            score -= KNIGHT_TABLE[square as usize];
        }
        if get_bit(board.bitboards[BB], square) {
            score -= BISHOP_TABLE[square as usize];
        }
        if get_bit(board.bitboards[BR], square) {
            score -= ROOK_TABLE[square as usize];
        }
        if get_bit(board.bitboards[BQ], square) {
            score -= QUEEN_TABLE[square as usize];
        }
        if get_bit(board.bitboards[BK], square) {
            score -= KING_TABLE[square as usize];
        }
    }
    
    
    score
}

//END OF EVALUATION---------------------------------------------------------------------------------------


//TESTS---------------------------------------------------------------------------------------------------

fn test_performance() {
    println!("\n=== Performance Testing ===");
    
    let board_state = BoardState::new();
    
    println!("Testing move generation speed...");
    
    // Test raw move generation speed
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        let _ = generate_moves(board_state.bitboards, true, &board_state);
    }
    let elapsed = start.elapsed();
    println!("1000 move generations took: {:?}", elapsed);
    println!("Average per generation: {:?}", elapsed / 1000);
    
    // Test legal move filtering speed
    println!("\nTesting legal move filtering...");
    let moves = generate_moves(board_state.bitboards, true, &board_state);
    let start = std::time::Instant::now();
    
    let legal_moves: Vec<(u8, u8)> = moves.into_iter()
        .filter(|&(from, to)| {
            let mut temp_state = SearchState {
                board: board_state,
                move_history: Vec::new(),
            };
            
            if temp_state.make_move(from, to) {
                let our_king_in_check = temp_state.board.white_king_in_check;
                !our_king_in_check
            } else {
                false
            }
        })
        .collect();
    
    let elapsed = start.elapsed();
    println!("Legal moves found: {}", legal_moves.len());
    println!("Filtering took: {:?}", elapsed);
    
    // Test evaluation speed
    println!("\nTesting evaluation speed...");
    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _ = evaluate_board_advanced(&board_state);
    }
    let elapsed = start.elapsed();
    println!("10000 evaluations took: {:?}", elapsed);
    println!("Average per evaluation: {:?}", elapsed / 10000);
}

fn test_minimax() {
    println!("\n=== Testing Minimax Algorithm ===");
    
    let board_state = BoardState::new();
    
    // Test with depth 1
    println!("\nSearching for best move at depth 1...");
    if let Some((from, to)) = find_best_move(&board_state, 1) {
        println!("Best move found: {} -> {}", 
            square_to_coordinates(from), 
            square_to_coordinates(to));
        
        // Show the resulting position
        let mut new_state = board_state;
        make_move(&mut new_state, from, to);
        print_board(&new_state);
        println!("Evaluation after move: {}", evaluate_board_advanced(&new_state));
    } else {
        println!("No legal moves found!");
    }
    
    // Test with depth 2 (more thorough search)
    println!("\nSearching for best move at depth 2...");
    if let Some((from, to)) = find_best_move(&board_state, 2) {
        println!("Best move found: {} -> {}", 
            square_to_coordinates(from), 
            square_to_coordinates(to));
    }
    
    // Test iterative deepening
    println!("\nSearching with iterative deepening (max depth 6, 5 second limit)...");
    if let Some((from, to)) = find_best_move_iterative_deepening_optimized(&board_state, 6, 5000) {
        println!("Best move found: {} -> {}", 
            square_to_coordinates(from), 
            square_to_coordinates(to));
    }
}


fn test_evaluation() {
    let board = BoardState::new();
    let score = evaluate_board_advanced(&board);
    println!("Initial position evaluation: {}", score);
    println!("(Should 0)");
    
    let mut test_board = BoardState::new();

    clear_bit(&mut test_board.bitboards[BP], 8); // Remove a2 pawn
    
    let test_score = evaluate_board_advanced(&test_board);
    println!("White up a pawn evaluation: {}", test_score);
    println!("(Should be around +100)");
    
    println!("\nTesting piece-square tables:");
    
    let mut knight_test = BoardState::new();
    // Clear all knights
    knight_test.bitboards[WN] = 0;
    knight_test.bitboards[BN] = 0;
    
    // Place white knight in center (good square)
    set_bit(&mut knight_test.bitboards[WN], 36); // e4
    // Place black knight on edge (bad square)
    set_bit(&mut knight_test.bitboards[BN], 7);  // h8
    
    let knight_eval = evaluate_board_advanced(&knight_test);
    println!("White knight in center vs black knight on edge: {}", knight_eval);
    println!("(Should show advantage for white due to better knight placement)");
}


fn test_unmake_move() {
    let mut board = BoardState::new();
    let initial_hash = compute_board_hash(&board);
    
    // Create a simple Move struct for testing
    #[derive(Clone, Copy)]
    struct TestMove {
        from: u8,
        to: u8,
        piece: usize,
        captured_piece: Option<usize>,
        previous_castling: (bool, bool, bool, bool),
    }
    
    // Simple make_move for testing
    fn make_move_test(board: &mut BoardState, from: u8, to: u8) -> Option<TestMove> {
        // Find which piece is moving
        let moving_piece = get_piece_at_square(&board.bitboards, from)?;
        
        // Store previous state
        let previous_castling = (
            board.white_kingside_castle,
            board.white_queenside_castle,
            board.black_kingside_castle,
            board.black_queenside_castle,
        );
        
        // Check for capture
        let captured_piece = get_piece_at_square(&board.bitboards, to);
        
        // Update bitboards
        clear_bit(&mut board.bitboards[moving_piece], from);
        if let Some(captured) = captured_piece {
            clear_bit(&mut board.bitboards[captured], to);
        }
        set_bit(&mut board.bitboards[moving_piece], to);
        
        // Update castling rights if king moves
        if moving_piece == WK {
            board.white_kingside_castle = false;
            board.white_queenside_castle = false;
        } else if moving_piece == BK {
            board.black_kingside_castle = false;
            board.black_queenside_castle = false;
        }
        
        // Switch sides
        board.white_to_move = !board.white_to_move;
        
        Some(TestMove {
            from,
            to,
            piece: moving_piece,
            captured_piece,
            previous_castling,
        })
    }
    
    // Simple unmake_move for testing
    fn unmake_move_test(board: &mut BoardState, mv: &TestMove) {
        // Switch side back
        board.white_to_move = !board.white_to_move;
        
        // Move piece back
        clear_bit(&mut board.bitboards[mv.piece], mv.to);
        set_bit(&mut board.bitboards[mv.piece], mv.from);
        
        // Restore captured piece
        if let Some(captured) = mv.captured_piece {
            set_bit(&mut board.bitboards[captured], mv.to);
        }
        
        // Restore castling rights
        board.white_kingside_castle = mv.previous_castling.0;
        board.white_queenside_castle = mv.previous_castling.1;
        board.black_kingside_castle = mv.previous_castling.2;
        board.black_queenside_castle = mv.previous_castling.3;
    }
    
    // Make a move
    if let Some(mv) = make_move_test(&mut board, 52, 36) { // e2-e4
        // Unmake it
        unmake_move_test(&mut board, &mv);
        
        let final_hash = compute_board_hash(&board);
        
        // Print diagnostic information
        println!("Initial hash: 0x{:016x}", initial_hash);
        println!("Final hash:   0x{:016x}", final_hash);
        
        // Check each bitboard
        for i in 0..12 {
            if board.bitboards[i] != create_board()[i] {
                println!("Bitboard {} differs!", i);
            }
        }
        
        assert_eq!(initial_hash, final_hash, "Board state not restored!");
        println!("Test passed! Board state restored correctly.");
    } else {
        panic!("Failed to make move");
    }
}

fn test_depth_x(depth: u8) {
    println!("\n=== Testing Best Move Search at Depth 6 ===");
    
    let board_state = BoardState::new();
    
    // Test 1: Initial position
    println!("\nTest 1: Initial Position (White to move)");
    let start = std::time::Instant::now();
    
    if let Some((from, to)) = find_best_move(&board_state, depth) {
        let elapsed = start.elapsed();
        println!("Best move found: {} -> {}", 
            square_to_coordinates(from), 
            square_to_coordinates(to));
        println!("Search time: {:?}", elapsed);
        
        // Show the move on the board
        let mut new_state = board_state;
        if let Some(mv) = make_move(&mut new_state, from, to) {
            println!("\nPosition after move:");
            print_board(&new_state);
            println!("Evaluation after move: {}", evaluate_board_advanced(&new_state));
            
            // Check if move is a capture
            if let Some(captured) = mv.captured_piece {
                let piece_names = ["WP", "WN", "WB", "WR", "WQ", "WK", 
                                   "BP", "BN", "BB", "BR", "BQ", "BK"];
                println!("Captured: {}", piece_names[captured]);
            }
        }
    } else {
        println!("No legal moves found!");
    }
    
    // Test 2: After 1.e4
    println!("\nTest 2: After 1.e4 (Black to move)");
    let mut after_e4 = BoardState::new();
    if let Some(_) = make_move(&mut after_e4, 52, 36) { // e2-e4
        let start = std::time::Instant::now();
        
        if let Some((from, to)) = find_best_move(&after_e4, 6) {
            let elapsed = start.elapsed();
            println!("Best move found: {} -> {}", 
                square_to_coordinates(from), 
                square_to_coordinates(to));
            println!("Search time: {:?}", elapsed);
            
            let move_names = ["e2-e4", "Nf3", "Nc3", "Bc4", "Bb5", "d3", "f4", "g3"];
            println!("Common opening moves at this position:");
            println!("- e2-e4 (already played)");
            println!("- Ng1-f3 (Knight to f3)");
            println!("- Bf1-c4 (Bishop to c4)");
            println!("- Bf1-b5 (Spanish/Ruy Lopez)");
        }
    }
    
    // Test 3: Specific tactical position (Scholar's mate position)
    println!("\nTest 3: Tactical Position (White to move and mate in 2)");
    let mut scholars_mate = BoardState::new();
    // Clear pieces for scholar's mate setup
    scholars_mate.bitboards = create_board();
    // Remove some pieces to create the position
    clear_bit(&mut scholars_mate.bitboards[WP], 52); // Remove e2 pawn
    clear_bit(&mut scholars_mate.bitboards[BP], 12); // Remove e7 pawn
    clear_bit(&mut scholars_mate.bitboards[BP], 11); // Remove d7 pawn
    clear_bit(&mut scholars_mate.bitboards[BP], 10); // Remove c7 pawn
    
    // Place pieces for scholar's mate
    set_bit(&mut scholars_mate.bitboards[WQ], 37); // Queen on d5
    set_bit(&mut scholars_mate.bitboards[WB], 45); // Bishop on f3
    
    println!("Position: White Queen d5, Bishop f3, Black King e8, pawns on f7, g7, h7");
    print_board(&scholars_mate);
    
    let start = std::time::Instant::now();
    if let Some((from, to)) = find_best_move(&scholars_mate, 6) {
        let elapsed = start.elapsed();
        println!("Best move found: {} -> {}", 
            square_to_coordinates(from), 
            square_to_coordinates(to));
        println!("Search time: {:?}", elapsed);
        
        // Check if it's Qxf7# (queen takes f7 mate)
        if from == 37 && to == 13 { // d5 to f7
            println!("✓ Found scholar's mate! Qxf7#");
        } else {
            println!("Expected Qxf7# (queen takes f7 mate)");
        }
    }
    
    // Test 4: Compare search algorithms
    println!("\nTest 4: Algorithm Comparison at Depth 4");
    let start_basic = std::time::Instant::now();
    let basic_move = find_best_move(&board_state, 4);
    let basic_time = start_basic.elapsed();
    
    let start_optimized = std::time::Instant::now();
    let optimized_move = find_best_move_iterative_deepening_optimized(&board_state, 4, 10000);
    let optimized_time = start_optimized.elapsed();
    
    println!("Basic minimax (depth 4): {:?} in {:?}", basic_move, basic_time);
    println!("Optimized (depth 4): {:?} in {:?}", optimized_move, optimized_time);
    
    if basic_move == optimized_move {
        println!("✓ Both algorithms agree on best move!");
    } else {
        println!("⚠ Algorithms disagree on best move");
    }
    
    // Test 5: Performance metrics
    println!("\nTest 5: Performance Metrics");
    test_search_performance(6);
}

fn test_search_performance(max_depth: u8) {
    let board_state = BoardState::new();
    
    for depth in 1..=max_depth {
        println!("\nSearching at depth {}...", depth);
        let start = std::time::Instant::now();
        
        if let Some((from, to)) = find_best_move(&board_state, depth) {
            let elapsed = start.elapsed();
            let nodes = estimate_nodes_searched(&board_state, depth);
            let nps = (nodes as f64 / elapsed.as_secs_f64()) as u64;
            
            println!("  Best move: {} -> {}", 
                square_to_coordinates(from), 
                square_to_coordinates(to));
            println!("  Time: {:?}", elapsed);
            println!("  Estimated nodes: {}", nodes);
            println!("  Estimated NPS: {}/sec", nps);
            
            if depth > 1 {
                let prev_start = std::time::Instant::now();
                let _ = find_best_move(&board_state, depth - 1);
                let prev_elapsed = prev_start.elapsed();
                
                if prev_elapsed.as_micros() > 0 {
                    let branching_factor = elapsed.as_micros() as f64 / prev_elapsed.as_micros() as f64;
                    println!("  Branching factor: {:.2}", branching_factor);
                }
            }
        }
    }
}

// Simple node estimation (for demonstration)
fn estimate_nodes_searched(board_state: &BoardState, depth: u8) -> u64 {
    // Very rough estimation based on average branching factor
    const AVERAGE_BRANCHING: f64 = 35.0; // Typical chess branching factor
    
    let mut total_nodes = 0.0;
    for d in 0..depth {
        total_nodes += AVERAGE_BRANCHING.powf(d as f64);
    }
    
    total_nodes as u64
}

// Also add a function to test move generation at depth 6
fn test_move_generation_depth_6() {
    println!("\n=== Testing Move Generation Tree at Depth 6 ===");
    
    let board_state = BoardState::new();
    
    // Count moves at each depth
    let mut total_positions = 0u64;
    let mut depth_counts = vec![0u64; 7]; // Depth 0 to 6
    
    // Recursive function to count positions
    fn count_positions(state: &BoardState, depth: u8, max_depth: u8, counts: &mut Vec<u64>) -> u64 {
        if depth == max_depth {
            return 1;
        }
        
        let moves = generate_moves(state.bitboards, state.white_to_move, state);
        let legal_moves: Vec<(u8, u8)> = moves.into_iter()
            .filter(|&(from, to)| {
                let mut temp_state = *state;
                make_move(&mut temp_state, from, to).is_some()
            })
            .collect();
        
        let mut positions = 0;
        for &(from, to) in &legal_moves {
            let mut next_state = *state;
            if make_move(&mut next_state, from, to).is_some() {
                positions += count_positions(&next_state, depth + 1, max_depth, counts);
            }
        }
        
        counts[depth as usize] += positions;
        positions
    }
    
    let start = std::time::Instant::now();
    total_positions = count_positions(&board_state, 0, 6, &mut depth_counts);
    let elapsed = start.elapsed();
    
    println!("Total positions at depth 6: {}", total_positions);
    println!("Time to count: {:?}", elapsed);
    
    // Print counts per depth
    for depth in 0..=6 {
        println!("  Depth {}: {} positions", depth, depth_counts[depth]);
    }
    
    // Compare with known chess statistics
    println!("\nChess statistics comparison:");
    println!("- Initial position has ~20 legal moves");
    println!("- Positions at depth 6: ~9,000,000 (typical)");
    println!("- Your engine found: {}", total_positions);
}

fn play_game() {
    println!("\n=== Chess Engine - Simple Game ===");
    
    let mut board_state = BoardState::new();
    let mut game_over = false;
    
    while !game_over {
        print_board(&board_state);
        
        if board_state.white_to_move {
            println!("White to move.");
            
            // Let the engine play for white
            if let Some((from, to)) = find_best_move(&board_state, 2) {
                println!("Engine plays: {} -> {}", 
                    square_to_coordinates(from), 
                    square_to_coordinates(to));
                
                make_move(&mut board_state, from, to);
            } else {
                println!("No legal moves for white!");
                game_over = true;
            }
        } else {
            println!("Black to move.");
            
            // Let the engine play for black
            if let Some((from, to)) = find_best_move(&board_state, 2) {
                println!("Engine plays: {} -> {}", 
                    square_to_coordinates(from), 
                    square_to_coordinates(to));
                
                make_move(&mut board_state, from, to);
            } else {
                println!("No legal moves for black!");
                game_over = true;
            }
        }
        
        // Check for checkmate
        if board_state.white_king_in_check && board_state.white_to_move {
            println!("White is in check!");
        }
        if board_state.black_king_in_check && !board_state.white_to_move {
            println!("Black is in check!");
        }
        
        // Simple game end condition - just play 10 moves each
        // In a real game, you'd check for checkmate/stalemate
        static mut MOVE_COUNT: u32 = 0;
        unsafe {
            MOVE_COUNT += 1;
            if MOVE_COUNT >= 20 {
                println!("Game over after 20 moves.");
                game_over = true;
            }
        }
    }
    
    println!("\nFinal position:");
    print_board(&board_state);
    println!("Final evaluation: {}", evaluate_board_advanced(&board_state));
}

fn benchmark_search() {
    println!("\n=== Search Benchmark ===");
    
    let positions = [
        ("Initial Position", BoardState::new()),
        ("Italian Game", create_italian_position()),
        ("Sicilian Defense", create_sicilian_position()),
    ];
    
    for (name, position) in positions.iter() {
        println!("\n{}:", name);
        
        for depth in [3, 4, 5] {
            let start = std::time::Instant::now();
            if let Some((from, to)) = find_best_move(&position, depth) {
                let elapsed = start.elapsed();
                println!("  Depth {}: {} -> {} in {:?}", 
                    depth,
                    square_to_coordinates(from),
                    square_to_coordinates(to),
                    elapsed);
            }
        }
    }
}

fn create_italian_position() -> BoardState {
    let mut board = BoardState::new();
    
    // Make moves to reach Italian Game: 1.e4 e5 2.Nf3 Nc6 3.Bc4
    let _ = make_move(&mut board, 52, 36); // e2-e4
    let _ = make_move(&mut board, 12, 28); // e7-e5
    let _ = make_move(&mut board, 57, 42); // Ng1-f3
    let _ = make_move(&mut board, 1, 16); // Nb8-c6
    let _ = make_move(&mut board, 58, 44); // Bf1-c4
    
    board
}

fn create_sicilian_position() -> BoardState {
    let mut board = BoardState::new();
    
    // Make moves to reach Sicilian Defense: 1.e4 c5
    let _ = make_move(&mut board, 52, 36); // e2-e4
    let _ = make_move(&mut board, 10, 26); // c7-c5
    
    board
}

//END OF TESTS--------------------------------------------------------------------------------------------


//ZOBRIST HASH--------------------------------------------------------------------------------------------

static ZOBRIST_TABLES: OnceLock<ZobristTables> = OnceLock::new();

struct ZobristTables {
    piece_square: [[u64; 64]; 12],      // 12 pieces × 64 squares
    black_to_move: u64,                  // When black is to move
    castling_rights: [u64; 4],           // 4 castling rights
    en_passant_file: [u64; 8],           // 8 possible en passant files
}

impl ZobristTables {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut piece_square = [[0u64; 64]; 12];
        let mut castling_rights = [0u64; 4];
        let mut en_passant_file = [0u64; 8];
        
        for piece in 0..12 {
            for square in 0..64 {
                piece_square[piece][square] = rng.r#gen::<u64>();
            }
        }
        
        let black_to_move = rng.r#gen::<u64>();
        
        for i in 0..4 {
            castling_rights[i] = rng.r#gen::<u64>();
        }
        
        for i in 0..8 {
            en_passant_file[i] = rng.r#gen::<u64>();
        }
        
        Self {
            piece_square,
            black_to_move,
            castling_rights,
            en_passant_file,
        }
    }
    
    fn get() -> &'static Self {
        ZOBRIST_TABLES.get_or_init(|| Self::new())
    }
}

fn compute_board_hash(board: &BoardState) -> u64 {
    let tables = ZobristTables::get();
    let mut hash = 0;
    
    // XOR in pieces on squares
    for (piece_index, &bitboard) in board.bitboards.iter().enumerate() {
        let mut bb = bitboard;
        while bb != 0 {
            let square = get_lsb(bb).unwrap();
            clear_bit(&mut bb, square);
            hash ^= tables.piece_square[piece_index][square as usize];
        }
    }
    
    // XOR in side to move
    if !board.white_to_move {
        hash ^= tables.black_to_move;
    }
    
    // XOR in castling rights
    if board.white_kingside_castle {
        hash ^= tables.castling_rights[0];
    }
    if board.white_queenside_castle {
        hash ^= tables.castling_rights[1];
    }
    if board.black_kingside_castle {
        hash ^= tables.castling_rights[2];
    }
    if board.black_queenside_castle {
        hash ^= tables.castling_rights[3];
    }
    
    // XOR in en passant target (if any)
    if let Some(ep_square) = board.en_passant_target {
        let file = ep_square % 8;
        hash ^= tables.en_passant_file[file as usize];
    }
    
    hash
}

//END OF ZOBRIST HASH-------------------------------------------------------------------------------------


//MINIMAX SEARCH------------------------------------------------------------------------------------------
static DEBUG: bool = false;


fn find_best_move(board_state: &BoardState, depth: u8) -> Option<(u8, u8)> {
    if DEBUG {
        println!("\n=== Starting search at depth {} ===", depth);
        println!("Position evaluation: {}", evaluate_board_advanced(board_state));
    }
    
    let mut search_state = SearchState {
        board: *board_state,
        move_history: Vec::new(),
    };
    
    let mut best_move = None;
    let mut best_value = if board_state.white_to_move { i32::MIN } else { i32::MAX };
    
    // Generate all legal moves
    let moves = generate_moves(search_state.board.bitboards, search_state.board.white_to_move, &search_state.board);
    
    if DEBUG {
        println!("[Root] Raw moves generated: {}", moves.len());
    }
    
    // Filter for legal moves (those that don't leave king in check)
    let legal_moves: Vec<(u8, u8)> = moves.into_iter()
        .filter(|&(from, to)| {
            let mut temp_state = SearchState {
                board: search_state.board,
                move_history: Vec::new(),
            };
            
            // Try to make the move
            if temp_state.make_move(from, to) {
                // Check if the move leaves our own king in check
                let our_king_in_check = if search_state.board.white_to_move {
                    temp_state.board.white_king_in_check
                } else {
                    temp_state.board.black_king_in_check
                };
                
                // The move is legal if it doesn't leave our king in check
                !our_king_in_check
            } else {
                false
            }
        })
        .collect();
    
    if DEBUG {
        println!("[Root] Legal moves: {}", legal_moves.len());
        println!("[Root] Moves list:");
        for &(from, to) in &legal_moves {
            println!("  {} -> {}", square_to_coordinates(from), square_to_coordinates(to));
        }
    }
    
    if legal_moves.is_empty() {
        if DEBUG {
            println!("[Root] No legal moves found!");
        }
        return None;
    }
    
    // Order moves at root
    let ordered_moves = order_moves(&search_state.board, &legal_moves);
    
    if DEBUG {
        println!("[Root] Ordered moves (top 10):");
        for (i, (score, from, to)) in ordered_moves.iter().take(10).enumerate() {
            println!("  {}. {} -> {} (score: {})", 
                i + 1, square_to_coordinates(*from), square_to_coordinates(*to), score);
        }
    }
    
    let mut move_counter = 0;
    for (_, from, to) in &ordered_moves {
        move_counter += 1;
        
        if DEBUG {
            println!("\n[Root] Evaluating move {} of {}: {} -> {}", 
                move_counter, ordered_moves.len(), 
                square_to_coordinates(*from), square_to_coordinates(*to));
        }
        
        // Make the move
        search_state.make_move(*from, *to);
        
        // Evaluate the position
        let value = minimax(&mut search_state, depth as i32 - 1, i32::MIN, i32::MAX, !board_state.white_to_move);
        
        // Unmake the move
        search_state.unmake_move();
        
        if DEBUG {
            println!("[Root] Move {} -> {} evaluation: {}", 
                square_to_coordinates(*from), square_to_coordinates(*to), value);
        }
        
        if board_state.white_to_move {
            // White wants to maximize the score
            if value > best_value || best_move.is_none() {
                if DEBUG && best_move.is_some() {
                    println!("[Root] New best move! Old: {}, New: {}", best_value, value);
                }
                best_value = value;
                best_move = Some((*from, *to));
            }
        } else {
            // Black wants to minimize the score
            if value < best_value || best_move.is_none() {
                if DEBUG && best_move.is_some() {
                    println!("[Root] New best move! Old: {}, New: {}", best_value, value);
                }
                best_value = value;
                best_move = Some((*from, *to));
            }
        }
    }
    
    if DEBUG {
        if let Some((from, to)) = best_move {
            println!("\n[Root] Best move found: {} -> {} with evaluation: {}", 
                square_to_coordinates(from), square_to_coordinates(to), best_value);
        }
        println!("=== End search at depth {} ===\n", depth);
    }
    
    best_move
}

fn find_best_move_iterative_deepening_optimized(
    board_state: &BoardState, 
    max_depth: u8, 
    _time_limit_ms: u64  // Prefix with underscore since it's unused
) -> Option<(u8, u8)> {
    let mut tt = TranspositionTable::new(16);
    let mut best_move = None;
    let mut alpha = i32::MIN + 1;
    let mut beta = i32::MAX - 1;
    
    for depth in 1..=max_depth {
        let window = 50;
        
        // Fix: Add the missing ply parameter (0 for root)
        let score = negamax_root(
            board_state,
            depth as i32,
            alpha,
            beta,
            0,  // ply parameter for root
            &mut tt,
        );
        
        if score <= alpha {
            alpha = i32::MIN + 1;
            beta = score + window;
        } else if score >= beta {
            alpha = score - window;
            beta = i32::MAX - 1;
        } else {
            alpha = score - window;
            beta = score + window;
            best_move = get_best_move_from_tt(board_state, &tt);
        }
    }
    
    best_move
}

/// Minimax algorithm with alpha-beta pruning
fn minimax(
    search_state: &mut SearchState,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    // Base case: reached maximum depth or terminal position
    if depth == 0 {
        if DEBUG && depth == 0 {
            println!("  [Leaf node reached, evaluating position]");
        }
        return evaluate_board_advanced(&search_state.board);
    }
    
    // Generate legal moves for the current position
    let moves = generate_moves(
        search_state.board.bitboards,
        search_state.board.white_to_move,
        &search_state.board,
    );
    
    if DEBUG {
        println!("  [Depth {}] Raw moves generated: {}", depth, moves.len());
    }
    
    // Filter for legal moves (those that don't leave king in check)
    let legal_moves: Vec<(u8, u8)> = moves.into_iter()
        .filter(|&(from, to)| {
            let mut temp_state = SearchState {
                board: search_state.board,
                move_history: Vec::new(),
            };
            
            if temp_state.make_move(from, to) {
                let our_king_in_check = if search_state.board.white_to_move {
                    temp_state.board.white_king_in_check
                } else {
                    temp_state.board.black_king_in_check
                };
                !our_king_in_check
            } else {
                false
            }
        })
        .collect();
    
    if DEBUG {
        println!("  [Depth {}] Legal moves after check: {}", depth, legal_moves.len());
        
        // Print first few moves for debugging
        if depth >= 3 && legal_moves.len() > 0 {
            println!("    First 5 moves:");
            for &(from, to) in legal_moves.iter().take(5) {
                println!("      {} -> {}", square_to_coordinates(from), square_to_coordinates(to));
            }
            if legal_moves.len() > 5 {
                println!("      ... and {} more", legal_moves.len() - 5);
            }
        }
    }
    
    let mut moves_with_scores: Vec<(i32, u8, u8)> = order_moves(&search_state.board, &legal_moves);
    
    if DEBUG && depth >= 3 && moves_with_scores.len() > 0 {
        println!("    After ordering - top 5 moves:");
        for (score, from, to) in moves_with_scores.iter().take(5) {
            println!("      {} -> {} (score: {})", 
                square_to_coordinates(*from), 
                square_to_coordinates(*to), 
                score);
        }
    }
    
    // Check for terminal positions
    if legal_moves.is_empty() {
        // No legal moves - checkmate or stalemate
        let in_check = if search_state.board.white_to_move {
            search_state.board.white_king_in_check
        } else {
            search_state.board.black_king_in_check
        };
        
        if in_check {
            // Checkmate - return very bad score for the player who is checkmated
            return if search_state.board.white_to_move {
                // White is checkmated - very bad for white
                i32::MIN + 1
            } else {
                // Black is checkmated - very good for white
                i32::MAX - 1
            };
        } else {
            // Stalemate - draw
            return 0;
        }
    }
    
    if maximizing_player {
        // Maximizing player (white in this context)
        let mut max_eval = i32::MIN;
        let mut moves_examined = 0;
        let mut pruned_moves = 0;
        
        for (_, from, to) in &moves_with_scores {
            moves_examined += 1;
            
            // Make the move
            search_state.make_move(*from, *to);
            
            if DEBUG && depth >= 3 {
                println!("    [Depth {}] Examining move {}: {} -> {}", 
                    depth, moves_examined, square_to_coordinates(*from), square_to_coordinates(*to));
            }
            
            // Recursively evaluate
            let eval = minimax(search_state, depth - 1, alpha, beta, false);
            
            // Unmake the move
            search_state.unmake_move();
            
            // Update max evaluation
            max_eval = max_eval.max(eval);
            
            if DEBUG && depth >= 3 {
                println!("    [Depth {}] Move {} -> {} evaluation: {} (alpha: {}, beta: {})", 
                    depth, square_to_coordinates(*from), square_to_coordinates(*to), eval, alpha, beta);
            }
            
            // Alpha-beta pruning
            alpha = alpha.max(eval);
            if beta <= alpha {
                if DEBUG && depth >= 3 {
                    println!("    [Depth {}] BETA CUTOFF after move {} -> {} (beta: {}, alpha: {})", 
                        depth, square_to_coordinates(*from), square_to_coordinates(*to), beta, alpha);
                }
                pruned_moves = moves_with_scores.len() - moves_examined;
                break; // Beta cutoff
            }
        }
        
        if DEBUG && depth >= 3 {
            println!("  [Depth {}] Examined {} moves, pruned {} moves, max_eval: {}", 
                depth, moves_examined, pruned_moves, max_eval);
        }
        
        max_eval
    } else {
        // Minimizing player (black in this context)
        let mut min_eval = i32::MAX;
        let mut moves_examined = 0;
        let mut pruned_moves = 0;
        
        for (_, from, to) in &moves_with_scores {
            moves_examined += 1;
            
            // Make the move
            search_state.make_move(*from, *to);
            
            if DEBUG && depth >= 3 {
                println!("    [Depth {}] Examining move {}: {} -> {}", 
                    depth, moves_examined, square_to_coordinates(*from), square_to_coordinates(*to));
            }
            
            // Recursively evaluate
            let eval = minimax(search_state, depth - 1, alpha, beta, true);
            
            // Unmake the move
            search_state.unmake_move();
            
            // Update min evaluation
            min_eval = min_eval.min(eval);
            
            if DEBUG && depth >= 3 {
                println!("    [Depth {}] Move {} -> {} evaluation: {} (alpha: {}, beta: {})", 
                    depth, square_to_coordinates(*from), square_to_coordinates(*to), eval, alpha, beta);
            }
            
            // Alpha-beta pruning
            beta = beta.min(eval);
            if beta <= alpha {
                if DEBUG && depth >= 3 {
                    println!("    [Depth {}] ALPHA CUTOFF after move {} -> {} (beta: {}, alpha: {})", 
                        depth, square_to_coordinates(*from), square_to_coordinates(*to), beta, alpha);
                }
                pruned_moves = moves_with_scores.len() - moves_examined;
                break; // Alpha cutoff
            }
        }
        
        if DEBUG && depth >= 3 {
            println!("  [Depth {}] Examined {} moves, pruned {} moves, min_eval: {}", 
                depth, moves_examined, pruned_moves, min_eval);
        }
        
        min_eval
    }
}



/// Quiescence search - extends search in capture positions to avoid horizon effect
fn quiescence_search(
    search_state: &mut SearchState,
    mut alpha: i32,
    beta: i32,
    maximizing_player: bool,
) -> i32 {
    // First, get a stand-pat evaluation
    let stand_pat = evaluate_board_advanced(&search_state.board);
    
    if maximizing_player {
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        if beta > stand_pat {
            alpha = stand_pat;
        }
    }
    
    // Generate only capture moves
    let capture_moves = generate_capture_moves(search_state);
    
    for &(from, to) in &capture_moves {
        // Make the move
        search_state.make_move(from, to);
        
        // Recursively evaluate captures
        let score = -quiescence_search(search_state, -beta, -alpha, !maximizing_player);
        
        // Unmake the move
        search_state.unmake_move();
        
        if maximizing_player {
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        } else {
            if score <= alpha {
                return alpha;
            }
            if score < beta {
                alpha = score;
            }
        }
    }
    
    alpha
}

/// Helper function to generate only capture moves
fn generate_capture_moves(search_state: &SearchState) -> Vec<(u8, u8)> {
    let mut capture_moves = Vec::new();
    
    // Get all moves
    let all_moves = generate_moves(
        search_state.board.bitboards,
        search_state.board.white_to_move,
        &search_state.board,
    );
    
    // Filter for captures
    for &(from, to) in &all_moves {
        // Check if this move captures a piece
        let enemy_start = if search_state.board.white_to_move { 6 } else { 0 };
        let enemy_end = if search_state.board.white_to_move { 12 } else { 6 };
        
        let mut is_capture = false;
        for piece_type in enemy_start..enemy_end {
            if get_bit(search_state.board.bitboards[piece_type], to) {
                is_capture = true;
                break;
            }
        }
        
        if is_capture {
            // Also check that the move is legal (doesn't leave king in check)
            let mut temp_state = SearchState {
                board: search_state.board,
                move_history: Vec::new(),
            };
            
            if temp_state.make_move(from, to) {
                let our_king_in_check = if search_state.board.white_to_move {
                    temp_state.board.white_king_in_check
                } else {
                    temp_state.board.black_king_in_check
                };
                
                if !our_king_in_check {
                    capture_moves.push((from, to));
                }
            }
        }
    }
    
    capture_moves
}


fn minimax_with_quiescence(
    search_state: &mut SearchState,
    depth: i32,
    alpha: i32,
    beta: i32,
    maximizing_player: bool,
) -> i32 {
    if depth == 0 {
        // Use quiescence search instead of static evaluation
        return quiescence_search(search_state, alpha, beta, search_state.board.white_to_move);
    }
    
    // ... rest of the minimax function remains the same as above ...
    // You can copy the rest of the minimax function here and modify the base case
    
    // For now, let's just call the regular minimax
    minimax(search_state, depth, alpha, beta, maximizing_player)
}

fn quiescence_search_enhanced(
    search_state: &mut SearchState,
    mut alpha: i32,
    beta: i32,
    maximizing_player: bool,
) -> i32 {
    let stand_pat = evaluate_board_fast(&search_state.board);
    
    if maximizing_player {
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }
    } else {
        if stand_pat <= alpha {
            return alpha;
        }
        if beta > stand_pat {
            alpha = stand_pat;
        }
    }
    
    // Generate only capture moves (and checks)
    let all_moves = generate_moves(
        search_state.board.bitboards,
        search_state.board.white_to_move,
        &search_state.board,
    );
    
    // Filter for captures and checks
    let capture_moves: Vec<(u8, u8)> = all_moves.into_iter()
        .filter(|&(from, to)| {
            // Is it a capture?
            let enemy_start = if search_state.board.white_to_move { 6 } else { 0 };
            let enemy_end = if search_state.board.white_to_move { 12 } else { 6 };
            
            let mut is_capture = false;
            for piece_type in enemy_start..enemy_end {
                if get_bit(search_state.board.bitboards[piece_type], to) {
                    is_capture = true;
                    break;
                }
            }
            
            // Also include moves that give check
            if !is_capture {
                // Check if move gives check
                let mut temp_state = search_state.board;
                if make_move(&mut temp_state, from, to).is_some() {
                    let opponent_in_check = if temp_state.white_to_move {
                        temp_state.black_king_in_check
                    } else {
                        temp_state.white_king_in_check
                    };
                    return opponent_in_check;
                }
                return false;
            }
            
            is_capture
        })
        .filter(|&(from, to)| is_move_legal_fast(&search_state.board, from, to))
        .collect();
    
    // Order captures by MVV-LVA
    let mut scored_captures: Vec<(i32, u8, u8)> = capture_moves.iter()
        .map(|&(from, to)| {
            let mut score = 0;
            if let Some(captured_piece) = get_piece_at_square(&search_state.board.bitboards, to) {
                let victim_value = get_piece_value(captured_piece);
                let aggressor_value = if let Some(aggressor) = get_piece_at_square(&search_state.board.bitboards, from) {
                    get_piece_value(aggressor)
                } else { 0 };
                score = 10000 + victim_value * 10 - aggressor_value;
            }
            (score, from, to)
        })
        .collect();
    
    scored_captures.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    
    for &(_, from, to) in &scored_captures {
        search_state.make_move(from, to);
    let score = -quiescence_search_enhanced(search_state, -beta, -alpha, !maximizing_player);
    search_state.unmake_move();
        
        if maximizing_player {
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        } else {
            if score <= alpha {
                return alpha;
            }
            if score < beta {
                alpha = score;
            }
        }
    }
    
    alpha
}

fn negamax_enhanced(
    search_state: &mut SearchState,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    ply: usize,
    tt: &mut TranspositionTable,
) -> i32  {
    if depth == 0 {
        return quiescence_search_enhanced(search_state, alpha, beta, search_state.board.white_to_move);
    }
    
    let original_alpha = alpha;
    let hash = compute_board_hash(&search_state.board);
    
    // TT lookup with improved probing
    if let Some((score, best_move)) = tt.probe(hash, depth, alpha, beta) {
        // Store as killer move if it caused a cutoff
        if (score >= beta) && (best_move.0 != best_move.1) { // Not a null move
            add_killer_move(ply, best_move.0, best_move.1);
        }
        return score;
    }
    
    // Null move pruning (optional but effective)
    if depth >= 3 && !search_state.board.is_current_king_in_check() {
    // Try a null move
    search_state.board.white_to_move = !search_state.board.white_to_move;
    // Fix: Add ply parameter
    let null_score = -negamax_root(
        &search_state.board,
        depth - 1 - 2,
        -beta,
        -beta + 1,
        ply + 1,  // Add ply parameter
        tt,
    );
    search_state.board.white_to_move = !search_state.board.white_to_move;
    
    if null_score >= beta {
        return beta;
    }
}

    
    let moves = generate_moves(
        search_state.board.bitboards,
        search_state.board.white_to_move,
        &search_state.board,
    );
    
    // Filter for legal moves
    let legal_moves: Vec<(u8, u8)> = moves.into_iter()
        .filter(|&(from, to)| is_move_legal_fast(&search_state.board, from, to))
        .collect();
    
    if legal_moves.is_empty() {
        // Terminal position
        let in_check = search_state.board.is_current_king_in_check();
        return if in_check {
            // Checkmate
            i32::MIN + ply as i32 // Prefer checkmates earlier
        } else {
            0 // Stalemate
        };
    }
    
    let ordered_moves = order_moves(&search_state.board, &legal_moves);
    
    let mut best_score = i32::MIN + 1;
    let mut best_move_found = (0, 0);
    let mut moves_searched = 0;
    
    for &(_, from, to) in &ordered_moves {
        search_state.make_move(from, to);
        let mut score;
    
    // Late Move Reduction (LMR)
    if moves_searched >= 4 && depth >= 3 && 
       !search_state.board.is_current_king_in_check() &&
       get_piece_at_square(&search_state.board.bitboards, to).is_none() {
        
        score = -negamax_enhanced(search_state, depth - 2, -alpha - 1, -alpha, ply + 1, tt);
        if score > alpha {
            // Research with full depth
            score = -negamax_enhanced(search_state, depth - 1, -beta, -alpha, ply + 1, tt);
        }
    } else if moves_searched == 0 {
        // Full window search for first move
        score = -negamax_enhanced(search_state, depth - 1, -beta, -alpha, ply + 1, tt);
    } else {
        // Null window search for other moves
        score = -negamax_enhanced(search_state, depth - 1, -alpha - 1, -alpha, ply + 1, tt);
        if score > alpha && score < beta {
            // Research with full window
            score = -negamax_enhanced(search_state, depth - 1, -beta, -alpha, ply + 1, tt);
        }
    }
        
        search_state.unmake_move();
        
        if score >= beta {
            // Beta cutoff - store as killer move
            if get_piece_at_square(&search_state.board.bitboards, to).is_none() { // Not a capture
                add_killer_move(ply, from, to);
                update_history_score(from, to, depth);
            }
            
            // Store in TT
            tt.store(hash, depth, beta, 2, (from, to)); // Lower bound
            return beta;
        }
        
        if score > best_score {
            best_score = score;
            best_move_found = (from, to);
            if score > alpha {
                alpha = score;
            }
        }
        
        moves_searched += 1;
    }
    
    // Determine TT flag
    let flag = if best_score <= original_alpha {
        1 // Upper bound
    } else if best_score >= beta {
        2 // Lower bound
    } else {
        0 // Exact
    };
    
    tt.store(hash, depth, best_score, flag, best_move_found);
    best_score
}

fn negamax_root(
    board_state: &BoardState,
    depth: i32,
    alpha: i32,
    beta: i32,
    ply: usize,  // Add this missing parameter
    tt: &mut TranspositionTable,
) -> i32 {
    let mut search_state = SearchState {
        board: *board_state,
        move_history: Vec::new(),
    };
    // Fix: Add the missing ply parameter (0 for root)
    negamax_enhanced(&mut search_state, depth, alpha, beta, ply, tt)
}


fn is_move_legal_fast(board: &BoardState, from: u8, to: u8) -> bool {
    let mut board_copy = *board;
    if make_move(&mut board_copy, from, to).is_none() {
        return false;
    }
    
    // Fix: Use the dot operator instead of dereferencing
    let king_square = if board.white_to_move {  // Fixed here
        get_lsb(board_copy.bitboards[WK])
    } else {
        get_lsb(board_copy.bitboards[BK])
    };
    
    if let Some(king_sq) = king_square {
        let attacks = complete_attacks_bitboard(&board_copy.bitboards, !board.white_to_move);
        return !get_bit(attacks, king_sq);
    }
    
    false
}


//END OF MINMAX SEARCH------------------------------------------------------------------------------------


//MOVE ORDERING-------------------------------------------------------------------------------------------

fn order_moves(board: &BoardState, moves: &[(u8, u8)]) -> Vec<(i32, u8, u8)> {
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

use std::cell::RefCell;
use std::thread_local;

// Killer moves are typically thread-local for performance
thread_local! {
    static KILLER_MOVES: RefCell<[[Option<(u8, u8)>; 2]; 64]> = RefCell::new([[None; 2]; 64]);
}

fn is_killer_move(from: u8, to: u8, _white_to_move: bool) -> bool {
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

fn add_killer_move(ply: usize, from: u8, to: u8) {
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

static HISTORY_TABLE: OnceLock<ThreadSafeHistoryTable> = OnceLock::new();

fn init_history_table() {
    let _ = HISTORY_TABLE.set(ThreadSafeHistoryTable::new());
}

fn get_history_score(from: u8, to: u8, _white_to_move: bool) -> i32 {
    HISTORY_TABLE.get().unwrap().get(from, to)
}

fn update_history_score(from: u8, to: u8, depth: i32) {
    HISTORY_TABLE.get().unwrap().update(from, to, depth);
}

//END OF MOVE ORDERING------------------------------------------------------------------------------------


//TRANSPOSITION TABLES------------------------------------------------------------------------------------

static TRANSPOSITION_TABLE: OnceLock<Arc<RwLock<TranspositionTable>>> = OnceLock::new();

#[derive(Clone, Copy)]
struct TTEntry {
    hash: u64,
    depth: i32,
    score: i32,
    flag: u8, // 0=exact, 1=upper bound, 2=lower bound
    best_move: (u8, u8),
}

struct TranspositionTable {
    entries: Vec<Option<TTEntry>>,
    size: usize,
}

impl TranspositionTable {
    fn new(size_mb: usize) -> Self {
        // Calculate number of entries based on memory size
        // Each entry is about 24 bytes (u64 + i32 + i32 + u8 + (u8, u8) padding)
        let entry_size = std::mem::size_of::<TTEntry>();
        let size = (size_mb * 1024 * 1024) / entry_size;
        
        // Make sure we have at least some minimum size
        let size = size.max(1024); // Minimum 1024 entries
        
        Self {
            entries: vec![None; size],
            size,
        }
    }
    
    fn store(&mut self, hash: u64, depth: i32, score: i32, flag: u8, best_move: (u8, u8)) {
        let index = (hash as usize) % self.size;
        
        // Replacement strategy: always replace if new entry is from deeper search
        if let Some(existing) = &self.entries[index] {
            if existing.depth > depth && existing.hash == hash {
                // Keep the existing deeper entry
                return;
            }
        }
        
        self.entries[index] = Some(TTEntry { 
            hash, 
            depth, 
            score, 
            flag, 
            best_move 
        });
    }
    
    fn probe(&self, hash: u64, depth: i32, alpha: i32, beta: i32) -> Option<(i32, (u8, u8))> {
        let index = (hash as usize) % self.size;
        
        if let Some(entry) = &self.entries[index] {
            // Check if this is the right entry (not a hash collision)
            if entry.hash == hash && entry.depth >= depth {
                match entry.flag {
                    0 => { // Exact score
                        return Some((entry.score, entry.best_move));
                    }
                    1 => { // Upper bound (score <= actual)
                        if entry.score <= alpha {
                            return Some((alpha, entry.best_move));
                        }
                    }
                    2 => { // Lower bound (score >= actual)
                        if entry.score >= beta {
                            return Some((beta, entry.best_move));
                        }
                    }
                    _ => {}
                }
            }
        }
        
        None
    }
    
    // Clear the entire transposition table
    fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = None;
        }
    }
    
    // Get statistics about the transposition table
    fn stats(&self) -> (usize, usize) {
        let total = self.size;
        let used = self.entries.iter().filter(|e| e.is_some()).count();
        (total, used)
    }
}

fn init_transposition_table(size_mb: usize) {
    let _ = TRANSPOSITION_TABLE.set(Arc::new(RwLock::new(TranspositionTable::new(size_mb))));
}


fn get_transposition_table() -> Arc<RwLock<TranspositionTable>> {
    TRANSPOSITION_TABLE
        .get_or_init(|| Arc::new(RwLock::new(TranspositionTable::new(16))))
        .clone()
}


fn get_tt_move(hash: u64) -> Option<(u8, u8)> {
    let tt = get_transposition_table();
    let tt_guard = tt.read().unwrap(); // Use read lock for shared access
    
    let index = (hash as usize) % tt_guard.size;
    
    if let Some(entry) = &tt_guard.entries[index] {
        if entry.hash == hash {
            return Some(entry.best_move);
        }
    }
    
    None
}


fn get_best_move_from_tt(board_state: &BoardState, tt: &TranspositionTable) -> Option<(u8, u8)> {
    let hash = compute_board_hash(board_state);
    // Look up in TT (simplified - in reality you'd need depth and bounds)
    let index = (hash as usize) % tt.size;
    if let Some(entry) = &tt.entries[index] {
        if entry.hash == hash {
            return Some(entry.best_move);
        }
    }
    None
}


struct ThreadSafeHistoryTable {
    table: Box<[AtomicI32; 64 * 64]>,
}

impl ThreadSafeHistoryTable {
    fn new() -> Self {
        let table = Box::new([const { AtomicI32::new(0) }; 64 * 64]);
        Self { table }
    }
    
    fn get(&self, from: u8, to: u8) -> i32 {
        let index = from as usize * 64 + to as usize;
        self.table[index].load(Ordering::Relaxed)
    }
    
    fn update(&self, from: u8, to: u8, depth: i32) {
        let index = from as usize * 64 + to as usize;
        let current = self.table[index].load(Ordering::Relaxed);
        let new_value = current + depth * depth;
        
        let capped_value = if new_value > 1_000_000 { new_value / 2 } else { new_value };
        self.table[index].store(capped_value, Ordering::Relaxed);
    }
}

//END OF TRANSPOSITION TABLES-----------------------------------------------------------------------------


//BOARD STATE---------------------------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct BoardState {
    bitboards: [u64; 12],
    white_to_move: bool,
    white_kingside_castle: bool,
    white_queenside_castle: bool,
    black_kingside_castle: bool,
    black_queenside_castle: bool,
    white_king_in_check: bool,
    black_king_in_check: bool,
    en_passant_target: Option<u8>,
}

impl BoardState {
    fn new() -> Self {
        let bitboards = create_board();
        let white_king_in_check = is_check(bitboards, true);
        let black_king_in_check = is_check(bitboards, false);
        
        Self {
            bitboards,
            white_to_move: true,
            white_kingside_castle: true,
            white_queenside_castle: true,
            black_kingside_castle: true,
            black_queenside_castle: true,
            white_king_in_check,
            black_king_in_check,
            en_passant_target: None,
        }
    }
    
    fn king_moved(&mut self, white: bool) {
        if white {
            self.white_kingside_castle = false;
            self.white_queenside_castle = false;
        } else {
            self.black_kingside_castle = false;
            self.black_queenside_castle = false;
        }
    }
    
    fn rook_moved(&mut self, square: u8, white: bool) {
        if white {
            if square == 56 { // a1
                self.white_queenside_castle = false;
            } else if square == 63 { // h1
                self.white_kingside_castle = false;
            }
        } else {
            if square == 0 { // a8
                self.black_queenside_castle = false;
            } else if square == 7 { // h8
                self.black_kingside_castle = false;
            }
        }
    }
    
    // Method to check if white king is in check
    fn is_white_king_in_check(&self) -> bool {
        self.white_king_in_check
    }
    
    // Method to check if black king is in check
    fn is_black_king_in_check(&self) -> bool {
        self.black_king_in_check
    }
    
    // Method to check if current side's king is in check
    fn is_current_king_in_check(&self) -> bool {
        if self.white_to_move {
            self.white_king_in_check
        } else {
            self.black_king_in_check
        }
    }
    
    // Update check status after a move
    fn update_check_status(&mut self) {
        self.white_king_in_check = is_check(self.bitboards, true);
        self.black_king_in_check = is_check(self.bitboards, false);
    }
}

//END OF BOARD STATE---------------------------------------------------------------------------------------

//UCI PROTOCOL-------------------------------------------------------------------------------------------

fn parse_fen(fen: &str) -> Option<BoardState> {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    if parts.len() < 1 {
        return None;
    }
    
    let board_str = parts[0];
    let mut bitboards = [0u64; 12];
    
    let mut rank = 0;
    let mut file = 0;
    
    for ch in board_str.chars() {
        if ch == '/' {
            rank += 1;
            file = 0;
            continue;
        }
        
        if ch.is_ascii_digit() {
            file += ch.to_digit(10)? as usize;
            continue;
        }
        
        if file >= 8 || rank >= 8 {
            return None;
        }
        
        let square = (rank * 8 + file) as u8;
        let piece_index = match ch {
            'P' => Some(WP),
            'N' => Some(WN),
            'B' => Some(WB),
            'R' => Some(WR),
            'Q' => Some(WQ),
            'K' => Some(WK),
            'p' => Some(BP),
            'n' => Some(BN),
            'b' => Some(BB),
            'r' => Some(BR),
            'q' => Some(BQ),
            'k' => Some(BK),
            _ => None,
        }?;
        
        set_bit(&mut bitboards[piece_index], square);
        file += 1;
    }
    
    let white_to_move = if parts.len() > 1 {
        parts[1] == "w"
    } else {
        true
    };
    
    let mut white_kingside_castle = false;
    let mut white_queenside_castle = false;
    let mut black_kingside_castle = false;
    let mut black_queenside_castle = false;
    
    if parts.len() > 2 && parts[2] != "-" {
        for ch in parts[2].chars() {
            match ch {
                'K' => white_kingside_castle = true,
                'Q' => white_queenside_castle = true,
                'k' => black_kingside_castle = true,
                'q' => black_queenside_castle = true,
                _ => {}
            }
        }
    }
    
    let en_passant_target = if parts.len() > 3 && parts[3] != "-" {
        coordinates_to_square(parts[3])
    } else {
        None
    };
    
    let mut board_state = BoardState {
        bitboards,
        white_to_move,
        white_kingside_castle,
        white_queenside_castle,
        black_kingside_castle,
        black_queenside_castle,
        white_king_in_check: false,
        black_king_in_check: false,
        en_passant_target,
    };
    
    board_state.update_check_status();
    
    Some(board_state)
}

struct UCISearchParams {
    depth: Option<u8>,
    movetime: Option<u64>, // milliseconds
    wtime: Option<u64>,    // milliseconds
    btime: Option<u64>,    // milliseconds
    winc: Option<u64>,     // milliseconds
    binc: Option<u64>,     // milliseconds
    movestogo: Option<u32>,
    infinite: bool,
}

impl UCISearchParams {
    fn new() -> Self {
        Self {
            depth: None,
            movetime: None,
            wtime: None,
            btime: None,
            winc: None,
            binc: None,
            movestogo: None,
            infinite: false,
        }
    }
    
    fn parse_go_command(cmd: &str) -> Self {
        let mut params = Self::new();
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        let mut i = 0;
        
        while i < parts.len() {
            match parts[i] {
                "depth" if i + 1 < parts.len() => {
                    if let Ok(d) = parts[i + 1].parse::<u8>() {
                        params.depth = Some(d);
                    }
                    i += 2;
                }
                "movetime" if i + 1 < parts.len() => {
                    if let Ok(mt) = parts[i + 1].parse::<u64>() {
                        params.movetime = Some(mt);
                    }
                    i += 2;
                }
                "wtime" if i + 1 < parts.len() => {
                    if let Ok(wt) = parts[i + 1].parse::<u64>() {
                        params.wtime = Some(wt);
                    }
                    i += 2;
                }
                "btime" if i + 1 < parts.len() => {
                    if let Ok(bt) = parts[i + 1].parse::<u64>() {
                        params.btime = Some(bt);
                    }
                    i += 2;
                }
                "winc" if i + 1 < parts.len() => {
                    if let Ok(wi) = parts[i + 1].parse::<u64>() {
                        params.winc = Some(wi);
                    }
                    i += 2;
                }
                "binc" if i + 1 < parts.len() => {
                    if let Ok(bi) = parts[i + 1].parse::<u64>() {
                        params.binc = Some(bi);
                    }
                    i += 2;
                }
                "movestogo" if i + 1 < parts.len() => {
                    if let Ok(mtg) = parts[i + 1].parse::<u32>() {
                        params.movestogo = Some(mtg);
                    }
                    i += 2;
                }
                "infinite" => {
                    params.infinite = true;
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
        
        params
    }
    
    fn calculate_time_limit(&self, white_to_move: bool) -> Option<Duration> {
        if self.infinite {
            return None;
        }
        
        if let Some(mt) = self.movetime {
            return Some(Duration::from_millis(mt));
        }
        
        let time_left = if white_to_move {
            self.wtime?
        } else {
            self.btime?
        };
        
        let increment = if white_to_move {
            self.winc.unwrap_or(0)
        } else {
            self.binc.unwrap_or(0)
        };
        
        let moves_to_go = self.movestogo.unwrap_or(20) as u64;
        
        // Simple time management: use time_left / moves_to_go + increment
        let time_per_move = time_left / moves_to_go.max(1) + increment;
        
        // Use 80% of calculated time to leave buffer
        Some(Duration::from_millis((time_per_move * 80) / 100))
    }
}

static SEARCH_STOP: AtomicBool = AtomicBool::new(false);

fn find_best_move_with_time(
    board_state: &BoardState,
    max_depth: u8,
    time_limit: Option<Duration>,
) -> Option<(u8, u8)> {
    SEARCH_STOP.store(false, Ordering::Relaxed);
    
    let start_time = Instant::now();
    let mut best_move = None;
    let mut best_score = if board_state.white_to_move { i32::MIN } else { i32::MAX };
    let mut nodes_searched = 0u64;
    
    // Output initial info to show engine is working
    println!("info depth 0");
    io::stdout().flush().ok();
    
    // Iterative deepening with time management
    for depth in 1..=max_depth {
        if SEARCH_STOP.load(Ordering::Relaxed) {
            break;
        }
        
        // Check time before starting new depth
        if let Some(limit) = time_limit {
            let elapsed = start_time.elapsed();
            if elapsed >= limit {
                break;
            }
            // Reserve some time for the next iteration
            let remaining = limit.saturating_sub(elapsed);
            if remaining < Duration::from_millis(50) {
                break; // Not enough time for another depth
            }
        }
        
        // Output info before starting depth search
        let time_ms = start_time.elapsed().as_millis() as u64;
        if time_ms > 0 {
            let nps = (nodes_searched * 1000) / time_ms.max(1);
            println!("info depth {} nodes {} time {} nps {}",
                depth - 1,
                nodes_searched,
                time_ms,
                nps
            );
            io::stdout().flush().ok();
        }
        
        if let Some(mv) = find_best_move(board_state, depth) {
            best_move = Some(mv);
            
            // Get evaluation after the move
            let mut test_state = *board_state;
            if make_move(&mut test_state, mv.0, mv.1).is_some() {
                best_score = evaluate_board_advanced(&test_state);
            }
            
            // Estimate nodes searched (rough approximation)
            nodes_searched = estimate_nodes_searched(board_state, depth);
            let time_ms = start_time.elapsed().as_millis() as u64;
            let nps = if time_ms > 0 { (nodes_searched * 1000) / time_ms.max(1) } else { 0 };
            
            // Build PV - for now just show the best move, but format it properly
            let pv_str = move_to_uci(mv.0, mv.1);
            
            // Output UCI info with proper format
            println!("info depth {} score cp {} nodes {} time {} nps {} pv {}",
                depth,
                best_score,
                nodes_searched,
                time_ms,
                nps,
                pv_str
            );
            
            io::stdout().flush().ok();
            
            // If we found a mate, we can stop early
            if best_score.abs() > 9000 {
                break;
            }
        } else {
            // No move found - might be checkmate/stalemate
            break;
        }
        
        // Check if we should stop after this depth
        if SEARCH_STOP.load(Ordering::Relaxed) {
            break;
        }
        
        if let Some(limit) = time_limit {
            if start_time.elapsed() >= limit {
                break;
            }
        }
    }
    
    best_move
}

fn parse_xboard_move(move_str: &str) -> Option<(u8, u8)> {
    // XBoard moves are in format like "d2d4" or "e7e8q" (with promotion)
    if move_str.len() < 4 {
        return None;
    }
    
    let from_coords = &move_str[0..2];
    let to_coords = &move_str[2..4];
    
    let from = coordinates_to_square(from_coords)?;
    let to = coordinates_to_square(to_coords)?;
    
    Some((from, to))
}

fn uci_loop() {
    // Ensure stdout is line buffered for UCI protocol
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    
    let mut board_state = BoardState::new();
    let stdin = io::stdin();
    let mut stdin_handle = stdin.lock();
    let mut buffer = String::new();
    
    // Track protocol mode
    let mut xboard_mode = false;
    let mut time_remaining = 30000u64; // centiseconds
    let mut opponent_time = 30000u64;
    let mut moves_per_session = 40u32;
    let mut base_time = 300u64; // seconds
    let mut increment = 0u64; // seconds
    
    loop {
        buffer.clear();
        match stdin_handle.read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = buffer.trim();
                if line.is_empty() {
                    continue;
                }
                
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                
                // Handle XBoard protocol
                if xboard_mode || parts[0] == "xboard" {
                    match parts[0] {
                        "xboard" => {
                            xboard_mode = true;
                            // Don't send response, just switch mode
                        }
                        "protover" => {
                            if parts.len() > 1 && parts[1] == "2" {
                                writeln!(stdout_handle, "feature done=0").ok();
                                writeln!(stdout_handle, "feature myname=\"Rust Chess Engine\"").ok();
                                writeln!(stdout_handle, "feature done=1").ok();
                                stdout_handle.flush().ok();
                            }
                        }
                        "new" => {
                            board_state = BoardState::new();
                            // Clear transposition table
                            if let Some(tt) = TRANSPOSITION_TABLE.get() {
                                tt.write().unwrap().clear();
                            }
                        }
                        "time" => {
                            if parts.len() > 1 {
                                if let Ok(t) = parts[1].parse::<u64>() {
                                    time_remaining = t;
                                }
                            }
                        }
                        "otim" => {
                            if parts.len() > 1 {
                                if let Ok(t) = parts[1].parse::<u64>() {
                                    opponent_time = t;
                                }
                            }
                        }
                        "level" => {
                            // level <moves> <minutes> <seconds> or level <moves> <base> <inc>
                            // Format: level 0 5 0 means 0 moves in 5 minutes 0 seconds
                            if parts.len() >= 3 {
                                if let (Ok(moves), Ok(base), Ok(inc)) = (
                                    parts[1].parse::<u32>(),
                                    parts[2].parse::<u64>(),
                                    if parts.len() > 3 { parts[3].parse::<u64>() } else { Ok(0) }
                                ) {
                                    moves_per_session = if moves == 0 { 40 } else { moves }; // Default to 40 if 0
                                    base_time = base * 60 + inc; // Convert to total seconds
                                    increment = 0; // Increment handled separately
                                }
                            }
                        }
                        "post" => {
                            // Show thinking - already enabled by default
                        }
                        "hard" => {
                            // Use all available time
                        }
                        "easy" => {
                            // Don't use all available time
                        }
                        "random" => {
                            // Random mode - ignore for now
                        }
                        "quit" => {
                            break;
                        }
                        _ => {
                            // Try to parse as a move (e.g., "d2d4")
                            if let Some((from, to)) = parse_xboard_move(parts[0]) {
                                // Make the opponent's move
                                if make_move(&mut board_state, from, to).is_some() {
                                    // Calculate time limit from time remaining
                                    // Use time_remaining (in centiseconds) or calculate from level
                                    let time_limit_ms = if time_remaining > 0 {
                                        // Use 80% of remaining time, convert centiseconds to milliseconds
                                        Duration::from_millis((time_remaining * 8) / 10)
                                    } else if moves_per_session > 0 {
                                        // Calculate from level: base_time is in seconds
                                        let time_per_move = (base_time * 1000) / moves_per_session as u64;
                                        Duration::from_millis(time_per_move)
                                    } else {
                                        Duration::from_secs(5)
                                    };
                                    
                                    // Find best move (use reasonable depth)
                                    let result = find_best_move_with_time(&board_state, 4, Some(time_limit_ms));
                                    
                                    if let Some((from_move, to_move)) = result {
                                        // Output move in XBoard format
                                        let move_str = format!("{}{}", 
                                            square_to_coordinates(from_move),
                                            square_to_coordinates(to_move)
                                        );
                                        writeln!(stdout_handle, "move {}", move_str).ok();
                                        stdout_handle.flush().ok();
                                    } else {
                                        // Try fallback
                                        let moves = generate_moves(board_state.bitboards, board_state.white_to_move, &board_state);
                                        let legal_moves: Vec<(u8, u8)> = moves.into_iter()
                                            .filter(|&(from, to)| is_move_legal_fast(&board_state, from, to))
                                            .collect();
                                        
                                        if let Some(&(from_move, to_move)) = legal_moves.first() {
                                            let move_str = format!("{}{}", 
                                                square_to_coordinates(from_move),
                                                square_to_coordinates(to_move)
                                            );
                                            writeln!(stdout_handle, "move {}", move_str).ok();
                                            stdout_handle.flush().ok();
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
                
                // Handle UCI protocol
                match parts[0] {
                    "uci" => {
                        writeln!(stdout_handle, "id name Rust Chess Engine").ok();
                        writeln!(stdout_handle, "id author Chess Engine Developer").ok();
                        writeln!(stdout_handle, "uciok").ok();
                        stdout_handle.flush().ok();
                    }
                    "isready" => {
                        writeln!(stdout_handle, "readyok").ok();
                        stdout_handle.flush().ok();
                    }
                    "ucinewgame" => {
                        board_state = BoardState::new();
                        // Clear transposition table
                        if let Some(tt) = TRANSPOSITION_TABLE.get() {
                            tt.write().unwrap().clear();
                        }
                    }
                    "position" => {
                        if parts.len() < 2 {
                            continue;
                        }
                        
                        if parts[1] == "startpos" {
                            board_state = BoardState::new();
                            
                            // Parse moves if any
                            if parts.len() > 2 && parts[2] == "moves" {
                                for i in 3..parts.len() {
                                    if let Some((from, to)) = uci_to_move(parts[i]) {
                                        if make_move(&mut board_state, from, to).is_none() {
                                            eprintln!("Invalid move: {}", parts[i]);
                                            break;
                                        }
                                    }
                                }
                            }
                        } else if parts[1] == "fen" {
                            // Parse FEN string
                            let fen_parts: Vec<&str> = parts[2..].iter().take_while(|&&s| s != "moves").cloned().collect();
                            let fen = fen_parts.join(" ");
                            
                            if let Some(new_state) = parse_fen(&fen) {
                                board_state = new_state;
                                
                                // Parse moves if any
                                if let Some(moves_idx) = parts.iter().position(|&s| s == "moves") {
                                    for i in moves_idx + 1..parts.len() {
                                        if let Some((from, to)) = uci_to_move(parts[i]) {
                                            if make_move(&mut board_state, from, to).is_none() {
                                                eprintln!("Invalid move: {}", parts[i]);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    "go" => {
                        let go_cmd = line;
                        let params = UCISearchParams::parse_go_command(go_cmd);
                        
                        // Use a reasonable default depth and time limit
                        // Start with depth 3 for faster response, increase if time allows
                        let max_depth = params.depth.unwrap_or(3);
                        let time_limit = params.calculate_time_limit(board_state.white_to_move);
                        
                        // Ensure we have at least some time limit to prevent hanging
                        // Use shorter default timeout for faster response
                        let effective_time_limit = time_limit.or_else(|| Some(Duration::from_secs(5)));
                        
                        // Run search synchronously (UCI engines typically block on go command)
                        let result = find_best_move_with_time(&board_state, max_depth, effective_time_limit);
                        
                        if let Some((from, to)) = result {
                            writeln!(stdout_handle, "bestmove {}", move_to_uci(from, to)).ok();
                        } else {
                            // Try to find any legal move as fallback
                            let moves = generate_moves(board_state.bitboards, board_state.white_to_move, &board_state);
                            let legal_moves: Vec<(u8, u8)> = moves.into_iter()
                                .filter(|&(from, to)| is_move_legal_fast(&board_state, from, to))
                                .collect();
                            
                            if let Some(&(from, to)) = legal_moves.first() {
                                writeln!(stdout_handle, "bestmove {}", move_to_uci(from, to)).ok();
                            } else {
                                writeln!(stdout_handle, "bestmove 0000").ok(); // No move found (checkmate/stalemate)
                            }
                        }
                        
                        stdout_handle.flush().ok();
                    }
                    "stop" => {
                        SEARCH_STOP.store(true, Ordering::Relaxed);
                    }
                    "quit" => {
                        break;
                    }
                    _ => {
                        // Ignore unknown commands
                    }
                }
            }
            Err(_) => break,
        }
    }
}

//END OF UCI PROTOCOL------------------------------------------------------------------------------------


fn main() {
    precompute_knight_attacks();
    precompute_king_attacks();
    precompute_pawn_attacks();
    
    // Initialize thread-safe structures
    init_transposition_table(16);
    init_history_table();
    
    // Check if running in UCI mode (default) or test mode
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "test" {
        // Test mode - run all tests
        println!("=== Chess Engine - Test Mode ===");
        test_performance();
        test_evaluation();
        test_minimax();
        test_depth_x(6);
        test_move_generation_depth_6();
        benchmark_search();
        play_game();
    } else {
        // UCI mode - default
        uci_loop();
    }
}