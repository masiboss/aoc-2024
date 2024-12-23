use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
#[allow(dead_code)]
const TEST: Testdata = Testdata {
    input: "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
    test_result_1: 2,
    test_result_2: 4,
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

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let answer = reader
        .lines()
        .map_while(Result::ok)
        .map(|record| {
            let values: Vec<isize> = record
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect();
            is_safe(values)
        })
        .filter(|&x| x)
        .count();
    Ok(answer)
}

fn is_safe(values: Vec<isize>) -> bool {
    values
        .clone()
        .into_iter()
        .zip(values.clone().into_iter().skip(1))
        .all(|(a, b)| a != b && a.abs_diff(b) <= 3)
        && (values.clone().is_sorted() || values.into_iter().rev().is_sorted())
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let answer = reader
        .lines()
        .map_while(Result::ok)
        .map(|record| {
            record
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect_vec()
        })
        .map(|report| {
            let safe = is_safe(report.clone());
            if safe {
                return true;
            }
            for i in 0..report.len() {
                if is_safe(
                    report
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(index, _)| *index != i)
                        .map(|(_, x)| x)
                        .collect(),
                ) {
                    return true;
                }
            }
            false
        })
        .filter(|&x| x)
        .count();
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
