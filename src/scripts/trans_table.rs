pub struct TranspositionTable {
    items: Vec<(u64, u64)>,
    len: usize,
}   

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            items: vec![(0, 0); size],    
            len: size,
        }
    }

    pub fn index(&self, key: u64) -> usize {
        (key as usize) % self.items.len()
    }

    pub fn insert(&mut self, key: u64, val: u64) {
        let idx: usize = self.index(key);
        self.items[idx] = (key, val);
    }

    pub fn get(&self, key: u64) -> u64 {
        let idx: usize = self.index(key);
        self.items[idx].1
    }

    pub fn reset(&mut self) {
        self.items = vec![(0, 0); self.len];
    }
}