use std::collections::HashMap;

use anyhow::{anyhow, Result};

// Type for numbers used for the puzzle input and answers
type Num = u32;

// Path to the input data
const INPUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

fn main() {
    println!("===== DAY 01 =====");

    if let Err(e) = runner() {
        eprintln!("ERROR: {:#?}", e);
    }
}

fn runner() -> Result<()> {
    let raw_input = std::fs::read_to_string(INPUT_PATH)?;
    let (mut a, mut b) = parse_input(raw_input.lines())?;

    let ans_one = part_one(&mut a, &mut b);
    println!("Part 1:\t{}", ans_one);

    let ans_two = part_two(&a, &b);
    println!("Part 2: {:?}", ans_two);
    Ok(())
}

fn parse_input<T: AsRef<str>>(input: impl IntoIterator<Item = T>) -> Result<(Vec<Num>, Vec<Num>)> {
    input
        .into_iter()
        .map(|line| {
            let mut split = line.as_ref().split_whitespace();

            let a: Num = split
                .next()
                .ok_or_else(|| anyhow!("Input missing first number"))?
                .parse()?;

            let b: Num = split
                .next()
                .ok_or_else(|| anyhow!("Input missing second number"))?
                .parse()?;

            Ok((a, b))
        })
        .collect()
}

fn part_one(a: &mut [Num], b: &mut [Num]) -> Num {
    a.sort();
    b.sort();
    a.iter()
        .zip(b)
        .map(|(first, second)| first.abs_diff(*second))
        .sum()
}

fn part_two(a: &[Num], b: &[Num]) -> Num {
    let freq_map = b.iter().fold(HashMap::new(), |mut acc, el| {
        acc.entry(el).and_modify(|val| *val += 1).or_insert(1);
        acc
    });

    a.iter()
        .map(|num| num * freq_map.get(num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part_one() {
        let expected = 11;

        let (mut a, mut b) = parse_input(TEST_INPUT.lines()).unwrap();
        let actual = part_one(&mut a, &mut b);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 31;

        let (a, b) = parse_input(TEST_INPUT.lines()).unwrap();
        let actual = part_two(&a, &b);
        assert_eq!(expected, actual);
    }
}
