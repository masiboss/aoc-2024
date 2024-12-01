use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        // TODO: Solve Part 1 of the puzzle
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| -> (u32, u32) {
                let mut nums = line.split_whitespace().map(|n| n.parse::<u32>().ok());
                let a = nums.next().flatten();
                let b = nums.next().flatten();
                (a.unwrap(), b.unwrap())
            })
            .unzip();
        left.sort();
        right.sort();
        let answer: u32 = left
            .iter()
            .zip(right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);\

    Ok(())
}
