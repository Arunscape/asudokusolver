use bit_set::BitSet;
use std::{fmt, str::FromStr};

#[derive(Debug)]
pub struct Sudoku {
    //   board: Array2<Square>,
    board: [[u8; 9]; 9],
    rows: [BitSet; 9],
    cols: [BitSet; 9],
    cells: [BitSet; 9],
}
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // for i in 0..9 {
        //     if i % 3 == 0 {
        //         write!(f, "+-------+-------+-------+\n")?;
        //     }
        //     for j in 0..9 {
        //         if j % 3 == 0 {
        //             print!("| ");
        //         }

        //         match self.board[i][j] {
        //             0 => write!(f, "  ")?,
        //             n => write!(f, "{} ", n)?,
        //         }

        //         // write!(f, "{} ", self.board[i][j]);
        //     }
        //     write!(f, "|\n")?;
        // }
        // write!(f, "+-------+-------+-------+\n")?;
        // Ok(())

        let s = Self::board_to_string(self.board);
        write!(f, "{s}")?;
        Ok(())
    }
}

impl FromStr for Sudoku {
    type Err = ();

    /// Parses a string of 81 digits into a Sudoku board.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 81 {
            dbg!(s.len());
            return Err(());
        }

        let mut board = [[0; 9]; 9];

        for (i, n) in s.chars().enumerate().flat_map(|(i, c)| {
            match c.to_digit(10) {
                Some(n) => Some((i, n as u8)),
                None => Some((i, 0)),
            }
        }) {
            let row = i / 9;
            let col = i % 9;
            board[row][col] = n;
        }

        let board = Sudoku::new(board);

        Ok(board)
    }
}
impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        // could have made a macro here oh well
        let mut rows = [
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
        ];
        let mut cols = [
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
        ];
        let mut cells = [
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
            BitSet::with_capacity(9),
        ];
        for (r, row) in board.iter().enumerate() {
            for (c, &n) in row.iter().enumerate() {
                //println!("{r}, {c}, {n}");

                let cell = Self::get_cell(r, c);
                if n != 0 {
                    let n = n as usize;
                    rows[r].insert(n);
                    cols[c].insert(n);
                    cells[cell].insert(n);
                }
            }
        }

        Self {
            board,
            rows,
            cols,
            cells,
        }
    }

    pub fn get_cell(row: usize, col: usize) -> usize {
        // let cell = match (r, c) {
        //     (0..=2, 0..=2) => 0,
        //     (0..=2, 3..=5) => 1,
        //     (0..=2, 6..=8) => 2,
        //     (3..=5, 0..=2) => 3,
        //     (3..=5, 3..=5) => 4,
        //     (3..=5, 6..=8) => 5,
        //     (6..=8, 0..=2) => 6,
        //     (6..=8, 3..=5) => 7,
        //     (6..=8, 6..=8) => 8,
        //     _ => panic!("Invalid row/col"),
        // };
        (row / 3) * 3 + col / 3
    }

    pub fn solve_all_solutions(&mut self) -> Vec<[[u8; 9]; 9]> {
        let mut solutions = vec![];
        self.solve(0, 0, &mut solutions);
        solutions
    }

    fn solve(&mut self, row: usize, col: usize, solutions: &mut Vec<[[u8; 9]; 9]>) {
        if 8 < row {
            solutions.push(self.board);
            return;
        }

        let next_row = match col {
            8 => row + 1,
            _ => row,
        };
        let next_col = (col + 1) % 9;

        if self.board[row][col] != 0 {
            self.solve(next_row, next_col, solutions);
            return;
        }

        let cell = Self::get_cell(row, col);
        for i in 1..=9 {
            if !self.rows[row].contains(i)
                && !self.cols[col].contains(i)
                && !self.cells[cell].contains(i)
            {
                self.rows[row].insert(i);
                self.cols[col].insert(i);
                self.cells[cell].insert(i);
                self.board[row][col] = i as u8;

                self.solve(next_row, next_col, solutions);

                self.rows[row].remove(i);
                self.cols[col].remove(i);
                self.cells[cell].remove(i);
                self.board[row][col] = 0;
            }
        }
    }

    pub fn board_to_string(board: [[u8; 9]; 9]) -> String {
        let mut s = String::with_capacity(338);
        for i in 0..9 {
            if i % 3 == 0 {
                s.push_str("+-------+-------+-------+\n");
            }
            for j in 0..9 {
                if j % 3 == 0 {
                    s.push_str("| ");
                }

                match board[i][j] {
                    0 => s.push_str("  "),
                    n => s.push_str(&format!("{} ", n)),
                }
            }
            s.push_str("|\n");
        }
        s.push_str("+-------+-------+-------+\n");

        // dbg!(&s.len());
        s
    }
}