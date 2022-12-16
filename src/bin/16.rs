use std::collections::HashMap;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/16");

fn main() {
    println!("This does not work :(");
    let mut cave = Cave::new();
    solved_level_1(cave.level1());
}

type Str = &'static str;

#[derive(Debug)]
struct Valve {
    id: Str,
    rate: i64,
    tunnels: Vec<Str>,
}

struct Cave {
    valves: HashMap<Str, Valve>,
    distance_cache: HashMap<(Str, Str), i64>,
}

impl Cave {
    fn level1(&mut self) -> i64 {
        let mut remaining = self
            .valves
            .keys()
            .cloned()
            .filter(|&k| k != "AA")
            .collect_vec();

        self.level1_rec("AA", &mut remaining, 30)
    }

    fn level1_rec(&mut self, at: Str, remaining: &mut Vec<Str>, time: i64) -> i64 {
        let mut other_releases = Vec::new();

        for _ in 0..remaining.len() {
            let next = remaining.remove(0);

            let time = time - self.distance(at, next, 0) - 1;
            if time > 0 {
                other_releases.push(self.level1_rec(next, remaining, time));
            }

            remaining.push(next);
        }

        let release = self.valves[at].rate * time;
        release + other_releases.into_iter().max().unwrap_or(0)
    }

    fn distance(&mut self, from: Str, to: Str, stopper: usize) -> i64 {
        if from == to {
            return 0;
        }
        if let Some(d) = self.distance_cache.get(&(from, to)) {
            return *d;
        }
        if let Some(d) = self.distance_cache.get(&(to, from)) {
            return *d;
        }
        if self.valves[from].tunnels.contains(&to) {
            return 1;
        }
        if stopper > 30 {
            return 1000;
        }

        // Find recursively via neighbors.
        let mut distances = Vec::new();
        for neighbor in self.valves[from].tunnels.clone() {
            let d = self.distance(neighbor, to, stopper + 1);
            distances.push(d + 1);
        }

        let best = distances.into_iter().min().unwrap();
        self.distance_cache.insert((from, to), best);
        best
    }

    fn new() -> Self {
        let valves = INPUT
            .lines()
            .map(|line| {
                let parts = line.split(' ').collect_vec();
                let id = parts[1];
                let rate = parts[4].split_once('=').unwrap().1;
                let rate = rate.strip_suffix(';').unwrap_or(rate).parse().unwrap();

                let tunnels = parts
                    .iter()
                    .skip(9)
                    .map(|part| part.strip_suffix(',').unwrap_or(part))
                    .collect();

                Valve { id, rate, tunnels }
            })
            .map(|v| (v.id, v))
            .collect();
        Self {
            valves,
            distance_cache: HashMap::new(),
        }
    }
}
