use crate::solver::Solver;
use scan_fmt::scan_fmt;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}
pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Vec<char>>, Vec<Instruction>);
    type Output = String;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        let (raw_boxes, raw_instructions) = raw_input.split_once("\n\n")?;
        let mut boxes_lines: Vec<&str> = raw_boxes.lines().collect();

        let count: usize = boxes_lines
            .pop()?
            .split("   ")
            .last()?
            .strip_suffix(" ")?
            .parse()
            .unwrap();

        let mut boxes = vec![Vec::<char>::new(); count];
        for line in boxes_lines {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(pos, chars)| {
                    if chars[1].is_alphabetic() {
                        boxes[pos].push(chars[1])
                    }
                });
        }

        let instructions: Vec<Instruction> = raw_instructions
            .lines()
            .map(|l| {
                let (count, from, to) =
                    scan_fmt!(l, "move {} from {} to {}", usize, usize, usize).unwrap();
                Instruction { count, from, to }
            })
            .collect();

        Some((boxes, instructions))
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut boxes = input.0.clone();
        for instruction in input.1.iter() {
            let mut from = boxes[instruction.from - 1].clone();
            let to = boxes[instruction.to - 1].clone();
            let mut x: Vec<char> = from.drain(..instruction.count).collect();
            x.reverse();
            x.extend(to);

            boxes[instruction.from - 1] = from;
            boxes[instruction.to - 1] = x;
        }

        Some(boxes.into_iter().map(|b| b[0]).collect())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut boxes = input.0.clone();
        for instruction in input.1.iter() {
            let mut from = boxes[instruction.from - 1].clone();
            let to = boxes[instruction.to - 1].clone();
            let mut x: Vec<char> = from.drain(..instruction.count).collect();
            x.extend(to);

            boxes[instruction.from - 1] = from;
            boxes[instruction.to - 1] = x;
        }

        Some(boxes.into_iter().map(|b| b[0]).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(String::from("CMZ")));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(String::from("MCD")));
    }
}
