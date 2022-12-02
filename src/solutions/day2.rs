use crate::solver::Solver;

pub struct Problem;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Play {
    ROCK,
    PAPER,
    SCISSOR,
}

#[derive(Copy, Clone, Debug)]
pub enum Winner {
    LEFT,
    RIGHT,
    DRAW,
}

impl Play {
    pub fn index(&self) -> u128 {
        *self as u128
    }

    fn beats(&self) -> Self {
        match *self {
            Play::ROCK => Play::SCISSOR,
            Play::PAPER => Play::ROCK,
            Play::SCISSOR => Play::PAPER,
        }
    }

    pub fn get_winner(left: Self, right: Self) -> Winner {
        if left == right {
            Winner::DRAW
        } else if left == Play::ROCK {
            if right == Play::PAPER {
                Winner::RIGHT
            } else {
                Winner::LEFT
            }
        } else if left == Play::PAPER {
            if right == Play::SCISSOR {
                Winner::RIGHT
            } else {
                Winner::LEFT
            }
        } else {
            if right == Play::ROCK {
                Winner::RIGHT
            } else {
                Winner::LEFT
            }
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Vec<Play>>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .map(|l| {
                    l.split(" ")
                        .map(|c| match c {
                            "A" => Play::ROCK,
                            "X" => Play::ROCK,
                            "B" => Play::PAPER,
                            "Y" => Play::PAPER,
                            "C" => Play::SCISSOR,
                            "Z" => Play::SCISSOR,
                            c => unreachable!("dafuq {}", c),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .map(|round| {
                    let winner = Play::get_winner(round[0], round[1]);
                    let result = match winner {
                        Winner::DRAW => 3,
                        Winner::LEFT => 0,
                        Winner::RIGHT => 6,
                    };
                    result + round[1].index() + 1
                })
                .sum(),
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .map(|round| {
                    match round[1] {
                        // LOSE
                        Play::ROCK => round[0].beats().index() + 1,
                        // DRAW
                        Play::PAPER => round[0].index() + 4,
                        // Win
                        Play::SCISSOR => round[0].beats().beats().index() + 7,
                    }
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
