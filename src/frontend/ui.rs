use yew::prelude::*;
use crate::scripts::board_array::{ArrayBoard, Cell, GameState};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct Connect4 {
    link: ComponentLink<Self>,
    array_board: ArrayBoard,
    game_over: bool,
}

pub enum Msg {
    ColumnClicked(usize),
    NewGameClicked,
}

impl Component for Connect4 {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4 {
            link,
            array_board: ArrayBoard::new(),
            game_over: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ColumnClicked(column) => {
                if !self.game_over {
                    if let Ok(state1) = self.array_board.play_turn(column) {
                        self.handle_game_state(state1);
                        if let Ok(state2) = self.array_board.ai_move() {
                            self.handle_game_state(state2);
                        }else {
                            println!("Column full, choose another column");
                        }
                        
                    } else {
                        println!("Column full, choose another column");
                    }
                }
            }
            Msg::NewGameClicked => {
                self.start_new_game();
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        self.array_board = ArrayBoard::new();
        true
    }

    fn view(&self) -> Html {
        let css = include_str!("ui.css");
        let game_result = self.render_game_state_message();
        let new_game = self.render_new_game();

        html! {
            <>
                <div class="connect4">
                    <div class="board">
                        { for (0..HEIGHT).map(|row| self.render_row(row)) }
                    </div>
                    <div class="buttons">
                        { for (0..WIDTH).map(|column| self.render_button(column)) }
                    </div>
                    <div class="message">
                        { self.render_turn_message() }
                        { game_result }
                    </div>
                    
                    { new_game }
                </div>
                <style>
                    {css}
                </style>
            </>
        }
    }
}

impl Connect4 {
    fn render_cell(&self, row: usize, column: usize) -> Html {
        let index = ((HEIGHT - 1 - row) * WIDTH) + column;
        let cell = self.array_board.board[index];

        let coin_class = match cell {
            Cell::Red => "red",
            Cell::Yellow => "yellow",
            Cell::Empty => "empty",
        };

        html! {
            <div class=("cell", coin_class)></div>
        }
    }

    fn render_row(&self, row: usize) -> Html {
        html! {
            <div class="row">
                { for (0..WIDTH).map(|column| self.render_cell(row, column)) }
            </div>
        }
    }

    fn render_button(&self, column: usize) -> Html {
        html! {
            <button onclick=self.link.callback(move |_| Msg::ColumnClicked(column))>
                {"|⇩⇩|"}
            </button>
        }
    }

    fn render_turn_message(&self) -> Html {
        let current_player = match self.array_board.red_turn {
            true => "Red",
            false => "Yellow",
        };

        let _state_message = match self.array_board.state {
            GameState::Default => return html! {
                <div class="turn-message">
                    { format!("It's your turn: {}", current_player) }
                </div>
            },
            _ => return html! {}
        };
    }

    fn render_game_state_message(&self) -> Html {
        let state_message = match self.array_board.state {
            GameState::Win => "You won :D",
            GameState::Loss => "You lost :(",
            GameState::Tie => "Tie :|",
            GameState::Default => "Think carefully...",
        };

        html! {
            <div class="game-state-message">
                { state_message }
            </div>
        }
    }

    fn render_new_game(&self) -> Html {
        if self.game_over {
            html! {
                <button onclick=self.link.callback(|_| Msg::NewGameClicked)>
                    {"New Game"}
                </button>
            }
        } else {
            html! {}
        }
    }

    fn handle_game_state(&mut self, state: GameState) {
        match state {
            GameState::Win | GameState::Loss | GameState::Tie => {
                self.game_over = true;
                self.render_turn_message();
            }
            GameState::Default => {}
        }
    }

    fn start_new_game(&mut self) {
        self.array_board = ArrayBoard::new();
        self.game_over = false;
    }
}
