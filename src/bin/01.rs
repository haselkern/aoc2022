use aoc2022::*;

const INPUT: &str = include_str!("../../input/01");

fn main() {
    let calories = INPUT.lines().map(|l| l.parse::<i64>().ok());

    let mut elves = Vec::new();
    let mut current = 0;
    for c in calories {
        match c {
            Some(c) => current += c,
            None => {
                elves.push(current);
                current = 0;
            }
        }
    }

    // Sort highest number first
    elves.sort_by_key(|e| -e);

    let max = elves[0];
    solved_level_1(max);

    let sum: i64 = elves.iter().take(3).sum();
    solved_level_2(sum);
}
