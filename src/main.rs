use std::collections::HashSet;
use std::fmt;
//use ndarray::Array2;

//#[derive(Debug)]
//enum Value {
//    One,
//    Two,
//    Three,
//    Four,
//    Five,
//    Six,
//    Seven,
//    Eight,
//    Nine,
//}

//#[derive(Debug)]
//enum Square {
//    Uncertain(Vec<Value>),
//    Certain(Value),
//}

//impl Default for Square {
//    fn default() -> Self {
//        Self::Uncertain(vec![
//            Value::One,
//            Value::Two,
//            Value::Three,
//            Value::Four,
//            Value::Five,
//            Value::Six,
//            Value::Seven,
//            Value::Eight,
//            Value::Nine,
//        ])
//    }
//}
//

#[derive(Debug)]
enum Square {
    Certain(u8),
    Uncertain(Vec<u8>),
}

impl From<u8> for Square {
    fn from(n: u8) -> Self {
        match n {
            (1..=9) => Self::Certain(n),
            _ => Self::default(),
        }
    }
}

impl Square {
    fn eliminate_number(&mut self, n: u8){
        match self {
            Self::Certain(_) => {},
            Self::Uncertain(ref mut ns) => {
                ns.retain(|&x| x != n);

                if ns.len() == 1 {
                    *self = Self::Certain(ns[0]);
                }
            }
        }
    }
}
impl Default for Square {
    fn default() -> Self {
        Self::Uncertain(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Certain(x) => format!("{x}"),
            Self::Uncertain(_) => String::from("■"),
        };
        write!(f, "{s}")
    }
}


#[derive(Debug, Default)]
struct Sudoku {
    //   board: Array2<Square>,
    board: [[Square; 9]; 9],
}
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.board {
            for s in row {
                write!(f, "{s}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Sudoku {
    /// assumes that the number inserted is correct
    /// i.e. like one of the starting numbers
    /// 0-8
    fn insert_unchecked(&mut self, n: u8, row: usize, col: usize){
        if !(1..=9).contains(&n) || !(0..=8).contains(&row) || !(0..=8).contains(&col){
            println!("invalid input, ignoring...");
            return;
        }


        self.board[row][col] = Square::Certain(n);
        self.remove_n_from_row(n, row);
        self.remove_n_from_col(n, col);
        self.remove_n_from_cell(n, row, col);

        println!("{self}");
    }

    fn remove_n_from_row(&mut self, n: u8, row: usize){
       for r in &mut self.board[row] {
           r.eliminate_number(n);
       }
    }

    fn remove_n_from_col(&mut self, n: u8, col: usize){
        for row in &mut self.board{
            row[col].eliminate_number(n);
        }
    }


    fn remove_n_from_cell(&mut self, n: u8, row: usize, col: usize){

        let ranges = [(0..=2), (3..=5), (6..=8)];

        //dbg!(&ranges, &n, &row, &col);

        let rows = ranges.iter().filter(|r| r.contains(&row)).next().unwrap().clone();
        let cols = ranges.into_iter().filter(|r| r.contains(&col)).next().unwrap();



        for r in rows {
            // need to clone for each iteration of row
            for c in cols.clone() {
                self.board[r][c].eliminate_number(n);
                //dbg!(r, c);
            }
        }

    }
}

fn main() {
    let mut s = Sudoku::default();

    let b = [
        [7, 8, 0, 4, 0, 0, 1, 2, 0],
        [6, 0, 0, 0, 7, 5, 0, 0, 9],
        [0, 0, 0, 6, 0, 1, 0, 7, 8],
        [0, 0, 7, 0, 4, 0, 2, 6, 0],
        [0, 0, 1, 0, 5, 0, 9, 3, 0],
        [9, 0, 4, 0, 6, 0, 0, 0, 5],
        [0, 7, 0, 3, 0, 0, 0, 1, 2],
        [1, 2, 0, 0, 0, 7, 4, 0, 0],
        [0, 4, 9, 2, 0, 6, 0, 0, 7]
    ];

    for (r, row) in b.iter().enumerate(){
        for (c, &n) in row.iter().enumerate(){
            //println!("{r}, {c}, {n}");
            s.insert_unchecked(n, r, c);
        }
    }

//    let before = Sudoku { board: b };
//    println!("{before}");
    println!("{s}");
}
