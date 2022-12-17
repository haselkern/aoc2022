use std::{collections::VecDeque, ops::Neg};

use aoc2022::*;
use indicatif::{ProgressBar, ProgressStyle};

const INPUT: &str = include_str!("../../input/17");

fn main() {
    level1();
    level2();
}

fn level1() {
    let rocks = Rock::stream().take(2022);
    let mut jet = jet();
    let mut pile = Pile::default();

    let mut max_drop = 0;

    for rock in rocks {
        let dropped = pile.drop(rock, &mut jet);
        max_drop = max_drop.max(dropped);
    }

    println!("The most a rock dropped was by {max_drop} spaces");
    solved_level_1(pile.height());
}

fn level2() {
    let count = 1000000000000;
    let rocks = Rock::stream().take(count);
    let mut jet = jet();
    let mut pile = Pile::default();

    let style = ProgressStyle::with_template("Solving level 2 [{bar:40}] {percent}%, ~{eta}")
        .unwrap()
        .progress_chars("=> ");
    let bar = ProgressBar::new(count as u64).with_style(style);

    for rock in rocks {
        bar.inc(1);
        pile.drop(rock, &mut jet);
    }

    solved_level_2(pile.height());
}

/// The pile of already placed rocks.
#[derive(Default)]
struct Pile {
    rows: VecDeque<[bool; 7]>,
    additional_height: i64,
}

impl Pile {
    /// Drop a new rock into the pile. The rock is assumed to be untampered.
    /// Returns the number of spaces the rock dropped down.
    fn drop(&mut self, mut rock: Rock, jet: &mut impl Iterator<Item = Shift>) -> usize {
        // Rocks spawn 2 from the left wall and 3 above the pile.
        rock.shift(Shift::Right);
        rock.shift(Shift::Right);
        rock.add_y(self.height() + 3);
        // println!("Spawned: {rock:?}");

        let mut dropped = 0;

        loop {
            let shift = jet.next().unwrap();
            rock.shift(shift);
            // println!("Pushed rock {shift:?}: {rock:?}");
            if !self.fits(&rock) {
                // println!("Nevermind, rock in the way.");
                // Oopsy, revert that.
                rock.shift(-shift);
            }

            rock.add_y(-1);
            dropped += 1;
            // println!("Dropped rock: {rock:?}");
            if !self.fits(&rock) {
                // The rock could no longer drop down. Revert and place it there.
                rock.add_y(1);
                dropped -= 1;
                // println!("Nevermind, that rock gets settled higher up.");
                self.place(rock);
                // self.print();
                return dropped;
            }
        }
    }
    fn height(&self) -> i64 {
        self.rows.len() as i64 + self.additional_height
        // self.placed.iter().map(|&(_x, y)| y).max().unwrap_or(0)
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

        // Truncate tower. Number found by using the maximum length a rock dropped and doubling that.
        while self.rows.len() > 70 {
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

fn jet() -> impl Iterator<Item = Shift> {
    INPUT
        .chars()
        .map(|c| match c {
            '<' => Shift::Left,
            '>' => Shift::Right,
            c => panic!("unknown char in input {c:?}"),
        })
        .cycle()
}
