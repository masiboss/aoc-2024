use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
#[allow(dead_code)]
const TEST: Testdata = Testdata {
    input: "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
",
    test_result_1: 143,
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

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let lines = reader.lines().map_while(Result::ok);
    let answer = 0;
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
