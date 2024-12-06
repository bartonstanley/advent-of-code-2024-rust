use std::fs::File;
use anyhow::*;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use advent_of_code_2024_rust::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)")?;
        let captures = re.captures_iter(&contents).collect::<Vec<_>>();
        let mut sum = 0;
        for capture in captures {
            let pair = capture.get(1).unwrap().as_str().split(",").collect::<Vec<_>>();
            sum = sum + (pair[0].parse::<usize>()? * pair[1].parse::<usize>()?);
        }
        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    println!("test passed");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let re = Regex::new(r"mul\(([0-9]+,[0-9]+)\)|do\(\)|don't\(\)")?;
        let captures = re.captures_iter(&contents).collect::<Vec<_>>();
        let mut sum = 0;
        let mut summation_enabled = true;
        for capture in captures {
            let expression = capture.get(0).unwrap().as_str().split(",").collect::<Vec<_>>()[0];
            if expression.starts_with("mul") && summation_enabled {
                let pair = capture.get(1).unwrap().as_str().split(",").collect::<Vec<_>>();
                sum = sum + (pair[0].parse::<usize>()? * pair[1].parse::<usize>()?);
            }
            if expression.starts_with("do(")  {
                summation_enabled = true;
            }
            if expression.starts_with("don't(")  {
                summation_enabled = false;
            }
        }
        Ok(sum)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    
    println!("test passed");
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
