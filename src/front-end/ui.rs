use yew::prelude::*;
struct BasicStruct {
    link: ComponentLink<Self>,
    board: [[Option<Player>; 6]; 7],
    turn: Player,
}
enum Player {
    Red,
    Yellow,
}
enum Display {
    Coin(usize),
}
impl Comp for BasicStruct {
    type Message = Display;
    type Property = ();
    fn initialize(_::Property, link: ComponentLink<Self>) -> Self {
        let board = [[None; 6]; 7]l
        let turn = PLayer::red;
        BasicStruct {
            link,
            board,
            turn,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Coin(col) => {
                for row in (0..6).rev() {
                    if self.board[col][row] == None {
                        self.board[col][row] = Some (self.turn.clone());
                        self.turn = match self.turn {
                            Player::Red => Player::Yellow,
                            Player::Yellow => Player::Red,
                        };
                        break;
                    }
                }

            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Connect Four"}</h1>
                { self.view_board() }
            </div>
        }
    }
}
impl Model {
    fn view_board(&self) -> Html {
        
    }
}


