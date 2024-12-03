use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::{Captures, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03"; // TODO: Fill the day
#[allow(dead_code)]
const TEST: Testdata = Testdata {
    input: "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
",
    input2: "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
",
    test_result_1: 161,
    test_result_2: 48,
};
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[allow(dead_code)]
struct Testdata {
    input: &'static str,
    input2: &'static str,
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
    let mul = Regex::new(r"mul\((\d+),\s*(\d+)\)")?;
    let answer = mul
        .captures_iter(&buf)
        .map(|capture: Captures| {
            let a = capture[1].parse::<usize>().unwrap();
            let b = capture[2].parse::<usize>().unwrap();
            a * b
        })
        .sum();
    Ok(answer)
}

fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mul = Regex::new(r"mul\((\d+),\s*(\d+)\)")?;
    let do_regex = Regex::new(r"do\(\)")?;
    let dont_regex = Regex::new(r"don't\(\)")?;
    let answer = mul
        .captures_iter(&buf)
        .filter(|capture: &Captures| {
            let preceding_text = &buf[..capture.get(0).unwrap().start()];
            let last_do_idx = do_regex
                .find_iter(preceding_text)
                .last()
                .map(|m| m.end())
                .unwrap_or(0);
            let last_dont_idx = dont_regex
                .find_iter(preceding_text)
                .last()
                .map(|m| m.end())
                .unwrap_or(0);
            last_do_idx >= last_dont_idx
        })
        .map(|capture: Captures| {
            let a = capture[1].parse::<usize>().unwrap();
            let b = capture[2].parse::<usize>().unwrap();
            a * b
        })
        .sum();
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
            time_snippet!(part2(BufReader::new(TEST.input2.as_bytes())).unwrap())
        );
    }
}
