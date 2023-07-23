use crate::scripts::array_board::{ArrayBoard, GameState};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct AIGame {}

impl AIGame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_move(&self, game: &mut ArrayBoard) -> Result<GameState, String> {
        let mut best_move = 0;
        let mut best_score = std::i64::MIN;

        for col in 0..WIDTH {
            if game.is_move_valid(col) {
                let score = self.negamax(game, -22, 22);

                if score > best_score {
                    best_move = col;
                    best_score = score;
                }
            }
        }

        return game.play_turn(best_move);
    }

    pub fn negamax(&self, game: &mut ArrayBoard, mut alpha: i64, mut beta: i64) -> i64 {
        if game.is_draw() {
            return 0;
        }

        for col in 0..WIDTH {
            if game.is_move_valid(col) && game.is_winning_move(col) {
                return ((WIDTH * HEIGHT + 1 - game.get_num_moves()) / 2).try_into().unwrap();
            }
        }

        let max = (WIDTH * HEIGHT - 1 - game.get_num_moves()) / 2;

        if beta > max.try_into().unwrap() {
            beta = max.try_into().unwrap();
            
            if alpha >= beta {
                return beta;
            }
        }

        for col in 0..WIDTH {
            if game.is_move_valid(col) {
                game.play_move(col);
                let score = -self.negamax(game, -beta, -alpha);
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