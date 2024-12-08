use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

const INPUT_FILE: &str = "input.txt";

/// A valid `mul` operation is defined as __exactly__ "mul(" + up to 3 digits + "," + up to 3 digits + ")".
static MUL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("`mul` regex should be a valid pattern")
});

/// Container for a `mul` instruction.
struct MulInstr(u16, u16);

impl MulInstr {
    /// Parses all valid `mul` instructions that exist in a given line.
    pub fn parse_line(line: &str) -> Vec<Self> {
        MUL_REGEX
            .captures_iter(line)
            .map(|captures| {
                let lhs = captures
                    .get(1)
                    .expect("Capture should contain the LHS number")
                    .as_str();
                let rhs = captures
                    .get(2)
                    .expect("Capture should contain the RHS number")
                    .as_str();

                let lhs = lhs.parse().expect("LHS should be a valid integer");
                let rhs = rhs.parse().expect("RHS should be a valid integer");

                Self(lhs, rhs)
            })
            .collect()
    }

    /// Executes the underlying instruction and returns its result.
    pub fn execute(self) -> u32 {
        (self.0 as u32) * (self.1 as u32)
    }
}

fn main() {
    println!("===== DAY 03 =====");

    if let Err(e) = runner() {
        eprintln!("ERROR: {:#?}", e);
    }
}

fn runner() -> Result<()> {
    let input_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(INPUT_FILE);
    let raw_input = std::fs::read_to_string(input_path)?;

    let ans_one = part_one(raw_input.lines());
    println!("Part one:\t{}", ans_one);

    Ok(())
}

fn part_one<Line: AsRef<str>>(input: impl IntoIterator<Item = Line>) -> u32 {
    input
        .into_iter()
        .flat_map(|line| {
            MulInstr::parse_line(line.as_ref())
                .into_iter()
                .map(MulInstr::execute)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

    #[test]
    fn test_part_one() {
        let expected = 161;
        let actual = part_one(TEST_INPUT.lines());
        assert_eq!(expected, actual);
    }
}
