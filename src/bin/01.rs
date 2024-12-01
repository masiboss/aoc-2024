use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[allow(dead_code)]
struct Testdata {
    input: &'static str,
    test_result_1: usize,
    test_result_2: usize,
}

#[allow(dead_code)] //used in tests
const TEST: Testdata = Testdata {
    input: "\
3   4
4   3
2   5
1   3
3   9
3   3
",
    test_result_1: 11,
    test_result_2: 31,
};

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| -> (usize, usize) {
            let mut nums = line.split_whitespace().map(|n| n.parse::<usize>().ok());
            let a = nums.next().flatten();
            let b = nums.next().flatten();
            (a.unwrap(), b.unwrap())
        })
        .unzip();
    left.sort();
    right.sort();
    let answer: usize = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let (left, right): (Vec<usize>, Vec<usize>) = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| -> (usize, usize) {
            let mut nums = line.split_whitespace().map(|n| n.parse::<usize>().ok());
            let a = nums.next().flatten();
            let b = nums.next().flatten();
            (a.unwrap(), b.unwrap())
        })
        .unzip();
    let right = right.iter().counts();
    let answer: usize = left.iter().map(|a| a * right.get(&a).unwrap_or(&0)).sum();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            TEST.test_result_1,
            time_snippet!(part1(BufReader::new(TEST.input.as_bytes())).unwrap())
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            TEST.test_result_2,
            time_snippet!(part2(BufReader::new(TEST.input.as_bytes())).unwrap())
        );
    }
}
