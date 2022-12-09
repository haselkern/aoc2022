use std::{
    collections::HashSet,
    fmt::Display,
    iter,
    ops::{Add, Sub},
    str::FromStr,
};

use aoc2022::*;

const INPUT: &str = include_str!("../../input/09");

fn main() {
    let mut rope = Rope::new(2);
    input().for_each(|a| rope.apply(a));
    solved_level_1(rope.visited.len());

    let mut rope = Rope::new(10);
    input().for_each(|a| rope.apply(a));
    solved_level_2(rope.visited.len());
}

#[derive(Debug)]
struct Rope {
    nodes: Vec<Position>,
    visited: HashSet<Position>,
}

impl Rope {
    fn new(nodes: usize) -> Self {
        Self {
            nodes: iter::repeat(Position::default()).take(nodes).collect(),
            visited: HashSet::from([Position::default()]),
        }
    }

    fn apply(&mut self, action: Action) {
        self.nodes[0] = self.nodes[0] + action.direction();

        for i in 1..self.nodes.len() {
            let (head, tail) = self.nodes.split_at_mut(i);
            let head = head.last_mut().unwrap();
            let tail = tail.first_mut().unwrap();
            let diff = *head - *tail;

            let pull = if (diff.0 == 0) ^ (diff.1 == 0) {
                // Horizontal/vertical
                let pull_x = if diff.0.abs() > 1 { diff.0.signum() } else { 0 };
                let pull_y = if diff.1.abs() > 1 { diff.1.signum() } else { 0 };
                Position(pull_x, pull_y)
            } else if diff.0.abs() > 1 || diff.1.abs() > 1 {
                // Diagonal
                Position(diff.0.signum(), diff.1.signum())
            } else {
                Position::default()
            };

            *tail = *tail + pull;
        }

        self.visited.insert(*self.nodes.last().unwrap());
    }
}

impl Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Find bounds
        let (min, max) = self.nodes.iter().chain(self.visited.iter()).fold(
            (Position::default(), Position::default()),
            |(min, max), p| {
                (
                    Position(min.0.min(p.0), min.1.min(p.1)),
                    Position(max.0.max(p.0), max.1.max(p.1)),
                )
            },
        );

        // Print field
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                let pos = Position(x, y);

                let mut c = '.';
                if self.visited.contains(&pos) {
                    c = '#';
                }
                if pos.is_zero() {
                    c = 's';
                }
                if let Some(node_idx) = self.nodes.iter().position(|&p| p == pos) {
                    c = "H123456789".chars().nth(node_idx).unwrap();
                }

                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Position(i64, i64);

impl Position {
    fn is_zero(self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

impl Action {
    fn direction(self) -> Position {
        match self {
            Action::Up => (0, -1).into(),
            Action::Down => (0, 1).into(),
            Action::Left => (-1, 0).into(),
            Action::Right => (1, 0).into(),
        }
    }
}

fn input() -> impl Iterator<Item = Action> {
    INPUT.lines().flat_map(|line| {
        let (action, count) = line.split_once(' ').unwrap();
        let action: Action = action.parse().unwrap();
        let count: usize = count.parse().unwrap();
        iter::repeat(action).take(count)
    })
}
