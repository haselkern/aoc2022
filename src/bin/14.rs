use std::collections::HashSet;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/14");

fn main() {
    solved_level_1(Cave::from_input(false).simulate());
    solved_level_2(Cave::from_input(true).simulate());
}

struct Cave {
    floor: Option<i64>,
    cells: HashSet<(i64, i64)>,
}

impl Cave {
    fn from_input(floor: bool) -> Self {
        let mut cells = HashSet::new();
        for line in INPUT.lines() {
            let pairs = line
                .split(" -> ")
                .flat_map(|pair| pair.split(','))
                .map(|n| n.parse::<i64>().unwrap())
                .tuples()
                .tuple_windows();
            for ((x1, y1), (x2, y2)) in pairs {
                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        cells.insert((x, y));
                    }
                }
            }
        }

        let floor = floor.then(|| cells.iter().map(|(_, y)| *y).max().unwrap() + 2);

        Self { floor, cells }
    }

    fn simulate(&mut self) -> usize {
        let max_y = self.cells.iter().copied().map(|(_, y)| y).max().unwrap();

        for count in 0.. {
            let mut x = 500;
            let mut y = 0;
            loop {
                // End of level 1: Sand falls into the void.
                if y > max_y + 10 {
                    return count;
                }

                // Check for free places
                let left = self.is_free(x - 1, y + 1);
                let down = self.is_free(x, y + 1);
                let right = self.is_free(x + 1, y + 1);

                match (left, down, right) {
                    (false, false, false) => {
                        self.cells.insert((x, y));

                        // End of level 2: The source has been filled.
                        if (x, y) == (500, 0) {
                            return count + 1;
                        }

                        break;
                    }
                    (_, true, _) => y += 1,
                    (true, false, _) => {
                        y += 1;
                        x -= 1;
                    }
                    (false, false, true) => {
                        y += 1;
                        x += 1;
                    }
                }
            }
        }

        unreachable!();
    }

    fn is_free(&self, x: i64, y: i64) -> bool {
        if let Some(floor) = self.floor {
            if y >= floor {
                return false;
            }
        }

        self.cells.get(&(x, y)).is_none()
    }
}
