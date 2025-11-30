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

fn get_bit(bitboard: u64, square: u8) -> bool {
    return (bitboard >> square) & 1 != 0;
}


fn set_bit(bitboard: &mut u64, square: u8) {
    *bitboard |= 1 << square;
}


fn clear_bit(bitboard: &mut u64, square: u8) {
    *bitboard &= !(1 << square);
}


fn count_bits(bitboard: u64) -> u32 {
    bitboard.count_ones()
}


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

static mut 
KNIGHT_ATTACKS: [u64; 64] = [0; 64];

fn precompute_knight_attacks() {
    let knight_moves = [
        (2, 1), (2, -1), (-2, 1), (-2, -1),
        (1, 2), (1, -2), (-1, 2), (-1, -2)
    ];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut attacks = 0;
        
        for &(dr, df) in &knight_moves {
            let new_rank = rank as i32 + dr;
            let new_file = file as i32 + df;
            
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as u8;
                set_bit(&mut attacks, target_square);
            }
        }
        
        unsafe { KNIGHT_ATTACKS[square as usize] = attacks; }
    }
}


static mut KING_ATTACKS: [u64; 64] = [0; 64];

fn precompute_king_attacks() {
    let king_moves: [(i32, i32); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1),
        (1, 1), (1, -1), (-1, 1), (-1, -1)
    ];
    
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut attacks = 0;
        
        for &(dr, df) in &king_moves {
            let new_rank = rank as i32 + dr;
            let new_file = file as i32 + df;
            
            if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
                let target_square = (new_rank * 8 + new_file) as u8;
                set_bit(&mut attacks, target_square);
            }
        }
        
        unsafe { KING_ATTACKS[square as usize] = attacks; }
    }
}


static mut WHITE_PAWN_ATTACKS: [u64; 64] = [0; 64];
static mut BLACK_PAWN_ATTACKS: [u64; 64] = [0; 64];

fn precompute_pawn_attacks() {
    for square in 0..64 {
        let (rank, file) = (square / 8, square % 8);
        let mut white_attacks = 0;
        let mut black_attacks = 0;
        
        // White pawns
        if rank < 7 {
            if file > 0 { set_bit(&mut white_attacks, square + 7); } // up-left
            if file < 7 { set_bit(&mut white_attacks, square + 9); } // up-right
        }
        
        // Black pawns
        if rank > 0 {
            if file > 0 { set_bit(&mut black_attacks, square - 9); } // down-left
            if file < 7 { set_bit(&mut black_attacks, square - 7); } // down-right
        }
        
        unsafe {
            WHITE_PAWN_ATTACKS[square as usize] = white_attacks;
            BLACK_PAWN_ATTACKS[square as usize] = black_attacks;
        }
    }
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
            unsafe {
                let attacks = WHITE_PAWN_ATTACKS[from_square as usize];
                let mut attacks_copy = attacks;
                while attacks_copy != 0 {
                    let target_square = get_lsb(attacks_copy).unwrap();
                    clear_bit(&mut attacks_copy, target_square);
                    
                    if get_bit(enemy_occupied, target_square) {
                        moves.push((from_square, target_square));
                    }
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
                let attacks = BLACK_PAWN_ATTACKS[from_square as usize];
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
        
        unsafe {
            let attacks = KNIGHT_ATTACKS[from_square as usize];
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
}


fn generate_king_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool, board_state: &BoardState) {
    let king = if white { board[WK] } else { board[BK] };
    let friendly_occupied = if white { get_all_white(*board) } else { get_all_black(*board) };
    
    if king != 0 {
        let from_square = get_lsb(king).unwrap();
        
        unsafe {
            let attacks = KING_ATTACKS[from_square as usize];
            let mut attacks_copy = attacks;
            while attacks_copy != 0 {
                let target_square = get_lsb(attacks_copy).unwrap();
                clear_bit(&mut attacks_copy, target_square);
                
                if !get_bit(friendly_occupied, target_square) {
                    moves.push((from_square, target_square));
                }
            }
        }
        
        // Add castling moves
        generate_castling_moves(board, moves, white, board_state, from_square);
    }
}

fn generate_castling_moves(board: &[u64; 12], moves: &mut Vec<(u8, u8)>, white: bool, board_state: &BoardState, king_square: u8) {
    let all_occupied = get_all_occupied(*board);
    let enemy_attacks = complete_attacks_bitboard(board, !white);
    
    if white {
        // White kingside castling (O-O)
        if board_state.white_kingside_castle {
            // Check if squares between king and rook are empty
            let empty_squares = !get_bit(all_occupied, 61) && !get_bit(all_occupied, 62); // f1 and g1
            
            // Check if king is not in check and doesn't pass through attacked squares
            let safe_squares = !get_bit(enemy_attacks, 60) && // e1
                             !get_bit(enemy_attacks, 61) && // f1
                             !get_bit(enemy_attacks, 62);   // g1
            
            if empty_squares && safe_squares {
                moves.push((60, 62)); // e1 -> g1
            }
        }
        
        // White queenside castling (O-O-O)
        if board_state.white_queenside_castle {
            // Check if squares between king and rook are empty
            let empty_squares = !get_bit(all_occupied, 59) && // d1
                             !get_bit(all_occupied, 58) && // c1
                             !get_bit(all_occupied, 57);   // b1
            
            // Check if king is not in check and doesn't pass through attacked squares
            let safe_squares = !get_bit(enemy_attacks, 60) && // e1
                             !get_bit(enemy_attacks, 59) && // d1
                             !get_bit(enemy_attacks, 58);   // c1
            
            if empty_squares && safe_squares {
                moves.push((60, 58)); // e1 -> c1
            }
        }
    } else {
        // Black kingside castling (O-O)
        if board_state.black_kingside_castle {
            // Check if squares between king and rook are empty
            let empty_squares = !get_bit(all_occupied, 5) && !get_bit(all_occupied, 6); // f8 and g8
            
            // Check if king is not in check and doesn't pass through attacked squares
            let safe_squares = !get_bit(enemy_attacks, 4) && // e8
                             !get_bit(enemy_attacks, 5) && // f8
                             !get_bit(enemy_attacks, 6);   // g8
            
            if empty_squares && safe_squares {
                moves.push((4, 6)); // e8 -> g8
            }
        }
        
        // Black queenside castling (O-O-O)
        if board_state.black_queenside_castle {
            // Check if squares between king and rook are empty
            let empty_squares = !get_bit(all_occupied, 3) && // d8
                             !get_bit(all_occupied, 2) && // c8
                             !get_bit(all_occupied, 1);   // b8
            
            // Check if king is not in check and doesn't pass through attacked squares
            let safe_squares = !get_bit(enemy_attacks, 4) && // e8
                             !get_bit(enemy_attacks, 3) && // d8
                             !get_bit(enemy_attacks, 2);   // c8
            
            if empty_squares && safe_squares {
                moves.push((4, 2)); // e8 -> c8
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
        
        unsafe {
            if white_pawns {
                attacks |= WHITE_PAWN_ATTACKS[square as usize];
            } else {
                attacks |= BLACK_PAWN_ATTACKS[square as usize];
            }
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
        
        unsafe {
            attacks |= KNIGHT_ATTACKS[square as usize];
        }
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
        unsafe {
            attacks |= KING_ATTACKS[square as usize];
        }
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
    let rank_char: char = (b'8' - rank) as char; // Note: 0=a8, 63=h1 in your system
    format!("{}{}", file_char, rank_char)
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

fn make_move(board: &mut BoardState, from: u8, to: u8) -> bool {
    // Find which piece is moving
    let mut moving_piece = None;
    let start_range = if board.white_to_move { 0 } else { 6 };
    let end_range = if board.white_to_move { 6 } else { 12 };
    
    for piece_type in start_range..end_range {
        if get_bit(board.bitboards[piece_type], from) {
            moving_piece = Some(piece_type);
            break;
        }
    }
    
    let moving_piece = match moving_piece {
        Some(p) => p,
        None => return false,
    };
    
    // Handle castling moves first
    if (moving_piece == WK || moving_piece == BK) && (from as i32 - to as i32).abs() == 2 {
        return make_castling_move(board, from, to, moving_piece == WK);
    }
    
    // Handle captures
    let mut captured_piece = None;
    let enemy_start = if board.white_to_move { 6 } else { 0 };
    let enemy_end = if board.white_to_move { 12 } else { 6 };
    
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
    
    // Update castling rights if king or rook moves
    if moving_piece == WK || moving_piece == BK {
        board.king_moved(moving_piece == WK);
    } else if moving_piece == WR || moving_piece == BR {
        board.rook_moved(from, moving_piece == WR);
    }
    
    // Switch sides
    board.white_to_move = !board.white_to_move;
    
    // Update check status after the move
    board.update_check_status();
    
    true
}

fn make_castling_move(board: &mut BoardState, from: u8, to: u8, white: bool) -> bool {
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
    
    // Move king
    clear_bit(&mut board.bitboards[if white { WK } else { BK }], king_from);
    set_bit(&mut board.bitboards[if white { WK } else { BK }], king_to);
    
    // Move rook
    clear_bit(&mut board.bitboards[if white { WR } else { BR }], rook_from);
    set_bit(&mut board.bitboards[if white { WR } else { BR }], rook_to);
    
    // Update castling rights
    if white {
        board.white_kingside_castle = false;
        board.white_queenside_castle = false;
    } else {
        board.black_kingside_castle = false;
        board.black_queenside_castle = false;
    }
    
    // Switch sides
    board.white_to_move = !board.white_to_move;
    
    // Update check status
    board.update_check_status();
    
    true
}

//END OF MOVE EXECUTION-----------------------------------------------------------------------------------


//EVALUATION----------------------------------------------------------------------------------------------

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

//END OF TESTS--------------------------------------------------------------------------------------------

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


fn main() {
    precompute_knight_attacks();
    precompute_king_attacks();
    precompute_pawn_attacks();
    
    println!("=== Chess Engine ===");
    
    // Test evaluation
    test_evaluation();
    
    // Create initial board state
    let board_state = BoardState::new();
    
    println!("\n=== Initial Position ===");
    print_board(&board_state);
    println!("Evaluation: {}", evaluate_board_advanced(&board_state));
    
    // Test some moves and their evaluations
    let white_moves = generate_moves(board_state.bitboards, true, &board_state); // Add &board_state
    println!("\n=== Testing First Few Moves ===");
    
    for &(from, to) in white_moves.iter().take(3) {
        let mut new_state = board_state;
        if make_move(&mut new_state, from, to) {
            let eval = evaluate_board_advanced(&new_state);
            println!(
                "Move: {} -> {} | Evaluation: {}",
                square_to_coordinates(from),
                square_to_coordinates(to),
                eval
            );
            print_board(&new_state);
        }
    }
    
    // Test a capture scenario
    println!("\n=== Testing Capture Scenario ===");
    // Set up a position where white can capture a black knight
    let mut capture_test = BoardState::new();
    // Move a white pawn to e4
    make_move(&mut capture_test, 52, 36); // e2 -> e4
    // Move a black knight to f6 (where it can be captured)
    make_move(&mut capture_test, 1, 21); // b8 -> f6
    
    print_board(&capture_test);
    println!("Evaluation before capture: {}", evaluate_board_advanced(&capture_test));
    
    // Generate captures for white
    let capture_moves = generate_moves(capture_test.bitboards, true, &capture_test); // Add &capture_test
    let knight_captures: Vec<(u8, u8)> = capture_moves.iter()
        .filter(|&&(_, to)| to == 21) // Moves that capture the knight on f6
        .cloned()
        .collect();
    
    if let Some(&(from, to)) = knight_captures.get(0) {
        let mut after_capture = capture_test;
        make_move(&mut after_capture, from, to);
        println!("\nAfter capturing knight with {} -> {}:", 
                square_to_coordinates(from), square_to_coordinates(to));
        print_board(&after_capture);
        println!("Evaluation after capture: {}", evaluate_board_advanced(&after_capture));
    }
}