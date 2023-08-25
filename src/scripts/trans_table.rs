pub struct TranspositionTable {
    items: [(u64, u64); 8388593],
}   

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            items: [(0, 0); 8388593],    
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
}