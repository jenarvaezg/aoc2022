use std::cmp::Reverse;

use crate::solver::Solver;

#[derive(Debug, Clone)]
pub enum Operation {
    Add(usize),
    Mult(usize),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monke {
    items: Vec<usize>,
    operation: Operation,
    test_divisible: usize,
    next_target: (usize, usize),
}

pub struct Problem;

fn last_number_in_line(line: &str) -> usize {
    line.split_ascii_whitespace()
        .last()
        .and_then(|x| Some(x.parse().unwrap()))
        .unwrap()
}

impl Solver for Problem {
    type Input = Vec<Monke>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .split("\n\n")
                .map(|raw_monke| {
                    let lines: Vec<&str> = raw_monke.split("\n").map(|l| l.trim()).collect();
                    let items = lines[1]
                        .split(": ")
                        .last()
                        .unwrap()
                        .split(", ")
                        .map(|n| n.parse().unwrap())
                        .collect();

                    let operation = lines[2]
                        .split("= ")
                        .last()
                        .map(|raw_op| {
                            if raw_op == "old * old" {
                                Operation::Square
                            } else if let Some(x) = raw_op.split_once("+ ") {
                                Operation::Add(x.1.parse().unwrap())
                            } else if let Some(x) = raw_op.split_once("* ") {
                                Operation::Mult(x.1.parse().unwrap())
                            } else {
                                unreachable!()
                            }
                        })
                        .unwrap();

                    let test_divisible = last_number_in_line(lines[3]);
                    let next_target =
                        (last_number_in_line(lines[4]), last_number_in_line(lines[5]));

                    Monke {
                        items,
                        operation,
                        test_divisible,
                        next_target,
                    }
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut monkes = input.to_vec();
        let mut inspections = vec![0; monkes.len()];

        for _ in 0..20 {
            monke_round(&mut monkes, &mut inspections, |x| x / 3);
        }
        inspections.sort_unstable_by_key(|x| Reverse(*x));

        Some(inspections.into_iter().take(2).product())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut monkes = input.to_vec();
        let mut inspections = vec![0; monkes.len()];
        let modulo: usize = monkes.iter().map(|m| m.test_divisible).product();

        for _ in 0..10000 {
            monke_round(&mut monkes, &mut inspections, |x| x % modulo);
        }
        inspections.sort_unstable_by_key(|x| Reverse(*x));

        Some(inspections.into_iter().take(2).product())
    }
}

fn monke_round(
    monkes: &mut Vec<Monke>,
    inspections: &mut Vec<usize>,
    worry_fn: impl Fn(usize) -> usize,
) {
    for monke_i in 0..monkes.len() {
        let monke = monkes[monke_i].clone();
        for item in monke.items {
            inspections[monke_i] += 1;

            let new_item = worry_fn(match monke.operation {
                Operation::Add(x) => item + x,
                Operation::Mult(x) => item * x,
                Operation::Square => item * item,
            });

            // to other monke
            if new_item % monke.test_divisible == 0 {
                monkes[monke.next_target.0].items.push(new_item);
            } else {
                monkes[monke.next_target.1].items.push(new_item);
            }
        }
        monkes[monke_i].items.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monke 2
    If false: throw to monke 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monke 2
    If false: throw to monke 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monke 1
    If false: throw to monke 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monke 0
    If false: throw to monke 1"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(2713310158));
    }
}
