use advent_of_code_2024_rust::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::ops::Mul;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<(), Box<dyn Error>> {
    start_day(DAY);

    println!("=== Part 1 ===");

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    println!("test passed");

    fn read_to_trimmed_string<R: BufRead>(reader: &mut R) -> Result<String, Box<dyn Error>> {
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;

        Ok(String::from(contents.trim()))
    }

    fn parse(value: &str) -> Result<usize, Box<dyn Error>> {
        let result = value.parse::<usize>();
        match result {
            Ok(value) => Ok(value),
            _ => Err(format!("Invalid input: {}", value).into()),
        }
    }

    fn read_into_sorted_vectors<R: BufRead>(
        reader: &mut R,
    ) -> Result<(Vec<usize>, Vec<usize>), Box<dyn Error>> {
        let mut list_a: Vec<usize> = vec![];
        let mut list_b: Vec<usize> = vec![];
        let contents = read_to_trimmed_string(reader)?;

        for line in contents.lines() {
            let pair = line.split_whitespace().collect::<Vec<&str>>();
            list_a.push(parse(&pair[0])?);
            list_b.push(parse(&pair[1])?);
        }

        list_a.sort();
        list_b.sort();

        Ok((list_a, list_b))
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize, Box<dyn Error>> {
        let (list_a, list_b) = read_into_sorted_vectors(&mut reader)?;

        let answer = zip(list_a, list_b).fold(0, |acc, pair| acc + pair.0.abs_diff(pair.1));

        Ok(answer)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize, Box<dyn Error>> {
        let (list_a, list_b) = read_into_sorted_vectors(&mut reader)?;

        let answer = list_a.iter().fold(0, |acc, a| {
            acc + a.mul(list_b.iter().filter(|b| a == *b).count())
        });

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    println!("test passed");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
