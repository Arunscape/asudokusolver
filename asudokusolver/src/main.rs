mod sudoku;
use sudoku::Sudoku;
use std::{io, str::FromStr};


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut board = Sudoku::from_str(input.trim()).unwrap();
    println!("INPUT: ");
    println!("{board}");

    println!("OUTPUT:");
    let solutions = board.solve_all_solutions();

    for s in solutions {
        println!("{}", Sudoku::board_to_string(s));
    }

    //dbg!(&s);
}
