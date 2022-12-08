use crate::solver::Solver;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    cells: Vec<char>,
    pub w: usize,
    pub h: usize,
}

impl Grid {
    fn get(&self, (x, y): (usize, usize)) -> Option<&char> {
        if x < self.w && y < self.h {
            self.cells.get(x + y * self.w)
        } else {
            None
        }
    }

    fn visible(&self) -> usize {
        let mut sum = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                // check is border
                if x == 0 || y == 0 || y == self.h - 1 || x == self.w - 1 {
                    sum += 1;
                    continue;
                }
                let val = self.get((x, y)).unwrap();
                // check from top
                if (0..y).all(|i| self.get((x, i)).unwrap() < val) {
                    sum += 1;
                    continue;
                }
                // check from bottom
                if (y + 1..self.h)
                    .rev()
                    .all(|i| self.get((x, i)).unwrap() < val)
                {
                    sum += 1;
                    continue;
                }
                // check from left
                if (0..x).all(|i| self.get((i, y)).unwrap() < val) {
                    sum += 1;
                    continue;
                }
                // check from right
                if (x + 1..self.w)
                    .rev()
                    .all(|i| self.get((i, y)).unwrap() < val)
                {
                    sum += 1;
                    continue;
                }
            }
        }
        sum
    }

    fn highest_scenic_score(&self) -> usize {
        (0..self.h)
            .flat_map(|e| std::iter::repeat(e).zip(0..self.w))
            .map(|(x, y)| {
                let mut score = 1;
                let val = self.get((x, y)).unwrap();
                // check left

                if x > 0 {
                    let mut los = (0..x)
                        .rev()
                        .take_while(|&i| self.get((i, y)).unwrap() < val)
                        .count();
                    if los < x {
                        los += 1
                    }
                    score *= los;
                }

                if x < self.w - 1 {
                    // check right
                    let mut los = (x + 1..self.w)
                        .take_while(|&i| self.get((i, y)).unwrap() < val)
                        .count();

                    if los < self.w - x - 1 {
                        los += 1
                    }
                    score *= los;
                }

                if y > 0 {
                    // check top
                    let mut los = (0..y)
                        .rev()
                        .take_while(|&i| self.get((x, i)).unwrap() < val)
                        .count();

                    if los < y {
                        los += 1
                    }

                    score *= los;
                }

                if y < self.h - 1 {
                    // check bottom
                    let mut los = (y + 1..self.h)
                        .take_while(|&i| self.get((x, i)).unwrap() < val)
                        .count();

                    if los < self.h - y - 1 {
                        los += 1
                    }
                    score *= los;
                }
                score
            })
            .max()
            .unwrap()
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Grid;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        let lines: Vec<&str> = raw_input.lines().collect();
        let h = lines.len();
        let w = lines[0].len();
        let cells: Vec<char> = lines.iter().flat_map(|s| s.chars()).collect();

        Some(Grid { cells, h, w })
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.visible())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(input.highest_scenic_score())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(8));
    }
}
