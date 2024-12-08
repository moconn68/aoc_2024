use std::path::Path;

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";

const DELTA: std::ops::RangeInclusive<i32> = 1..=3;

fn main() {
    println!("===== DAY 02 =====");

    if let Err(e) = runner() {
        eprintln!("ERROR: {:#?}", e);
    }
}

fn runner() -> Result<()> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let raw_input = std::fs::read_to_string(input_path)?;
    let input = parse_input(raw_input.lines())?;

    let ans_one = part_one(&input);
    println!("Part 1:\t{}", ans_one);

    let ans_two = part_two(&input);
    println!("Part 2:\t{}", ans_two);

    Ok(())
}

/// Utils candidate (via trait)
fn parse_input<T: AsRef<str>>(raw_input: impl IntoIterator<Item = T>) -> Result<Vec<Vec<u8>>> {
    Ok(raw_input
        .into_iter()
        .map(|line| line.as_ref().split_whitespace().map(str::parse).collect())
        .collect::<Result<Vec<_>, _>>()?)
}

fn part_one<T: AsRef<[u8]>>(input: &[T]) -> usize {
    input
        .iter()
        .filter(|report| evaluate_report_safety(report.as_ref()))
        .count()
}

fn part_two<T: AsRef<[u8]>>(input: &[T]) -> usize {
    let (safe, not_safe): (Vec<_>, Vec<_>) = input
        .iter()
        .partition(|report| evaluate_report_safety(report.as_ref()));

    let damped_safe = not_safe.iter().filter(|report| {
        let mut new_safe = false;
        let report = report.as_ref();
        for i in 0..report.len() {
            let row = [&report[0..i], &report[i + 1..report.len()]].concat();
            if evaluate_report_safety(&row) {
                new_safe = true;
                break;
            }
        }
        new_safe
    });

    safe.iter().chain(damped_safe).count()
}

/// Evaluates whether a given row is considered "safe" or not.
fn evaluate_report_safety(report: &[u8]) -> bool {
    let mut safe = true;
    let mut dir: Option<bool> = None;
    for window in report.as_ref().windows(2) {
        let diff = i32::from(window[0]) - i32::from(window[1]);

        match dir {
            Some(d) => {
                if diff.is_positive() != d {
                    safe = false;
                    break;
                }
            }
            None => dir = Some(diff.is_positive()),
        }

        if !DELTA.contains(&diff.abs()) {
            safe = false;
            break;
        }
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_part_one() {
        let expected = 2;

        let input = parse_input(TEST_INPUT.lines()).unwrap();
        let actual = part_one(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 4;

        let input = parse_input(TEST_INPUT.lines()).unwrap();
        let actual = part_two(&input);
        assert_eq!(expected, actual);
    }
}
