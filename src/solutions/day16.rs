use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::solver::Solver;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Valve {
    name: String,
    flow_rate: usize,
    connections: Vec<String>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"([A-Z]{2})").unwrap();
}

pub struct Problem;

impl Solver for Problem {
    type Input = HashMap<String, Valve>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .map(|l| {
                    let name = l[6..8].to_string();
                    let flow_rate = l[23..].split_once(";").unwrap().0.trim().parse().unwrap();
                    let connections = RE
                        .captures_iter(l)
                        .skip(1)
                        .map(|x| x.get(0).unwrap().as_str().to_string())
                        .collect();

                    (
                        name.clone(),
                        Valve {
                            name,
                            flow_rate,
                            connections,
                        },
                    )
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let distances = compute_distances(input);
        Some(
            possible_pressures_released(
                input.get("AA").unwrap(),
                &distances,
                &input,
                HashSet::new(),
                0,
                1,
                0,
                30,
            )
            .into_iter()
            .max_by_key(|x| x.0)
            .unwrap_or((0, None))
            .0,
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let distances = compute_distances(input);
        let pressures_at_26 = possible_pressures_released(
            input.get("AA").unwrap(),
            &distances,
            &input,
            HashSet::new(),
            0,
            1,
            0,
            26,
        );

        let mut max = 0;
        for (one_pressure, one_visited) in pressures_at_26.iter() {
            for (other_pressure, other_visited) in pressures_at_26.iter() {
                if one_visited
                    .as_ref()
                    .unwrap()
                    .intersection(&other_visited.as_ref().unwrap())
                    .count()
                    == 0
                {
                    max = max.max(one_pressure + other_pressure);
                }
            }
        }

        Some(max)
    }
}

fn distance(
    valves: &HashMap<String, Valve>,
    from: &String,
    to: &String,
    visited: HashSet<&String>,
) -> usize {
    let from_valve = valves.get(from).unwrap();
    if from_valve.connections.contains(to) {
        return 1;
    }

    from_valve
        .connections
        .iter()
        .filter(|&conn| !visited.contains(conn))
        .map(|conn| {
            let mut visited = visited.clone();
            visited.insert(conn);
            distance(valves, conn, to, visited) + 1
        })
        .min()
        .unwrap_or(150)
}

fn compute_distances(valves: &HashMap<String, Valve>) -> HashMap<(&String, &String), usize> {
    valves
        .values()
        .flat_map(|from| {
            valves
                .values()
                .filter(move |to| *to != from)
                .filter(|to| to.flow_rate > 0)
                .map(|to| {
                    (
                        (&from.name, &to.name),
                        distance(&valves, &from.name, &to.name, HashSet::new()),
                    )
                })
        })
        .collect()
}

fn possible_pressures_released<'a>(
    current_valve: &Valve,
    distances: &HashMap<(&String, &String), usize>,
    valves: &'a HashMap<String, Valve>,
    open: HashSet<&'a String>,
    flow_rate: usize,
    steps: usize,
    released: usize,
    max_steps: usize,
) -> Vec<(usize, Option<HashSet<&'a String>>)> {
    if steps == max_steps {
        return vec![(released, Some(open))];
    }

    let unopened: HashSet<&String> = distances
        .keys()
        .filter_map(|(_, destination)| (!open.contains(destination)).then_some(*destination))
        .collect();

    if unopened.is_empty() {
        return vec![(released + (max_steps - steps) * flow_rate, Some(open))];
    }

    unopened
        .iter()
        .filter_map(
            |destination| match distances.get(&(&current_valve.name, destination)) {
                None => None,
                Some(distance) if steps + *distance >= max_steps => None,
                Some(distance) => {
                    let target_valve = valves.get(*destination).unwrap();
                    let mut open_with_target = open.clone();
                    open_with_target.insert(&target_valve.name);

                    Some(possible_pressures_released(
                        target_valve,
                        distances,
                        valves,
                        open_with_target,
                        flow_rate + target_valve.flow_rate,
                        steps + *distance + 1,
                        released + (flow_rate * (distance)) + flow_rate + target_valve.flow_rate,
                        max_steps,
                    ))
                }
            },
        )
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(1651));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(1707));
    }
}
