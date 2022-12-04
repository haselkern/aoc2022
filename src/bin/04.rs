use aoc2022::*;

const INPUT: &str = include_str!("../../input/04");

fn main() {
    let solution = INPUT
        .lines()
        .map(parse_line)
        .filter(|&(a, b)| a.contains(b) || b.contains(a))
        .count();
    solved_level_1(solution);

    let solution = INPUT
        .lines()
        .map(parse_line)
        .filter(|&(a, b)| a.overlaps(b))
        .count();
    solved_level_2(solution);
}

fn parse_line(line: &str) -> (Range, Range) {
    let (a, b) = line.split_once(',').unwrap();
    (a.into(), b.into())
}

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn contains<O>(self, other: O) -> bool
    where
        O: Into<Self>,
    {
        let other = other.into();
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(self, other: Self) -> bool {
        self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

impl From<i64> for Range {
    fn from(n: i64) -> Self {
        Self { start: n, end: n }
    }
}
