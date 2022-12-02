use itertools::Itertools;

use crate::solver::Solver;

pub struct Problem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Hand {
    ROCK = 1,
    PAPER = 2,
    SCISSOR = 3,
}

#[derive(Copy, Clone, Debug)]
pub enum Winner {
    LEFT = 0,
    DRAW = 3,
    RIGHT = 6,
}

impl Winner {
    pub fn value(&self) -> u128 {
        *self as u128
    }
}

impl Hand {
    fn value(&self) -> u128 {
        *self as u128
    }

    fn beats(&self) -> Self {
        match *self {
            Hand::ROCK => Hand::SCISSOR,
            Hand::PAPER => Hand::ROCK,
            Hand::SCISSOR => Hand::PAPER,
        }
    }

    fn get_winner(left: Self, right: Self) -> Winner {
        if left == right {
            Winner::DRAW
        } else if left.beats() == right {
            Winner::LEFT
        } else {
            Winner::RIGHT
        }
    }
}

impl Into<Winner> for Hand {
    fn into(self) -> Winner {
        match self {
            Hand::ROCK => Winner::LEFT,
            Hand::PAPER => Winner::DRAW,
            Hand::SCISSOR => Winner::RIGHT,
        }
    }
}

impl Solver for Problem {
    type Input = Vec<(Hand, Hand)>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .flat_map(str::split_ascii_whitespace)
                .map(|c| match c {
                    "A" | "X" => Hand::ROCK,
                    "B" | "Y" => Hand::PAPER,
                    "C" | "Z" => Hand::SCISSOR,
                    c => unreachable!("Unexpected value {}", c),
                })
                .tuples()
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .map(|hands| {
                    let winner = Hand::get_winner(hands.0, hands.1);
                    winner.value() + hands.1.value()
                })
                .sum(),
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .map(|hands| match hands.1.into() {
                    Winner::LEFT => hands.0.beats().value() + Winner::LEFT.value(),
                    Winner::DRAW => hands.0.value() + Winner::DRAW.value(),
                    Winner::RIGHT => hands.0.beats().beats().value() + Winner::RIGHT.value(),
                })
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = r#"A Y
B X
C Z"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_second() {
        let input = r#"A Y
B X
C Z"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(12));
    }
}
