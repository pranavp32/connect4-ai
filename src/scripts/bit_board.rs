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

    pub fn top_col_mask(column: usize) -> u64 {
        return (1 << (HEIGHT - 1)) << ((HEIGHT + 1) * column);
    }

    pub fn bottom_col_mask(column: usize) -> u64 {
        return 1 << ((HEIGHT + 1) * column);
    }

    pub fn full_col_mask(column: usize) -> u64 {
        return ((1 << HEIGHT) - 1) << ((HEIGHT + 1) * column);
    }

    pub fn is_move_valid(&self, column: usize) -> bool {
        return mask & top_col_mask(column) == 0;
    }

    pub fn get_height_mask(&self) -> u64 {
        return mask + self.bottom_row;
    }

    pub fn undo_move(&mut self, column: usize) -> Result<GameState, String> {
        if self.total_mask & self.bottom_col_mask(col) == 0 {
            return Err("Column in empty!".to_string())
        }

        self.red_turn = !self.red_turn;
        self.num_moves -= 1;

        if self.red_turn {
            self.player_mask ^= (((self.player_mask + self.bottom_col_mask(col))) & self.height_mask()) >> 1;
        }

        self.total_mask ^= (((self.total_mask + self.bottom_col_mask(col))) & self.get_height_mask()) >> 1;        
        Ok(self.state)
    }

    pub fn play_move(&mut self, column: usize) { 
        if self.red_turn {
            self.player_mask |= self.player_mask + self.bottom_col_mask(column);
        }

        self.total_mask |= self.total_mask + self.bottom_col_mask(column);
        self.num_moves += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn play_turn(&mut self, column: usize) -> Result<GameState, String> {
        if !self.is_move_valid(column) {
            return Err("Column is full. Choose another move!".to_string());
        }

        if self.is_winning_move(column) {
            self.state = if self.red_turn {Win} else {Loss};
        } else if self.is_draw() {
            self.state = Tie;
        } else {
            self.state = Default;
        }
        
        self.play_move(column);

        return Ok(self.state);
    }

    pub fn is_draw(&self) -> bool {
        return self.num_moves == (WIDTH * HEIGHT);
    }

    pub fn get_num_moves(&self) -> usize {
        return self.num_moves;
    }

    pub fn is_winning_move(&self, column: usize) -> bool {
        let positon:u64 = self.total_mask ^ self.player_mask if !self.red_turn else self.player_mask;

        //horizontal direction
        let n:u64 = 
        
    }
} 
