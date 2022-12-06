use std::collections::HashSet;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/06");

fn main() {
    solved_level_1(detect(INPUT, 4));
    solved_level_2(detect(INPUT, 14));
}

fn detect(signal: &str, size: usize) -> usize {
    let (i, _) = signal
        .chars()
        .collect_vec()
        .windows(size)
        .enumerate()
        .find(|(_, cs)| {
            let unique: HashSet<_> = cs.iter().collect();
            unique.len() >= size
        })
        .unwrap();
    i + size
}
