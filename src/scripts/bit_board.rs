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

    pub fn top_col_mask(&self, column: usize) -> u64 {
        let col: usize = WIDTH - column - 1;
        return (1 << (HEIGHT - 1)) << ((HEIGHT + 1) * col);
    }

    pub fn bottom_col_mask(&self, column: usize) -> u64 {
        let col: usize = WIDTH - column - 1;
        return 1 << ((HEIGHT + 1) * col);
    }

    pub fn full_col_mask(&self, column: usize) -> u64 {
        let col: usize = WIDTH - column - 1;
        return ((1 << HEIGHT) - 1) << ((HEIGHT + 1) * col);
    }

    pub fn is_move_valid(&self, column: usize) -> bool {
        let col: usize = WIDTH - column - 1;
        return self.total_mask & self.top_col_mask(col) == 0;
    }

    pub fn get_height_mask(&self) -> u64 {
        return self.total_mask + self.bottom_row;
    }

    pub fn undo_move(&mut self, column: usize) -> Result<GameState, String> {
        let col: usize = WIDTH - column - 1;

        if self.total_mask & self.bottom_col_mask(col) == 0 {
            return Err("Column in empty!".to_string())
        }
        
        self.red_turn = !self.red_turn;
        self.num_moves -= 1;

        if self.red_turn {
            self.player_mask ^= (((self.player_mask + self.bottom_col_mask(col))) & self.get_height_mask()) >> 1;
        }

        self.total_mask ^= (((self.total_mask + self.bottom_col_mask(col))) & self.get_height_mask()) >> 1;        
        Ok(self.state)
    }

    pub fn play_move(&mut self, column: usize) { 
        let col: usize = WIDTH - column - 1;

        if self.red_turn {
            self.player_mask |= self.player_mask + self.bottom_col_mask(col);
        }

        self.total_mask |= self.total_mask + self.bottom_col_mask(col);
        self.num_moves += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn play_turn(&mut self, column: usize) -> Result<GameState, String> {
        let col: usize = WIDTH - column - 1;

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

    pub fn is_winning_move(&mut self, column: usize) -> bool {
        let col: usize = WIDTH - column - 1;
        self.play_move(column);
        let position:u64 = if self.red_turn {self.total_mask ^ self.player_mask}  else {self.player_mask};
        self.undo_move(column);
        let n:u64 = position;

        //horizontal direction
        if n & (n >> 3 * (HEIGHT + 1)) > 0 {
            return true;
        }

        //vertical direction
        if n & (n >> 3) > 0 {
            return true;
        }

        //diagonal = (/) and diagonal = (\)
        if n & (n >> (3 * (HEIGHT + 2))) > 0 || n & (n << (3 * HEIGHT)) > 0 {
            return true;
        }
        
        return false;
    }
} 
