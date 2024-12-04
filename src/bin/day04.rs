use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04"; // TODO: Fill the day
#[allow(dead_code)]
const TEST: Testdata = Testdata {
    input: "\
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
",
    test_result_1: 18,
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

#[derive(Clone, Copy, PartialEq)]
enum Xmas {
    X,
    M,
    A,
    S,
}

const XMAS_WORD: [Xmas; 4] = [Xmas::X, Xmas::M, Xmas::A, Xmas::S];

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    // should step through the field with for searches, 4 for the traigt directions anf for 4 the diagonals
    let grid: Vec<Vec<Xmas>> = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|c| match c {
                    'X' => Some(Xmas::X),
                    'M' => Some(Xmas::M),
                    'A' => Some(Xmas::A),
                    'S' => Some(Xmas::S),
                    _ => None,
                })
                .collect_vec()
        })
        .collect();

    let mut found_xmas: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let directions: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut positions = Vec::new();
                let mut valid = true;

                for i in XMAS_WORD {
                    let nr = row as isize + i as isize * dr;
                    let nc = col as isize + i as isize * dc;

                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let nr = nr as usize;
                        let nc = nc as usize;

                        if grid[nr][nc] == i {
                            positions.push((nr, nc));
                        } else {
                            valid = false;
                            break;
                        }
                    } else {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    found_xmas.insert((positions[0], positions[3]));
                }
            }
        }
    }

    let answer = found_xmas.len();
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
