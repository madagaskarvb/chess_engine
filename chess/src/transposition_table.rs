use std::sync::OnceLock;
use std::sync::{RwLock, Arc};
use std::sync::atomic::{AtomicI32, Ordering};

pub static TRANSPOSITION_TABLE: OnceLock<Arc<RwLock<TranspositionTable>>> = OnceLock::new();

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: i32,
    pub score: i32,
    pub flag: u8, // 0=exact, 1=upper bound, 2=lower bound
    pub best_move: (u8, u8),
}

pub struct TranspositionTable {
    pub entries: Vec<Option<TTEntry>>,
    pub size: usize,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        let entry_size = std::mem::size_of::<TTEntry>();
        let size = (size_mb * 1024 * 1024) / entry_size;
        let size = size.max(1024);
        
        Self {
            entries: vec![None; size],
            size,
        }
    }
    
    pub fn store(&mut self, hash: u64, depth: i32, score: i32, flag: u8, best_move: (u8, u8)) {
        let index = (hash as usize) % self.size;
        
        if let Some(existing) = &self.entries[index] {
            if existing.depth > depth && existing.hash == hash {
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
    
    pub fn probe(&self, hash: u64, depth: i32, alpha: i32, beta: i32) -> Option<(i32, (u8, u8))> {
        let index = (hash as usize) % self.size;
        
        if let Some(entry) = &self.entries[index] {
            if entry.hash == hash && entry.depth >= depth {
                match entry.flag {
                    0 => {
                        return Some((entry.score, entry.best_move));
                    }
                    1 => {
                        if entry.score <= alpha {
                            return Some((alpha, entry.best_move));
                        }
                    }
                    2 => {
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
    
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = None;
        }
    }
}

pub fn init_transposition_table(size_mb: usize) {
    let _ = TRANSPOSITION_TABLE.set(Arc::new(RwLock::new(TranspositionTable::new(size_mb))));
}

pub fn get_transposition_table() -> Arc<RwLock<TranspositionTable>> {
    TRANSPOSITION_TABLE
        .get_or_init(|| Arc::new(RwLock::new(TranspositionTable::new(16))))
        .clone()
}

pub fn get_tt_move(hash: u64) -> Option<(u8, u8)> {
    let tt = get_transposition_table();
    let tt_guard = tt.read().unwrap();
    
    let index = (hash as usize) % tt_guard.size;
    
    if let Some(entry) = &tt_guard.entries[index] {
        if entry.hash == hash {
            return Some(entry.best_move);
        }
    }
    
    None
}

pub fn get_best_move_from_tt(board_state: &crate::board_state::BoardState, tt: &TranspositionTable) -> Option<(u8, u8)> {
    let hash = crate::zobrist::compute_board_hash(board_state);
    let index = (hash as usize) % tt.size;
    if let Some(entry) = &tt.entries[index] {
        if entry.hash == hash {
            return Some(entry.best_move);
        }
    }
    None
}

pub struct ThreadSafeHistoryTable {
    pub table: Box<[AtomicI32; 64 * 64]>,
}

impl ThreadSafeHistoryTable {
    pub fn new() -> Self {
        let table = Box::new([const { AtomicI32::new(0) }; 64 * 64]);
        Self { table }
    }
    
    pub fn get(&self, from: u8, to: u8) -> i32 {
        let index = from as usize * 64 + to as usize;
        self.table[index].load(Ordering::Relaxed)
    }
    
    pub fn update(&self, from: u8, to: u8, depth: i32) {
        let index = from as usize * 64 + to as usize;
        let current = self.table[index].load(Ordering::Relaxed);
        let new_value = current + depth * depth;
        
        let capped_value = if new_value > 1_000_000 { new_value / 2 } else { new_value };
        self.table[index].store(capped_value, Ordering::Relaxed);
    }
}

