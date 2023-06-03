use yew::prelude::*;

mod board_array;

use board_array::{ArrayBoard, Cell, GameState};

const HEIGHT: usize = 6;
const WIDTH: usize = 7;

pub struct Connect4 {
    link: ComponentLink<Self>,
    board: ArrayBoard,
}

pub enum Msg {
    ColumnClicked(usize),
}

impl Component for Connect4 {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4 {
            link,
            board: ArrayBoard::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ColumnClicked(column) => {
                if let Ok(state) = self.board.play_turn(column) {
                    match state {
                        GameState::Win => {
                            println!("You won :D");
                        }
                        GameState::Loss => {
                            println!("You lost :(");
                        }
                        GameState::Tie => {
                            println!("Tie :|");
                        }
                        GameState::Default => {
                            println!("Think carefully...");
                        }
                    }
                } else {
                    println!("Column full, choose another column");
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="connect4">
                <div class="board">
                    { for (0..HEIGHT).map(|row| self.render_row(row)) }
                </div>
                <div class="message">
                    { self.render_turn_message() }
                    { self.render_game_state_message() }
                </div>
                <div class="buttons">
                    { for (0..WIDTH).map(|column| self.render_button(column)) }
                </div>
            </div>
        }
    }
}

impl Connect4 {

    fn render_cell(&self, row: usize, column: usize) -> Html {
        let index = row*WIDTH + column;
        let cell = self.board.board[index];

        let class = match cell {
            Cell::Red => "red",
            Cell::Yellow => "yellow",
            Cell::Empty => "empty",
        };

        html! {
            <div class={class}></div>
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
                {"Move valid"}
            </button>
        }
    }

    fn render_turn_message(&self) -> Html {
        let current_player = match self.board.current_player {
            Cell::Red => "Red",
            Cell::Yellow => "Yellow",
        };

        html! {
            <div class="turn-message">
                { format!("It's your turn: {}", current_player) }
            </div>
        }
    }

    fn render_game_state_message(&self) -> Html {
        let state_message = match self.board.get_game_state() {
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

}
