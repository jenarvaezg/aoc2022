use crate::solver::Solver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output = isize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .map(|l| match l {
                    "noop" => Instruction::Noop,
                    l => Instruction::Addx(l.split_once(" ").unwrap().1.parse().unwrap()),
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut x = 1;
        let mut strengths = Vec::<isize>::new();
        let mut cycle = 1;

        for instruction in input.into_iter() {
            cycle += 1;

            if (cycle + 20) % 40 == 0 {
                strengths.push(cycle * x);
            }

            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(v) => {
                    cycle += 1;
                    x += v;
                    if (cycle + 20) % 40 == 0 {
                        strengths.push(cycle * x);
                    }
                }
            }
        }

        Some(strengths.into_iter().sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut x = 1;
        let mut cycle = 1_isize;
        let mut buf = vec![' '; 40 * 6];
        draw(&mut buf, cycle, x);

        for instruction in input.into_iter() {
            draw(&mut buf, cycle, x);
            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(v) => {
                    cycle += 1;
                    draw(&mut buf, cycle, x);
                    x += v;
                }
            }
            cycle += 1;
        }

        // Could be done with a string writer or something?
        buf.iter().enumerate().for_each(|(i, v)| {
            if (i + 1) % 40 == 0 {
                println!("{}", v)
            } else {
                print!("{}", v)
            }
        });

        Some(1)
    }
}

fn draw(buf: &mut Vec<char>, cycle: isize, x: isize) -> () {
    let col = (cycle - 1) % 40;
    if x.abs_diff(col) < 2 {
        buf[cycle as usize - 1] = '█';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, None);
    }
}
