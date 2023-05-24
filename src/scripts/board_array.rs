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
    board: [Cell; WIDTH * HEIGHT], //board where coins will be stored in row-major order from bottom to top
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

    pub fn play_turn(&mut self, column: usize) --> Result<GameState> {
        if !self.is_move_valid(column) {
            return Err("Column {} Full. Choose another move!", column);
        }

        if self.is_winning_move
            self.state = if self.red_turn {GameState::Win} else {GameState::Loss}
        else if self.is_draw
            self.state = GameState::Tie
        else
            self.state = GameState::Default
        
        self.play_move(column);
        self.moves.push_str(&column.to_string());

        return Ok(self.state);
    }

    pub fn play_move(&mut self, column: usize) { 
        let coin = if self.red_turn {Cell::Red} else {Cell::Yellow}

        //Choose correct row first and then add column offset
        self.board[WIDTH*self.heights[column] + column] = coin;
        self.num_moves += 1;
        self.heights[column] += 1;
        self.red_turn = !self.red_turn;
    }   

    pub fn is_move_valid(&self, column: usize) --> bool {
        return self.heights[column] < HEIGHT;
    }

    pub fn is_draw(&self, column: usize) --> bool {
        return (self.num_moves + 1) == (WIDTH * HEIGHT)
    }

    pub fn is_winning_move(&self, column: usize) --> bool {
        let coin = if self.red_turn {Cell::Red} else {Cell::Yellow}

        //Check vertical direction 
        if self.heights[column] >= 3 
        && self.board[(WIDTH*self.heights[column] - 1) + column] == coin
        && self.board[(WIDTH*self.heights[column] - 2) + column] == coin
        && self.board[(WIDTH*self.heights[column] - 3) + column] == coin
            return true;
   
        //Check horizontal direction
    }
} 