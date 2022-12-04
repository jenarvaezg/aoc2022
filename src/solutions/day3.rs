use itertools::Itertools;

use crate::solver::Solver;
use std::collections::HashSet;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<u128>>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|x| {
                            if x.is_uppercase() {
                                x as u128 - 38
                            } else {
                                x as u128 - 96
                            }
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
                .map(|rucksack| {
                    let mut chunks = rucksack.chunks(rucksack.len() / 2);
                    let first_half_chars: HashSet<&u128> =
                        chunks.next().unwrap().into_iter().collect();

                    chunks
                        .next()
                        .unwrap()
                        .into_iter()
                        .find(|x| first_half_chars.contains(x))
                        .unwrap()
                })
                .sum(),
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .chunks(3)
                .into_iter()
                .map(|chunk| {
                    let reduction = chunk
                        .map(|x| x.into_iter().collect::<HashSet<_>>())
                        .reduce(|accum, item| {
                            accum.intersection(&item).copied().collect::<HashSet<_>>()
                        })
                        .unwrap();

                    *reduction.iter().next().unwrap()
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
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_second() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(70));
    }
}
