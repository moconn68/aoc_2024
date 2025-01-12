use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use anyhow::{Context, Result};
use itertools::Itertools;
use strum::VariantArray;

#[derive(Clone, Copy, VariantArray)]
pub enum Operator {
    Add,
    Mult,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Mult => "*",
            }
        )
    }
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}

impl Operator {
    /// Performs the operation described by the associated operator, using provided integer
    /// values for the left hand side and right hand side. Returns the result of the operation.
    pub fn operate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Add => lhs + rhs,
            Self::Mult => lhs * rhs,
        }
    }

    /// Generates all possible orderings for operators where `num` is the number of operators.
    ///
    /// For example, if `num` = 1, the result would be `[[Operator::Add], [Operator::Mult]]`
    ///
    /// And for `num` = 3:
    /// ```
    /// [
    ///     [Operator::Add, Operator::Add, Operator::Add],
    ///     [Operator::Add, Operator::Add, Operator::Mult],
    ///     [Operator::Add, Operator::Mult, Operator::Add],
    ///     [Operator::Add, Operator::Mult, Operator::Mult],
    ///     [Operator::Mult, Operator::Add, Operator::Add],
    ///     [Operator::Mult, Operator::Add, Operator::Mult],
    ///     [Operator::Mult, Operator::Mult, Operator::Add],
    ///     [Operator::Mult, Operator::Mult, Operator::Mult],
    /// ]
    /// ```
    pub fn generate_orderings(num: usize) -> Vec<Vec<Self>> {
        if num == 0 {
            return vec![vec![]];
        }

        let mut orderings = vec![];
        for el in Self::VARIANTS.iter().copied() {
            let sub_orderings = Self::generate_orderings(num - 1);
            for mut sub in sub_orderings {
                sub.push(el);
                orderings.push(sub);
            }
        }
        orderings
    }
}

/// A simple math expression represented by sets of numbers and operators.
pub struct Expression<O = Vec<Operator>> {
    nums: VecDeque<usize>,
    operators: O,
}

impl<O> Expression<O> {
    pub const fn new(nums: VecDeque<usize>, operators: O) -> Self {
        Self { nums, operators }
    }
}

impl<O: AsRef<[Operator]>> Expression<O> {
    /// Evaluate the expression from right to left, using only the order of operators as precedence.
    ///
    /// Returns the value of the expression, or an error if the expression is not valid.
    pub fn evaluate(&mut self) -> Result<usize> {
        for op in self.operators.as_ref() {
            let lhs = self.nums.pop_front().context("Missing LHS")?;
            let rhs = self.nums.pop_front().context("Missing RHS")?;
            let eval = op.operate(lhs, rhs);
            self.nums.push_front(eval);
        }

        self.nums
            .pop_front()
            .context("No numbers left in queue at end of evaluation")
    }
}

impl<O: AsRef<[Operator]>> Display for Expression<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            self.nums
                .iter()
                .map(|num| num as &dyn Display)
                .interleave(self.operators.as_ref().iter().map(|op| op as &dyn Display))
                .join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case([3, 4], [Operator::Add], 7)]
    #[case([1, 2, 3, 4], [Operator::Mult, Operator::Add, Operator::Add], 9)]
    #[case([5, 1, 2, 4, 3], [Operator::Add, Operator::Mult, Operator::Add, Operator::Mult], 48)]
    #[case([6, 2, 3, 4], [Operator::Add, Operator::Mult, Operator::Add], 28)]
    fn expression_evaluate(
        #[case] nums: impl Into<VecDeque<usize>>,
        #[case] ops: impl Into<Vec<Operator>>,
        #[case] expected: usize,
    ) {
        let mut expression = Expression::new(nums.into(), ops.into());
        assert_eq!(expected, expression.evaluate().unwrap());
    }
}
