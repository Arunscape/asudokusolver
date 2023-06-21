use std::fmt;

use bit_set::BitSet;


#[derive(Debug, Default)]
struct Sudoku {
    //   board: Array2<Square>,
    board: [[u8; 9]; 9],
    rows: BitSet,
    cols: BitSet,
    cells: BitSet,
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
//            s.insert_unchecked(n, r, c);
        }
    }

//    let before = Sudoku { board: b };
//    println!("{before}");
    println!("{s}");


    //dbg!(&s);
}
