use std::convert::TryInto;
const HEIGHT: usize = 6; 
const WIDTH: usize = 7;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    Win,
    Loss,
    Tie,
    Default,
}

use GameState::{Win, Loss, Tie, Default};

pub struct TranspositionTable {
    items: Vec<(u64, u64)>,
}   

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            items: vec![(0, 0); size],    
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

pub struct BitBoard {
    pub player_mask: u64, //first 49 bits used to store current player position
    pub total_mask: u64, //first 49 bits used to store all played coins
    pub bottom_row: u64,
    num_moves: usize, //total number of moves played in the current game
    pub red_turn: bool, //used to signify whose turn it is
    pub state: GameState, //current game state
}

//Implementation of board
impl BitBoard {
    pub fn new() -> Self {
        let mut temp:u64 = 1;

        for col in 1..WIDTH {
            temp |= 1 << (HEIGHT + 1) * col
        }

        Self {
            player_mask: 0, 
            total_mask: 0,
            bottom_row: temp,
            num_moves: 0,
            red_turn: true,
            state: Default,
        }
    }

    pub fn top_col_mask(&self, col: usize) -> u64 {
        return (1 << (HEIGHT - 1)) << ((HEIGHT + 1) * col);
    }

    pub fn bottom_col_mask(&self, col: usize) -> u64 {
        return 1 << ((HEIGHT + 1) * col);
    }

    pub fn full_col_mask(&self, col: usize) -> u64 {
        return ((1 << HEIGHT) - 1) << ((HEIGHT + 1) * col);
    }

    pub fn is_move_valid(&self, col: usize) -> bool {
        return self.total_mask & self.top_col_mask(col) == 0;
    }

    pub fn get_height_mask(&self) -> u64 {
        return self.total_mask + self.bottom_row;
    }

    pub fn get_unique_key(&self) -> u64 {
        let current = if self.red_turn{self.player_mask} else {self.player_mask ^ self.total_mask};
        //let current = self.player_mask;
        return current + self.total_mask;
    }

    pub fn undo_move(&mut self, col: usize) -> Result<GameState, String> {

        if self.total_mask & self.bottom_col_mask(col) == 0 {
            return Err("Column in empty!".to_string())
        }
        
        self.red_turn = !self.red_turn;
        self.num_moves -= 1;
        let yellow_mask:u64 = self.player_mask ^ self.total_mask;
        self.total_mask ^= (((self.total_mask + self.bottom_col_mask(col))) & self.get_height_mask()) >> 1;        

        if self.red_turn {
            self.player_mask = yellow_mask ^ self.total_mask;
        }

        Ok(self.state)
    }

    pub fn play_move(&mut self, col: usize) { 

        let yellow_mask:u64 = self.player_mask ^ self.total_mask;
        self.total_mask |= self.total_mask + self.bottom_col_mask(col);

        if self.red_turn {
            self.player_mask = yellow_mask ^ self.total_mask;
        }

        self.num_moves += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn play_turn(&mut self, col: usize) -> Result<GameState, String> {
        if !self.is_move_valid(col) {
            return Err("Column is full. Choose another move!".to_string());
        }

        if self.is_winning_move(col) {
            self.state = if self.red_turn {Win} else {Loss};
        } else if self.is_draw() {
            self.state = Tie;
        } else {
            self.state = Default;
        }
        
        self.play_move(col);

        return Ok(self.state);
    }

    pub fn is_draw(&self) -> bool {
        return self.num_moves == (WIDTH * HEIGHT);
    }

    pub fn get_num_moves(&self) -> usize {
        return self.num_moves;
    }

    pub fn is_winning_move(&mut self, col: usize) -> bool {
        self.play_move(col);
        let position:u64 = if self.red_turn {self.total_mask ^ self.player_mask}  else {self.player_mask};
        self.undo_move(col);
        
        //horizontal direction
        let n:u64 = position & (position >> (HEIGHT + 1));
        if n & (n >> 2 * (HEIGHT + 1)) > 0 {
            return true;
        }

        //vertical direction
        let n:u64 = position & (position >> 1);

        if n & (n >> 2) > 0 {
            return true;
        }

        //diagonal = (/) direction
        let n:u64 = position & (position >> (HEIGHT - 1));
        if n & (n >> 2 * (HEIGHT - 2)) > 0 {
            return true;
        }
        
        //diagonal = (\) direction
        let n:u64 = position & (position >> (HEIGHT + 2));
        if n & (n >> 2 * (HEIGHT + 2)) > 0 {
            return true;
        }

        return false;
    }
} 

pub struct AIGame {
    column_order: [i64; WIDTH],
}

impl AIGame {
    pub fn new() -> Self {
            let mut column_order = [0; WIDTH];

            for i in 0..WIDTH {
                column_order[i] = WIDTH as i64 / 2 + (1 - 2 * (i as i64 % 2)) * (i as i64 + 1) / 2;
            }

            AIGame {
                column_order
            }
    }

    pub fn make_move(&self, game: &mut BitBoard, trans_table: &mut TranspositionTable) -> Result<GameState, String> {
        let mut best_move = 0;
        let mut best_score = std::i64::MIN;

        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                if game.is_winning_move(chosen_col) {
                    return game.play_turn(chosen_col);
                }

                let init:i64 = ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -init, init, 43);
                let _ = game.undo_move(chosen_col);

                if score > best_score {
                    best_move = chosen_col;
                    best_score = score;
                }
            }
        }

        return game.play_turn(best_move);
    }

    pub fn negamax(&self, game: &mut BitBoard, trans_table: &mut TranspositionTable, mut alpha: i64, mut beta: i64, depth: i64) -> i64 {
        if game.get_num_moves() >= WIDTH * HEIGHT {
            return 0;
        } 

        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
            }
        }

        let min = -(((WIDTH * HEIGHT - 2 - game.get_num_moves()) / 2) as i64);

        if alpha < min {
            alpha = min;                     
            
            if alpha >= beta {
                return alpha; 
            }
        }

        let mut max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) as i64 / 2;
        let val = trans_table.get(game.get_unique_key());
        if val != 0 {
            max = val as i64 - (WIDTH * HEIGHT / 2) as i64 + 3;
        }

        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
            
            if alpha >= beta {
                return beta;
            }
        }
        
        if depth == 0 {
            // return alpha;
            return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2) as i64;
        }

        for col in 0..WIDTH {
            let chosen_col = self.column_order[col].try_into().unwrap(); 

            if game.is_move_valid(chosen_col) {
                game.play_move(chosen_col);
                let score = -self.negamax(game, trans_table, -beta, -alpha, depth - 1);
                let _ = game.undo_move(chosen_col);

                if score >= beta {
                    return score;
                }   

                if score > alpha {
                    alpha = score;
                }
            }
        }

        trans_table.insert(game.get_unique_key(), (alpha + (WIDTH * HEIGHT / 2) as i64 - 3) as u64);
        return alpha;
    }
}



fn main() {
    let mut bit_board = BitBoard::new();
    let mut ai = AIGame::new();
    let mut trans_table = TranspositionTable::new(83885931);


    bit_board.play_move(3);
    bit_board.play_move(3);
    bit_board.play_move(3);
    bit_board.play_move(3);
    bit_board.play_move(3);
    bit_board.play_move(3);
    bit_board.play_move(2);
    ai.make_move(&mut bit_board, &mut trans_table);
}