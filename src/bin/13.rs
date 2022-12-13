use std::cmp::Ordering;

use aoc2022::*;
use itertools::Itertools;
use serde::Deserialize;

const INPUT: &str = include_str!("../../input/13");

fn main() {
    let solution: usize = input()
        .iter()
        .tuples()
        .map(|(left, right)| left.cmp(right))
        .enumerate()
        .filter(|(_, v)| v == &Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();
    solved_level_1(solution);

    let divider2 = Item::List(vec![Item::List(vec![Item::Int(2)])]);
    let divider6 = Item::List(vec![Item::List(vec![Item::Int(6)])]);
    let mut packets = input();
    packets.extend([divider2.clone(), divider6.clone()]);
    packets.sort();
    let a = packets.iter().position(|p| p == &divider2).unwrap();
    let b = packets.iter().position(|p| p == &divider6).unwrap();
    solved_level_2((a + 1) * (b + 1));
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Item {
    Int(i64),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Int(left), Item::Int(right)) => left.cmp(right),
            (Item::List(left), Item::List(right)) => {
                for i in 0.. {
                    let (left, right) = match (left.get(i), right.get(i)) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(l), Some(r)) => (l, r),
                    };
                    match left.cmp(right) {
                        Ordering::Equal => { /* Continue checking */ }
                        o => return o,
                    }
                }
                Ordering::Equal
            }
            (left @ Item::Int(_), right @ Item::List(_)) => {
                Item::List(vec![left.clone()]).cmp(right)
            }
            (left @ Item::List(_), right @ Item::Int(_)) => {
                left.cmp(&Item::List(vec![right.clone()]))
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn input() -> Vec<Item> {
    INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Item>(l).unwrap())
        .collect()
}
