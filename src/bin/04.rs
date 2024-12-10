// use std::error::Error;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024_rust::*;
use crate::ScannerState::{AwaitingX, AwaitingM, AwaitingA, AwaitingS, Found};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[derive(Debug)]
enum ScannerState {
    AwaitingX,
    AwaitingM,
    AwaitingA,
    AwaitingS,
    Found,
}

struct Scanner {
    pub number_of_occurrences_found: usize,
    forward_state: ScannerState,
    backward_state: ScannerState,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            number_of_occurrences_found: 0,
            forward_state: AwaitingX,
            backward_state: AwaitingS,
        }
    }

    fn accept_input(&mut self, c: char) -> Result<usize> {
        let prev_number_of_occurrences = self.number_of_occurrences_found;
        // print!("from {:?}", &self.forward_state);
        self.forward_state = match c {
            'X' => AwaitingM,
            'M' => if let AwaitingM = &self.forward_state {AwaitingA} else {AwaitingX},
            'A' => if let AwaitingA = &self.forward_state {AwaitingS} else {AwaitingX},
            'S' => if let AwaitingS = &self.forward_state {Found} else {AwaitingX},
            _ => AwaitingX,
        };
        // println!(" to {:?}", &self.forward_state);

        if let Found = &self.forward_state {
            self.number_of_occurrences_found += 1;
            self.forward_state = AwaitingX;
        }

        self.backward_state = match c {
            'S' => AwaitingA,
            'A' => if let AwaitingA = &self.backward_state {AwaitingM} else {AwaitingS},
            'M' => if let AwaitingM = &self.backward_state {AwaitingX} else {AwaitingS},
            'X' => if let AwaitingX = &self.backward_state {Found} else {AwaitingS},
            _ => AwaitingS,
        };

        if let Found = &self.backward_state {
            self.number_of_occurrences_found += 1;
            self.backward_state = AwaitingS;
        }

        Ok(self.number_of_occurrences_found - prev_number_of_occurrences)
    }

    fn reset(&mut self) {
        // self.number_of_occurrences_found = 0;
        self.forward_state = AwaitingX;
        self.backward_state = AwaitingS;
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut matrix: Vec<Vec<char>> = vec![];
        let answer = reader.lines();
        for line in answer {
            matrix.push(line?.chars().collect::<Vec<char>>());
        }
        let number_of_rows = matrix.len();
        let number_of_cols = matrix[0].len();

        // rows
        let mut scanner = Scanner::new();
        for r in 0..number_of_rows {
            for c in 0..number_of_cols {
                let number_found = scanner.accept_input(matrix[r][c])?;
                if number_found > 0 {
                    println!("found row-wise XMAS or SAMX ending at: ({r}, {c})");
                }
            }
            scanner.reset();
        }

        // columns
        scanner.reset();
        for c in 0..number_of_cols {
            scanner.reset();
            for r in 0..number_of_rows {
                let number_found = scanner.accept_input(matrix[r][c])?;
                if number_found > 0 {
                    println!("found column-wise XMAS or SAMX ending at: ({r}, {c})");
                }
            }
        }

        // left to right diagonal, 2nd half
        // scanner.reset();
        // for bias in 0..number_of_cols {
        //     scanner.reset();
        //     for i in 0..number_of_rows - bias {
        //         let number_found = scanner.accept_input(matrix[i][i+bias])?;
        //         if number_found > 0 {
        //             println!("found left-right diagonal XMAS or SAMX ending at: ({i}, {})", i+bias);
        //         }
        //     }
        // }
        // let mut number_found_on_left_to_right_diagonals = scanner.number_of_occurrences_found;

        // left to right diagonal, 1st half
        // scanner.reset();
        // for r in 1..number_of_rows {
        //     scanner.reset();
        //     for c in 0..number_of_cols - r {
        //         let number_found = scanner.accept_input(matrix[r][c])?;
        //         if number_found > 0 {
        //             println!("found left-right diagonal XMAS or SAMX ending at: ({r}, {c})");
        //         }
        //     }
        // }
        // number_found_on_left_to_right_diagonals += scanner.number_of_occurrences_found;
        // println!("number found on left-right diagonals {number_found_on_left_to_right_diagonals}");

        for diag in (0..(number_of_rows + number_of_cols - 1)).rev() {
            // determine the starting row index for the current diagonal
            let mut row = if diag < number_of_rows {0} else {diag - number_of_cols + 1};

            // determine the starting column index for the current diagonal
            let mut col: isize = if diag < number_of_rows {isize::try_from(diag)?} else {isize::try_from(number_of_cols - 1)?};

            scanner.reset();
            // collect all the elements from the current diagonal
            while row < number_of_rows && col >= 0 {
                let number_found = scanner.accept_input(matrix[row][usize::try_from(col)?])?;
                if number_found > 0 {
                    println!("found right-left diagonal XMAS or SAMX ending at: ({row}, {col})");
                }
                row += 1;
                col -= 1;
            }
        }

        println!("diag found {:?}.", &scanner.number_of_occurrences_found);

        // find out how many, then convert the below to do left right and see if there is a difference.

        for diag in 0..(number_of_rows + number_of_cols - 1) {
            // determine the starting row index for the current diagonal
            let mut row = if diag < number_of_rows {0} else {diag - number_of_cols + 1};

            // determine the starting column index for the current diagonal
            let mut col: isize = if diag < number_of_rows {isize::try_from(diag)?} else {isize::try_from(number_of_cols - 1)?};

            // collect all the elements from the current diagonal
            scanner.reset();
            while row < number_of_rows && col >= 0 {
                let number_found = scanner.accept_input(matrix[row][usize::try_from(col)?])?;
                if number_found > 0 {
                    println!("found right-left diagonal XMAS or SAMX ending at: ({row}, {col})");
                }
                row += 1;
                col -= 1;
            }
        }

        println!("diag rev found {:?}.", &scanner.number_of_occurrences_found);

        Ok(scanner.number_of_occurrences_found)
    }

    part1(BufReader::new(TEST.as_bytes()))?;
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    println!("test passed");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    // TODO: Solve Part 2 of the puzzle
    //     Ok(0)
    // }
    //
    // TODO: Set the expected answer for the test input
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // println!("test passed");
    // 
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);

    Ok(())
}
