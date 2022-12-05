use std::collections::VecDeque;

use aoc2022::*;
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../../input/05");

fn main() {
    let mut game = input();
    game.apply_actions9000();
    solved_level_1(game.top_letters());

    let mut game = input();
    game.apply_actions9001();
    solved_level_2(game.top_letters());
}

struct Game {
    stacks: Vec<VecDeque<char>>,
    actions: Vec<Action>,
}

impl Game {
    fn top_letters(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.iter().last().unwrap())
            .collect()
    }

    fn apply_actions9000(&mut self) {
        for a in &self.actions {
            for _ in 0..a.count {
                let item = self.stacks[a.from].pop_back().unwrap();
                self.stacks[a.to].push_back(item);
            }
        }
    }

    fn apply_actions9001(&mut self) {
        for a in &self.actions {
            let drain_start = self.stacks[a.from].len() - a.count;
            let items = self.stacks[a.from].drain(drain_start..).collect_vec();
            self.stacks[a.to].extend(items.iter());
        }
    }
}

struct Action {
    from: usize,
    to: usize,
    count: usize,
}

fn input() -> Game {
    let mut stacks = Vec::new();
    let mut actions = Vec::new();
    let action_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for line in INPUT.lines() {
        if line.contains('[') {
            for (i, mut letter) in line.chars().chunks(4).into_iter().enumerate() {
                let letter = letter.nth(1).unwrap();

                if i >= stacks.len() {
                    stacks.push(VecDeque::new());
                }

                if letter.is_whitespace() {
                    continue;
                }

                stacks[i].push_front(letter);
            }
        }

        if let Some(cap) = action_pattern.captures(line) {
            let count = cap.get(1).unwrap().as_str().parse().unwrap();
            let from: usize = cap.get(2).unwrap().as_str().parse().unwrap();
            let to: usize = cap.get(3).unwrap().as_str().parse().unwrap();

            actions.push(Action {
                count,
                to: to - 1,
                from: from - 1,
            })
        }
    }

    Game { stacks, actions }
}
