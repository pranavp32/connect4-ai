use crate::scripts::array_board::{ArrayBoard, GameState};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct AIGame {
    depth: usize,
}

impl AIGame {
    pub fn new() -> Self {
        Self {
            depth: 6,
        }
    }

    pub fn make_move(&self, game: &mut ArrayBoard) -> Result<GameState, String> {
        let mut best_move = 0;
        let mut best_score = std::i64::MIN;

        for col in 0..WIDTH {
            let temp_depth = self.depth;

            if game.is_move_valid(col) {
                let score = self.negamax(game, -21, 21, temp_depth);

                if score > best_score {
                    best_move = col;
                    best_score = score;
                }
            }
        }

        return game.play_turn(best_move);
    }

    pub fn negamax(&self, game: &mut ArrayBoard, mut alpha: i64, mut beta: i64, depth: usize) -> i64 {
        if game.is_draw() {
            return 0;
        } else if depth == 0 {
            return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2).try_into().unwrap();   
        }

        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2).try_into().unwrap();
            }
        }

        let max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) as i64 / 2;
        
        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
            
            if alpha >= beta {
                return beta;
            }
        }

        let temp_alpha = -alpha.clamp(-max, max);
        let temp_beta = -beta.clamp(-max, max);

        for col in 0..WIDTH {
            if game.is_move_valid(col) {
                game.play_move(col);
                let score = -self.negamax(game, temp_beta, temp_alpha, depth - 1);
                let _ = game.undo_move(col);

                if score >= beta {
                    return score;
                }   

                if score > alpha {
                    alpha = score;
                }
            }
        }

        return alpha;
    }
}