use crate::utils::*;
use crate::types::*;
use crate::board_state::BoardState;
use crate::move_execution::{SearchState, make_move};
use crate::movegen::generate_moves;
use crate::move_ordering::{order_moves, add_killer_move, update_history_score};
use crate::transposition_table::{TranspositionTable, get_best_move_from_tt};
use crate::attack_bitboards::complete_attacks_bitboard;
use crate::evaluation::{evaluate_board_advanced, evaluate_board_fast};
use crate::printing::square_to_coordinates;
use crate::zobrist::compute_board_hash;

static DEBUG: bool = false;

pub fn find_best_move(board_state: &BoardState, depth: u8) -> Option<(u8, u8)> {
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

pub fn find_best_move_iterative_deepening_optimized(
    board_state: &BoardState, 
    max_depth: u8, 
    _time_limit_ms: u64
) -> Option<(u8, u8)> {
    let mut tt = TranspositionTable::new(16);
    let mut best_move = None;
    let mut alpha = i32::MIN + 1;
    let mut beta = i32::MAX - 1;
    
    for depth in 1..=max_depth {
        let window = 50;
        
        let score = negamax_root(
            board_state,
            depth as i32,
            alpha,
            beta,
            0,
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
pub fn minimax(
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
    
    let moves_with_scores: Vec<(i32, u8, u8)> = order_moves(&search_state.board, &legal_moves);
    
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
pub fn quiescence_search(
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

pub fn quiescence_search_enhanced(
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

pub fn negamax_enhanced(
    search_state: &mut SearchState,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    ply: usize,
    tt: &mut TranspositionTable,
) -> i32 {
    if depth == 0 {
        return quiescence_search_enhanced(search_state, alpha, beta, search_state.board.white_to_move);
    }
    
    let original_alpha = alpha;
    let hash = compute_board_hash(&search_state.board);
    
    // TT lookup with improved probing
    if let Some((score, best_move)) = tt.probe(hash, depth, alpha, beta) {
        // Store as killer move if it caused a cutoff
        if (score >= beta) && (best_move.0 != best_move.1) {
            add_killer_move(ply, best_move.0, best_move.1);
        }
        return score;
    }
    
    // Null move pruning (optional but effective)
    if depth >= 3 && !search_state.board.is_current_king_in_check() {
        // Try a null move
        search_state.board.white_to_move = !search_state.board.white_to_move;
        let null_score = -negamax_root(
            &search_state.board,
            depth - 1 - 2,
            -beta,
            -beta + 1,
            ply + 1,
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
            if get_piece_at_square(&search_state.board.bitboards, to).is_none() {
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

pub fn negamax_root(
    board_state: &BoardState,
    depth: i32,
    alpha: i32,
    beta: i32,
    ply: usize,
    tt: &mut TranspositionTable,
) -> i32 {
    let mut search_state = SearchState {
        board: *board_state,
        move_history: Vec::new(),
    };
    negamax_enhanced(&mut search_state, depth, alpha, beta, ply, tt)
}

pub fn is_move_legal_fast(board: &BoardState, from: u8, to: u8) -> bool {
    let mut board_copy = *board;
    if make_move(&mut board_copy, from, to).is_none() {
        return false;
    }
    
    let king_square = if board.white_to_move {
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

