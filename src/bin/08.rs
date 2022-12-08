use std::ops::Add;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/08");

fn main() {
    let forest = Forest::from_input();
    solved_level_1(forest.level1());
    solved_level_2(forest.level2());
}

#[derive(Debug)]
struct Forest(Vec<Vec<i64>>);

impl Forest {
    fn from_input() -> Self {
        let trees = INPUT
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i64)
                    .collect_vec()
            })
            .collect_vec();
        Self(trees)
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let x = 0..self.0[0].len();
        let y = 0..self.0.len();
        (x).cartesian_product(y)
    }

    fn level1(&self) -> usize {
        self.iter().filter(|&(x, y)| self.is_visible(x, y)).count()
    }

    fn level2(&self) -> i64 {
        self.iter().map(|(x, y)| self.score(x, y)).max().unwrap()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.0[y][x];

        let mut right_visible = true;
        for xx in x.add(1)..self.0[0].len() {
            if self.0[y][xx] >= height {
                right_visible = false;
            }
        }

        let mut left_visible = true;
        for xx in 0..x {
            if self.0[y][xx] >= height {
                left_visible = false;
            }
        }

        let mut down_visible = true;
        for yy in y.add(1)..self.0.len() {
            if self.0[yy][x] >= height {
                down_visible = false;
            }
        }

        let mut up_visible = true;
        for yy in 0..y {
            if self.0[yy][x] >= height {
                up_visible = false;
            }
        }

        right_visible || left_visible || down_visible || up_visible
    }

    fn score(&self, x: usize, y: usize) -> i64 {
        let height = self.0[y][x];

        let mut right = 0;
        for xx in x.add(1)..self.0[0].len() {
            right += 1;
            if self.0[y][xx] >= height {
                break;
            }
        }

        let mut left = 0;
        for xx in (0..x).rev() {
            left += 1;
            if self.0[y][xx] >= height {
                break;
            }
        }

        let mut down = 0;
        for yy in y.add(1)..self.0.len() {
            down += 1;
            if self.0[yy][x] >= height {
                break;
            }
        }

        let mut up = 0;
        for yy in (0..y).rev() {
            up += 1;
            if self.0[yy][x] >= height {
                break;
            }
        }

        right * left * down * up
    }
}
