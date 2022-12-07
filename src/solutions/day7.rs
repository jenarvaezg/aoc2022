use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::solver::Solver;

type FS = HashMap<PathBuf, HashSet<(usize, String)>>;

pub struct Problem;

impl Solver for Problem {
    type Input = FS;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        let mut fs = HashMap::new();
        let mut pwd = PathBuf::new();
        for l in raw_input.split('$').skip(1) {
            match l.trim().lines().next().unwrap() {
                "ls" => {
                    let entries = l.lines().skip(1).map(|output| {
                        let (size, f) = output.split_once(' ').unwrap();
                        (size.parse().unwrap_or(0), f.to_string())
                    });
                    fs.entry(pwd.clone())
                        .or_insert(HashSet::new())
                        .extend(entries);
                }
                "cd .." => {
                    pwd.pop();
                }
                cd_dir => {
                    pwd.push(cd_dir.split_once(' ').unwrap().1);
                }
            }
        }

        Some(fs)
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .keys()
                .map(|k| dir_size(input, k))
                .filter(|&size| size < 100000)
                .sum(),
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let total_size = dir_size(input, &PathBuf::from("/"));

        input
            .keys()
            .map(|k| dir_size(input, k))
            .filter(|&size| 40000000 + size >= total_size)
            .min()
    }
}

fn dir_size(fs: &FS, dir: &PathBuf) -> usize {
    let files = fs.get(dir).unwrap();
    files
        .iter()
        .map(|(size, name)| match size {
            0 => dir_size(fs, &dir.join(name)),
            x => *x,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(95437));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(24933642));
    }
}
