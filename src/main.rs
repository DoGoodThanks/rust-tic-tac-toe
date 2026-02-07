use ndarray::{Array2, ArrayBase, DataMut, Data, Ix2};

fn main() {
    let mut board = get_board(); // the board with each space being ' ', 'x', or 'o', as an ndarray Array2 / ArrayBase
    let is_human_x = decide_if_human_goes_first();
    let is_human_turn = is_human_x; // if human is x, they go first, otherwise robot is x and goes first

    draw_board(&board);
    play_game(is_human_turn, is_human_x, &mut board);
    }

fn get_board() -> Array2<char> {
    //let size_as_string = ask_for_size();
    //let (rows, columns) = convert_string_to_int();
    //let mut board = Array2::from_elem((rows, columns), ' ');

     Array2::from_elem((3, 3), ' ')
}

fn decide_if_human_goes_first() -> bool {
    true
}

fn draw_board<S: Data<Elem = char>>(board: &ArrayBase<S, Ix2>) {
    //board is a generic ndarray of any sort as long as it is 2d and stores char
    let (rows, columns) = board.dim();

    for r in 0..rows {   
        for c in 0..columns {

            print!("{}", board[[r, c]]);
            if c!=columns-1  {
                print!("|");
            } else {print!("\n");}

        }

        if r!=rows-1 {
            print!("-----\n");
        } else {print!("\n");}

    }
}

fn play_game<S: DataMut<Elem = char>>(mut is_human_turn: bool, is_human_x: bool, board: &mut ArrayBase<S, Ix2>) {
    while !game_over(&board)  {
         if is_human_turn  {
        get_human_move(is_human_x, &mut *board);
    } else {get_robot_move(is_human_x, &mut *board);}
    
    is_human_turn = !is_human_turn;
    draw_board(&board);

    }
}

fn get_human_move<S: DataMut<Elem = char>>(_is_human_x: bool, board: &mut ArrayBase<S, Ix2>) {
    let (rows, columns) = board.dim();

    for r in 0..rows {   
        for c in 0..columns {
            if board[[r, c]] == ' ' {
                board[[r, c]] = 'x';
                return;
            }
        }

    }
}

fn get_robot_move<S: DataMut<Elem = char>>(_is_human_x: bool, board: &mut ArrayBase<S, Ix2>) {
    let (rows, columns) = board.dim();

    for r in 0..rows {   
        for c in 0..columns {
            if board[[r, c]] == ' ' {
                board[[r, c]] = 'o';
                return;
            }
        }

    }
}

fn game_over<S: Data<Elem = char>>(board: &ArrayBase<S, Ix2>) -> bool {
    let (rows, columns) = board.dim();

    for r in 0..rows {   
        for c in 0..columns {
            if board[[r, c]] == ' ' {
                println!("still going...\n");
                return false;
            }
        }
    }
    println!("game over!\n");
    true
}