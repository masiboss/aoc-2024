use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::{Captures, RegexSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
#[allow(dead_code)]
const TEST: Testdata = Testdata {
    input: "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
",
    test_result_1: 161,
    test_result_2: 0,
};
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[allow(dead_code)]
struct Testdata {
    input: &'static str,
    test_result_1: usize,
    test_result_2: usize,
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result1 = time_snippet!(part1(input_file)?);
    println!("Result = {}", result1);

    println!("\n=== Part 2 ===");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result2 = time_snippet!(part2(input_file)?);
    println!("Result = {}", result2);

    Ok(())
}

fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let regex = regex::Regex::new(r"mul\((\d+),\s*(\d+)\)")?;
    let answer = regex
        .captures_iter(&buf)
        .map(|capture: Captures| {
            let a = capture[1].parse::<usize>().unwrap();
            let b = capture[2].parse::<usize>().unwrap();
            a * b
        })
        .sum();
    Ok(answer)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let answer = reader.lines().map_while(Result::ok).count();
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
