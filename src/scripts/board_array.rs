const HEIGHT: usize = 6,; 
const WEIGHT: usize = 7;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cell {
    Red,
    Yellow,
    Empty
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GameState {
    Win,
    Loss,
    Tie,
    Default
}

pub struct ArrayBoard {
    board: [Cell; WIDTH * HEIGHT], //board
    num_moves: usize, //number of played moves
    pub red_turn: bool, //used to signify whose turn it is
    heights: [usize; WIDTH], //height of each column (number of coins in each col)
    pub moves: String::new(), //numeric string to signify sequence of moves
    pub state: GameState, //current game state
}

//Implementation of board
impl ArrayBoard {
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; WIDTH * HEIGHT],
            num_moves: 0,
            red_turn: true,
            heights: [0; WIDTH],
            moves: String::new(),
            state: GameState::Default,
        }
    }

    pub fn play_move(&mut self, column: usize) {
        let coin = if self.red_turn {Cell::Red} else {Cell::Yellow}

        
    }  
} 