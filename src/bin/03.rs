use core::panic;
use std::collections::HashSet;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/03");

fn main() {
    let solution: i64 = INPUT
        .lines()
        .map(split_rucksack)
        .map(find_common)
        .map(priority)
        .sum();
    solved_level_1(solution);

    let solution: i64 = INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(find_common)
        .map(priority)
        .sum();
    solved_level_2(solution);
}

fn find_common(parts: impl IntoIterator<Item = &'static str>) -> char {
    parts
        .into_iter()
        .map(|p| p.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn split_rucksack(r: &str) -> [&str; 2] {
    let (a, b) = r.split_at(r.len() / 2);
    [a, b]
}

fn priority(item: char) -> i64 {
    match item {
        'a'..='z' => item as i64 - 'a' as i64 + 1,
        'A'..='Z' => item as i64 - 'A' as i64 + 27,
        _ => panic!("unknown item: {item}"),
    }
}
