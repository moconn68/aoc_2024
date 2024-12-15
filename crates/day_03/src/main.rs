mod instruction;

use crate::instruction::{Instruction, Instructions};

use std::path::Path;
use std::str::FromStr;

use anyhow::Result;

const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("===== DAY 03 =====");

    if let Err(e) = runner() {
        eprintln!("ERROR: {:#?}", e);
    }
}

fn runner() -> Result<()> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let raw_input = std::fs::read_to_string(input_path)?;

    let ans_one = part_one(raw_input.lines())?;
    println!("Part one:\t{}", ans_one);

    let ans_two = part_two(raw_input.lines())?;
    println!("Part two:\t{}", ans_two);

    Ok(())
}

fn part_one<Line: AsRef<str>>(input: impl IntoIterator<Item = Line>) -> Result<u32> {
    let mut sum = 0;

    for line in input {
        let instructions = Instructions::from_str(line.as_ref())?;

        let products = instructions.into_iter().filter_map(|instr| match instr {
            Instruction::Mul(lhs, rhs) => Some((lhs as u32) * (rhs as u32)),
            _ => None,
        });
        sum += products.sum::<u32>();
    }

    Ok(sum)
}

fn part_two<Line: AsRef<str>>(input: impl IntoIterator<Item = Line>) -> Result<u32> {
    let mut flag = true;
    let mut sum = 0;

    for line in input {
        let instructions = Instructions::from_str(line.as_ref())?;

        for inst in instructions {
            match inst {
                Instruction::Mul(lhs, rhs) => {
                    if flag {
                        sum += (lhs as u32) * (rhs as u32);
                    }
                }
                Instruction::Do => flag = true,
                Instruction::Dont => flag = false,
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = 161;

        let actual = part_one(input.lines()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = 48;

        let actual = part_two(input.lines()).unwrap();
        assert_eq!(expected, actual);
    }
}
