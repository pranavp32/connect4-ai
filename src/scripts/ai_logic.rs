fn minimax(turn: mut &bool, alpha: &i32, beta: &i32, p1: &u64, p2:&u64, empty:&u64 depth: &u32, row: &u64, col: &u64) {
    if turn {
        let position = row*7 + col + 1;
        let digit = pow(10, position);
        let value = board % digit;
        if value == 00 {
            turn = !turn;
            board ;
        }
    }
}

fn gameover() -> bool {

}

fn place(board: &u64) -> u64{
    
}