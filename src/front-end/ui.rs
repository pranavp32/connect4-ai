use yew::prelude::*;

mod board_array;

use board_array::{ArrayBoard, Cell};

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
        ShouldRender::Yes
    }

    fn view(&self) -> Html {
        html! {
            <div class="connect4">
                { for (0..HEIGHT).map(|row| self.render_row(row)) }
                <div class="buttons">
                    { for (0..WIDTH).map(|column| self.render_button(column)) }
                </div>
            </div>
        }
    }
}

impl Connect4 {
    fn render_row(&self, row: usize) -> Html {
        html! {
            <div class="row">
                { for (0..WIDTH).map(|column| self.render_cell(row, column)) }
            </div>
        }
    }

    fn render_cell(&self, row: usize, column: usize) -> Html {
        let index = WIDTH * (HEIGHT - 1 - row) + column;
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

    fn render_button(&self, column: usize) -> Html {
        html! {
            <button onclick=self.link.callback(move |_| Msg::ColumnClicked(column))>
                {"Drop"}
            </button>
        }
    }
}
