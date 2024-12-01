use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use advent_of_code_2024_rust::*;

const DAY: &str = "NN"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
<TEST-INPUT>
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    // TODO: Set the expected answer for the test input
    assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);
    
    println!("test passed");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let answer = contents.lines().count();
        Ok(answer)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
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
