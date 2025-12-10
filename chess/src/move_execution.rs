use crate::utils::*;
use crate::types::*;
use crate::board_state::BoardState;

pub struct Move {
    pub from : u8,
    pub to : u8,
    pub piece : usize,
    pub captured_piece : Option<usize>,
    pub promotion : Option <usize>,
    pub castling_move: bool,
    pub en_passant: bool,
    pub previous_castling_rights : (bool, bool, bool, bool),
    pub previous_en_passant_target: Option<u8>,
}

pub fn make_move(board: &mut BoardState, from: u8, to: u8) -> Option<Move> {
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
        None => return None,
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

pub fn unmake_move(board: &mut BoardState, mv: &Move) {
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

pub fn make_castling_move(board: &mut BoardState, from: u8, to: u8, white: bool) -> Option<Move> {
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

pub struct SearchState {
    pub move_history: Vec<Move>,
    pub board: BoardState,
}

impl SearchState {
    pub fn make_move(&mut self, from: u8, to: u8) -> bool {
        if let Some(mv) = make_move(&mut self.board, from, to) {
            self.move_history.push(mv);
            true
        } else {
            false
        }
    }
    
    pub fn unmake_move(&mut self) -> bool {
        if let Some(mv) = self.move_history.pop() {
            unmake_move(&mut self.board, &mv);
            true
        } else {
            false
        }
    }
}

