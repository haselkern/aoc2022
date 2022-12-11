use std::{collections::VecDeque, fmt, str::FromStr};

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/11");

fn main() {
    let mut troop = Troop::from_input();
    troop.play(20, 3);
    solved_level_1(troop.level());

    let mut troop = Troop::from_input();
    troop.play(10000, 1);
    solved_level_2(troop.level());
}

#[derive(Debug)]
struct Troop(Vec<Monkey>);

impl Troop {
    fn level(&self) -> usize {
        let mut inspections = self.0.iter().map(|m| m.inspected).collect_vec();
        inspections.sort();
        inspections.reverse();
        inspections[0] * inspections[1]
    }

    fn play(&mut self, rounds: usize, divider: u64) {
        for _ in 0..rounds {
            for monkey in 0..self.0.len() {
                self.monkey_play(monkey, divider);
            }
        }
    }

    fn monkey_play(&mut self, monkey: usize, divider: u64) {
        self.0[monkey].inspected += self.0[monkey].items.len();

        let modulus: u64 = self.0.iter().map(|m| m.test).product();

        while let Some(mut item) = self.0[monkey].items.pop_front() {
            let target = {
                let monkey = &self.0[monkey];
                match monkey.op {
                    Operation::Add(n) => item += n,
                    Operation::Mul(n) => item *= n,
                    Operation::Square => item *= item,
                };
                item /= divider;

                item %= modulus;

                if item % monkey.test == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                }
            };

            self.0[target].items.push_back(item);
        }
    }

    fn from_input() -> Self {
        let mut monkeys = Vec::new();
        // Placeholder while building.
        let mut current = Monkey {
            items: VecDeque::new(),
            op: Operation::Add(0u8.into()),
            test: 0,
            throw_true: 0,
            throw_false: 0,
            inspected: 0,
        };

        for (i, line) in INPUT.lines().enumerate() {
            match i % 7 {
                0 | 6 => { /* Monkey index or empty line can be ignored */ }
                1 => {
                    let (_, items) = line.split_once(": ").unwrap();
                    current.items = items
                        .split(", ")
                        .map(|item| item.parse().unwrap())
                        .collect();
                }
                2 => {
                    current.op = if line.contains("old * old") {
                        Operation::Square
                    } else if line.contains('*') {
                        Operation::Mul(line.parse_last())
                    } else {
                        Operation::Add(line.parse_last())
                    }
                }
                3 => current.test = line.parse_last(),
                4 => current.throw_true = line.parse_last(),
                5 => {
                    current.throw_false = line.parse_last();
                    // This is the final line
                    monkeys.push(current.clone());
                }
                i => unreachable!("{i} is impossible to get in mod 7"),
            }
        }

        Self(monkeys)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    test: u64,
    throw_true: usize,
    throw_false: usize,
    inspected: usize,
}

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

/// Helper trait to make code more readable.
/// Parses the last element from a whitespace separated str.
trait ParseLast<T> {
    fn parse_last(self) -> T;
}

impl<'a, T> ParseLast<T> for &'a str
where
    T: FromStr,
    T::Err: fmt::Debug,
{
    fn parse_last(self) -> T {
        self.split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap()
    }
}
