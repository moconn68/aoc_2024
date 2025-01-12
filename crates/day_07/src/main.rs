mod math;

use crate::math::{Expression, Operator};

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{Context, Result};

const INPUT_FILE: &str = "input.txt";

type InputLine = (usize, Vec<usize>);

fn main() {
    if let Err(e) = runner() {
        eprintln!("ERROR: {}", e);
    }
}

fn runner() -> Result<()> {
    println!("===== DAY 07 =====");

    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let input_reader = BufReader::new(File::open(&input_path)?);
    let input = parse_input(input_reader)?;

    let ans_one = part_one(input)?;
    println!("Part one:\t{}", ans_one);

    Ok(())
}

fn parse_input(input_reader: impl BufRead) -> Result<Vec<InputLine>> {
    input_reader
        .lines()
        .map(|line| {
            let line = line?;
            let (test_value, equation) = line
                .split_once(':')
                .context("Equation should be split via colon")?;

            let equation = equation
                .split_whitespace()
                .map(|el| el.parse())
                .collect::<Result<_, _>>()?;

            Ok((test_value.parse()?, equation))
        })
        .collect()
}

fn part_one(input: impl IntoIterator<Item = InputLine>) -> Result<usize> {
    let mut sum = 0;
    for (target_value, nums) in input {
        let op_combos = Operator::generate_orderings(nums.len() - 1);

        let mut matched = false;
        for ops in op_combos {
            let mut expr = Expression::new(nums.clone().into(), ops);
            let ans = expr.evaluate()?;
            if ans == target_value {
                matched = true;
                break;
            }
        }
        if matched {
            sum += target_value;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_one() {
        let expected = 3749;

        let input = parse_input(TEST_INPUT.as_bytes()).unwrap();

        let actual = part_one(input).unwrap();
        assert_eq!(expected, actual);
    }
}
