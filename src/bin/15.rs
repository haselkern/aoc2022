use aoc2022::*;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use regex::Regex;

// const INPUT: &str = include_str!("../../input/15-test");
// const Y: i64 = 10;
// const MAX_COORD: i64 = 20;

const INPUT: &str = include_str!("../../input/15");
const Y: i64 = 2_000_000;
const MAX_COORD: i64 = 4_000_000;

fn main() {
    solved_level_1(level1());
    solved_level_2(level2());
}

fn level1() -> usize {
    let sensors = input().collect_vec();
    let (min, max) = sensors
        .iter()
        .flat_map(|s| [s.x - s.beacon_distance(), s.x + s.beacon_distance()])
        .minmax()
        .into_option()
        .unwrap();

    (min..=max)
        .filter(|&x| {
            sensors.iter().any(|s| {
                s.distance(x, Y) <= s.beacon_distance() && !(x == s.beacon_x && Y == s.beacon_y)
            })
        })
        .count()
}

fn level2() -> i64 {
    // Efficient solution? 🚫
    // Fancy progress bar? ✅

    let style = ProgressStyle::with_template("[{bar:60}] {percent}%, ~{eta}")
        .unwrap()
        .progress_chars("❌🔎🔍🔎🔍🔎🔍❓");
    let bar = ProgressBar::new(MAX_COORD as u64).with_style(style);

    (0..=MAX_COORD)
        .into_par_iter()
        .flat_map(|y| {
            bar.inc(1);

            let mut x = 0;
            // Push x to the right for each sensor range.
            let mut moved = true;
            while moved {
                moved = false;
                for sensor in input() {
                    if sensor.distance(x, y) <= sensor.beacon_distance() {
                        let rightmost = sensor.x + sensor.beacon_distance();
                        let y_diff = (sensor.y - y).abs();
                        let next_x = rightmost - y_diff;
                        x = x.max(next_x) + 1;
                        moved = true;
                    }
                }
                if x > MAX_COORD {
                    break;
                }
            }

            if x <= MAX_COORD {
                println!("Found x={x}, y={y}");
                Some(x * 4000000 + y)
            } else {
                None
            }
        })
        .find_any(|_| true)
        .unwrap()
}

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
}

impl Sensor {
    fn distance(&self, x: i64, y: i64) -> i64 {
        (self.x.abs_diff(x) + self.y.abs_diff(y)) as i64
    }

    fn beacon_distance(&self) -> i64 {
        self.distance(self.beacon_x, self.beacon_y)
    }
}

fn input() -> impl Iterator<Item = Sensor> {
    let re =
        Regex::new(r"^Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)$").unwrap();
    INPUT
        .lines()
        .map(move |line| re.captures(line).unwrap())
        .map(|cap| Sensor {
            x: cap.get(1).unwrap().as_str().parse().unwrap(),
            y: cap.get(2).unwrap().as_str().parse().unwrap(),
            beacon_x: cap.get(3).unwrap().as_str().parse().unwrap(),
            beacon_y: cap.get(4).unwrap().as_str().parse().unwrap(),
        })
}
