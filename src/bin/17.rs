use std::{
    collections::{hash_map::DefaultHasher, VecDeque},
    hash::{Hash, Hasher},
    ops::Neg,
};

use aoc2022::*;

const INPUT: &str = include_str!("../../input/17");

fn main() {
    let (solution, max_drop) = simulate(2022, None);
    solved_level_1(solution);
    let (solution, _) = simulate(1000000000000, max_drop);
    solved_level_2(solution);
}

/// Get (pile height, maximum drop height) after simulating a number of rocks.
/// This detects cycles in the forming pile and skips simulating large chunks of repetition.
/// Truncate is the maximum height of the tower that should be kept.
fn simulate(rocks: usize, truncate: impl Into<Option<usize>>) -> (i64, usize) {
    let mut jet = input().cycle();
    let mut pile = Pile::new(truncate.into().unwrap_or(usize::MAX));

    let mut rock_stream = Rock::stream();

    // There are five types of rocks and the input has a certain length.
    // This is the period the input repeats.
    // We always simulate a whole repeat_period while checking for cycles, since only after
    // that time has the pile a chance to repeat.
    let repeat_period = input().count() * 5;

    // Keep track of (iteration, hash of pile, height of pile) to detect cycles.
    let mut states = Vec::new();

    let mut max_drop = 0;

    let mut i = 0;
    while i < rocks {
        i += 1;

        let rock = rock_stream.next().unwrap();
        let dropped = pile.drop(rock, &mut jet);
        max_drop = max_drop.max(dropped);

        // This if-block was added as an optimization for level 2.
        // The code works without it, but is impossibly slow in level 2.
        if i % repeat_period == 0 {
            let mut hasher = DefaultHasher::new();
            pile.rows.hash(&mut hasher);
            let hash = hasher.finish();
            let height = pile.height();

            // Have we already seen this pile?
            if let Some(&(prev_i, _, prev_height)) =
                states.iter().find(|(_, prev_hash, _)| *prev_hash == hash)
            {
                // We have already seen this pile -> cycle found. We can now skip a large portion of the simulation.
                let jump_height = height - prev_height;
                let jump_i = i - prev_i;
                let remaining = rocks - i;
                let jumps = remaining / jump_i;
                if jumps == 0 {
                    // Nothing to skip, simulation is almost done.
                    continue;
                }
                println!("Found cycle: Iteration {i} looks like iteration {prev_i}.");
                println!("Fast fowarding {} iterations.", jumps * jump_i);
                i += jumps * jump_i;
                pile.additional_height += jumps as i64 * jump_height;
                println!("{} iterations left.", rocks - i);
            }

            states.push((i, hash, height));
        }
    }

    (pile.height(), max_drop)
}

/// The pile of already placed rocks.
struct Pile {
    rows: VecDeque<[bool; 7]>,
    additional_height: i64,
    truncate: usize,
}

impl Pile {
    fn new(truncate: usize) -> Self {
        Self {
            rows: VecDeque::new(),
            additional_height: 0,
            truncate,
        }
    }

    /// Drop a new rock into the pile. The rock is assumed to be untampered.
    /// Returns the number of spaces the rock dropped down.
    fn drop(&mut self, mut rock: Rock, jet: &mut impl Iterator<Item = Shift>) -> usize {
        // Rocks spawn 2 from the left wall and 3 above the pile.
        rock.shift(Shift::Right);
        rock.shift(Shift::Right);
        rock.add_y(self.height() + 3);

        let mut dropped = 0;

        loop {
            let shift = jet.next().unwrap();
            rock.shift(shift);
            if !self.fits(&rock) {
                // Oopsy, revert that.
                rock.shift(-shift);
            }

            rock.add_y(-1);
            dropped += 1;
            if !self.fits(&rock) {
                // The rock could no longer drop down. Revert and place it there.
                rock.add_y(1);
                dropped -= 1;
                self.place(rock);
                return dropped;
            }
        }
    }
    fn height(&self) -> i64 {
        self.rows.len() as i64 + self.additional_height
    }
    fn place(&mut self, mut rock: Rock) {
        // Bring world-coordinates to truncated coordinates.
        rock.add_y(-self.additional_height);

        // Increase pile height to fit rock
        let max_y = rock.0.iter().map(|&(_x, y)| y as usize).max().unwrap();
        while self.rows.len() <= max_y {
            self.rows.push_back(Default::default());
        }

        for pos in rock.0 {
            self.rows[pos.1 as usize][pos.0 as usize] = true;
        }

        // Truncate pile.
        while self.rows.len() > self.truncate {
            self.rows.pop_front();
            self.additional_height += 1;
        }
    }
    fn fits(&self, rock: &Rock) -> bool {
        for &(x, y) in &rock.0 {
            let y = y - self.additional_height;
            if y < 0 {
                return false;
            }
            if y as usize >= self.rows.len() {
                continue;
            }
            if self.rows[y as usize][x as usize] {
                return false;
            }
        }
        true
    }

    fn _print(&self) {
        for row in self.rows.iter().rev() {
            print!("|");
            for &taken in row {
                let c = if taken { '#' } else { '.' };
                print!("{c}");
            }
            println!("|");
        }
        if self.additional_height > 0 {
            println!(
                "vvvvvvvvv ({} rows already discarded)\n",
                self.additional_height
            );
        } else {
            println!("+-------+");
        }
    }
}

/// A falling rock.
#[derive(Debug, Clone)]
struct Rock(Vec<(i64, i64)>);

/// x points right, y points up. Points are (x, y).
/// New rocks have their bottom left corner at (0,0).
impl Rock {
    fn horizontal() -> Self {
        Rock(vec![(0, 0), (1, 0), (2, 0), (3, 0)])
    }
    fn plus() -> Self {
        Rock(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)])
    }
    fn corner() -> Self {
        Rock(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)])
    }
    fn vertical() -> Self {
        Rock(vec![(0, 0), (0, 1), (0, 2), (0, 3)])
    }
    fn square() -> Self {
        Rock(vec![(0, 0), (1, 0), (0, 1), (1, 1)])
    }
    /// Endless stream of rocks in the correct order.
    fn stream() -> impl Iterator<Item = Self> {
        [
            Self::horizontal(),
            Self::plus(),
            Self::corner(),
            Self::vertical(),
            Self::square(),
        ]
        .into_iter()
        .cycle()
    }
    /// Push the rock left/right. Keeps 0 <= x < 7.
    fn shift(&mut self, shift: Shift) {
        match shift {
            Shift::Left => {
                if self.0.iter().any(|&(x, _y)| x <= 0) {
                    // Already all the way left.
                    return;
                }
                self.0.iter_mut().for_each(|(x, _y)| *x -= 1);
            }
            Shift::Right => {
                if self.0.iter().any(|&(x, _y)| x >= 6) {
                    // Already all the way right.
                    return;
                }
                self.0.iter_mut().for_each(|(x, _y)| *x += 1);
            }
        }
    }
    fn add_y(&mut self, dy: i64) {
        self.0.iter_mut().for_each(|(_x, y)| *y += dy);
    }
}

#[derive(Debug, Clone, Copy)]
enum Shift {
    Left,
    Right,
}

impl Neg for Shift {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Shift::Left => Shift::Right,
            Shift::Right => Shift::Left,
        }
    }
}

fn input() -> impl Iterator<Item = Shift> + Clone {
    INPUT.chars().map(|c| match c {
        '<' => Shift::Left,
        '>' => Shift::Right,
        c => panic!("unknown char in input {c:?}"),
    })
}
