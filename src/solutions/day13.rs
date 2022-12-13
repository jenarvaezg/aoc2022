use core::str;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;

use crate::solver::Solver;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> IResult<&str, Packet> {
        alt((
            map(i64, |v| Packet::Int(v)),
            map(
                delimited(tag("["), separated_list0(tag(","), Packet::parse), tag("]")),
                |v| Packet::List(v),
            ),
        ))(line)
    }

    pub fn new(line: &str) -> Self {
        Self::parse(line).unwrap().1
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(s), Packet::Int(o)) => s.partial_cmp(o),
            (Packet::List(_), Packet::Int(_)) => {
                self.partial_cmp(&Packet::List(vec![other.clone()]))
            }
            (Packet::Int(_), Packet::List(_)) => {
                Packet::List(vec![self.clone()]).partial_cmp(other)
            }
            (Packet::List(s), Packet::List(o)) => {
                for (v1, v2) in s.iter().zip(o) {
                    if let Some(result) = v1.partial_cmp(v2) {
                        if result != Ordering::Equal {
                            return Some(result);
                        }
                    }
                }

                s.len().partial_cmp(&o.len())
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(Packet, Packet)>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .split("\n\n")
                .map(|pairs| {
                    let mut x = pairs.split("\n").map(Packet::new);

                    (x.next().unwrap(), x.next().unwrap())
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .iter()
                .enumerate()
                .filter(|(_, (first, second))| first <= second)
                .map(|(i, _)| i + 1)
                .sum(),
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut packets: Vec<_> = input
            .iter()
            .map(|x| vec![x.0.clone(), x.1.clone()])
            .flatten()
            .collect();

        let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
        packets.push(divider_1.clone());
        packets.push(divider_2.clone());

        packets.sort_unstable();

        let pos1 = packets.binary_search(&divider_1).unwrap() + 1;
        let pos2 = packets.binary_search(&divider_2).unwrap() + 1;

        Some(pos1 * pos2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(140));
    }
}
