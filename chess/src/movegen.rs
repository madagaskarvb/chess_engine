use crate::utils::*;
use crate::attacks::*;
use crate::types::*;
use crate::attack_bitboards::complete_attacks_bitboard;
use crate::board_state::BoardState;

pub fn generate_pawn_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
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
            if single_push < 64 && get_bit(empty, single_push) {
                moves.push((from_square, single_push));
                
                // Double push from starting rank
                if rank == 6 {  // White pawns start on rank 6 (48-55)
                    let double_push = from_square - 16;
                    if double_push < 64 && get_bit(empty, double_push) {
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
            if single_push < 64 && get_bit(empty, single_push) {
                moves.push((from_square, single_push));
                
                // Double push from starting rank  
                if rank == 1 {  // Black pawns start on rank 1 (8-15)
                    let double_push = from_square + 16;
                    if double_push < 64 && get_bit(empty, double_push) {
                        moves.push((from_square, double_push));
                    }
                }
            }
            
            // Captures
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

pub fn generate_knight_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let knights = if white { board[WN] } else { board[BN] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    
    let mut knights_copy = knights;
    while knights_copy != 0 {
        let from_square = get_lsb(knights_copy).unwrap();
        clear_bit(&mut knights_copy, from_square);
        
        let attacks: u64 = KNIGHT_ATTACKS.get().unwrap()[from_square as usize];
        let mut attacks_copy: u64 = attacks;
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

pub fn generate_king_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool, board_state: &BoardState) {
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
        
        generate_castling_moves(board, moves, white, board_state, from_square);
    }
}

pub fn generate_castling_moves(
    board: &[u64; 12], 
    moves: &mut Vec<(u8, u8)>, 
    white: bool, 
    board_state: &BoardState, 
    _king_square: u8
) {
    let all_occupied = get_all_occupied(*board);
    let enemy_attacks = complete_attacks_bitboard(board, !white);
    
    if white {
        // White kingside castling (O-O)
        if board_state.white_kingside_castle {
            let empty_squares = !get_bit(all_occupied, 61) && !get_bit(all_occupied, 62);
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

pub fn generate_bishop_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let bishops = if white { board[WB] } else { board[BB] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut bishops_copy = bishops;
    while bishops_copy != 0 {
        let from_square = get_lsb(bishops_copy).unwrap();
        clear_bit(&mut bishops_copy, from_square);
        
        let attacks = crate::attacks::get_bishop_attacks(from_square, all_occupied);
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

pub fn generate_rook_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let rooks = if white { board[WR] } else { board[BR] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut rooks_copy = rooks;
    while rooks_copy != 0 {
        let from_square = get_lsb(rooks_copy).unwrap();
        clear_bit(&mut rooks_copy, from_square);
        
        let attacks = crate::attacks::get_rook_attacks(from_square, all_occupied);
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

pub fn generate_queen_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool) {
    let queens = if white { board[WQ] } else { board[BQ] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    let all_occupied = get_all_occupied(*board);
    
    let mut queens_copy = queens;
    while queens_copy != 0 {
        let from_square = get_lsb(queens_copy).unwrap();
        clear_bit(&mut queens_copy, from_square);
        
        let attacks = crate::attacks::get_queen_attacks(from_square, all_occupied);
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

pub fn generate_moves(board: [u64; 12], white_move: bool, board_state: &BoardState) -> Vec<(u8, u8)> {
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

