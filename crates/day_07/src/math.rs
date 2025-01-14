use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::marker::PhantomData;

use anyhow::{Context, Result};
use itertools::Itertools;

pub trait Variants {
    fn variants() -> impl Iterator<Item = Self>;
}

pub trait Operator {
    /// Perform the operation described by the associated operator, using provided integer
    /// values for the left hand side and right hand side. Returns the result of the operation.
    fn operate(&self, lhs: usize, rhs: usize) -> usize;
}

#[derive(Clone, Copy)]
pub enum Op {
    Add,
    Mult,
}

impl Variants for Op {
    fn variants() -> impl Iterator<Item = Self> {
        [Self::Add, Self::Mult].into_iter()
    }
}

impl Operator for Op {
    fn operate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Self::Add => lhs + rhs,
            Self::Mult => lhs * rhs,
        }
    }
}

impl Display for Op {
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

impl Debug for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{self}")
    }
}

#[derive(Clone, Copy)]
pub enum Op2 {
    Base(Op),
    Concat,
}

impl Variants for Op2 {
    fn variants() -> impl Iterator<Item = Self> {
        Op::variants()
            .map(Self::Base)
            .chain(std::iter::once(Self::Concat))
    }
}

impl Operator for Op2 {
    fn operate(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Op2::Base(op) => op.operate(lhs, rhs),
            Op2::Concat => {
                let right_digits = rhs.checked_ilog10().map(|x| x + 1).unwrap_or(1);
                let output = lhs * 10_usize.pow(right_digits);
                output + rhs
            }
        }
    }
}

impl Display for Op2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Op2::Base(op) => op.to_string(),
                Op2::Concat => "||".to_string(),
            }
        )
    }
}

impl Debug for Op2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

/// A simple math expression represented by sets of numbers and operators.
pub struct Expression<Op, Ops> {
    nums: VecDeque<usize>,
    operators: Ops,
    operator: PhantomData<Op>,
}

impl<Op, Ops> Expression<Op, Ops> {
    pub const fn new(nums: VecDeque<usize>, operators: Ops) -> Self {
        Self {
            nums,
            operators,
            operator: PhantomData,
        }
    }
}

impl<Op: Operator, Ops: AsRef<[Op]>> Expression<Op, Ops> {
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

impl<Op: Display, Ops: AsRef<[Op]>> Display for Expression<Op, Ops> {
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

pub trait Ordering {
    /// Generate all possible orderings for operators where `num` is the number of operators.
    fn orderings(num: usize) -> Vec<Vec<Self>>
    where
        Self: Sized;
}

impl<T> Ordering for T
where
    T: Variants + Copy,
{
    fn orderings(num: usize) -> Vec<Vec<Self>> {
        if num == 0 {
            return vec![vec![]];
        }

        let mut orderings = vec![];
        for el in Self::variants() {
            let sub_orderings = Self::orderings(num - 1);
            for mut sub in sub_orderings {
                sub.push(el);
                orderings.push(sub);
            }
        }
        orderings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case([3, 4], [Op::Add], 7)]
    #[case([1, 2, 3, 4], [Op::Mult, Op::Add, Op::Add], 9)]
    #[case([5, 1, 2, 4, 3], [Op::Add, Op::Mult, Op::Add, Op::Mult], 48)]
    #[case([6, 2, 3, 4], [Op::Add, Op::Mult, Op::Add], 28)]
    fn expression_evaluate(
        #[case] nums: impl Into<VecDeque<usize>>,
        #[case] ops: impl Into<Vec<Op>>,
        #[case] expected: usize,
    ) {
        let mut expression = Expression::new(nums.into(), ops.into());
        assert_eq!(expected, expression.evaluate().unwrap());
    }
}
