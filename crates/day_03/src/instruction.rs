use std::iter::IntoIterator;
use std::str::FromStr;
use std::sync::LazyLock;

use anyhow::{Context, Error};
use regex::Regex;

/// A valid `mul` operation is defined as __exactly__ "mul(" + up to 3 digits + "," + up to 3 digits + ")".
static MUL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("`mul` regex should be a valid pattern")
});

/// Matches exactly "do()"
static DO_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"do\(\)").expect("`do` regex should be a valid pattern"));

/// Matches exactly "don't()"
static DONT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"don't\(\)").expect("`don't` regex should be a valid pattern"));

pub enum Instruction {
    Mul(u16, u16),
    Do,
    Dont,
}

/// Wrapper around a list of instructions.
///
/// Exists so that we can implement [`FromStr`] and parse a list of instructions from a string
/// instead of a single one.
pub struct Instructions(Vec<Instruction>);

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dos = DO_REGEX
            .find_iter(s)
            .map(|val| Ok((val.start(), Instruction::Do)));

        let donts = DONT_REGEX
            .find_iter(s)
            .map(|val| Ok((val.start(), Instruction::Dont)));

        let muls = MUL_REGEX
            .captures_iter(s)
            .map(|cap| -> Result<_, Self::Err> {
                let lhs = cap
                    .get(1)
                    .context("Capture should contain the LHS number")?;
                let rhs = cap.get(2).expect("Capture should contain the RHS number");

                let lhs_val = lhs
                    .as_str()
                    .parse()
                    .context("LHS should be a valid integer")?;
                let rhs_val = rhs
                    .as_str()
                    .parse()
                    .context("RHS should be a valid integer")?;

                Ok((lhs.start(), Instruction::Mul(lhs_val, rhs_val)))
            });

        let mut ops = dos
            .chain(donts)
            .chain(muls)
            .collect::<Result<Vec<_>, _>>()?;

        ops.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(Self(ops.into_iter().map(|(_, op)| op).collect()))
    }
}

/// Propagate inner Vec's iterator
impl IntoIterator for Instructions {
    type Item = Instruction;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
