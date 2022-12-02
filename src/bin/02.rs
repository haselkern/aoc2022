use aoc2022::*;

const INPUT: &str = include_str!("../../input/02");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("unknown char: {c}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Result {
    Win,
    Draw,
    Lose,
}

impl From<char> for Result {
    fn from(c: char) -> Self {
        match c {
            'X' => Result::Lose,
            'Y' => Result::Draw,
            'Z' => Result::Win,
            _ => panic!("unknown char: {c}"),
        }
    }
}

fn main() {
    let total: i64 = INPUT
        .lines()
        .map(|l| l.chars())
        .map(|mut c| {
            let elf: Hand = c.next().unwrap().into();
            let me: Hand = c.last().unwrap().into();

            match (me, elf) {
                (Hand::Rock, Hand::Rock) => 4,
                (Hand::Rock, Hand::Paper) => 1,
                (Hand::Rock, Hand::Scissors) => 7,
                (Hand::Paper, Hand::Rock) => 8,
                (Hand::Paper, Hand::Paper) => 5,
                (Hand::Paper, Hand::Scissors) => 2,
                (Hand::Scissors, Hand::Rock) => 3,
                (Hand::Scissors, Hand::Paper) => 9,
                (Hand::Scissors, Hand::Scissors) => 6,
            }
        })
        .sum();
    solved_level_1(total);

    let total: i64 = INPUT
        .lines()
        .map(|l| l.chars())
        .map(|mut c| {
            let elf: Hand = c.next().unwrap().into();
            let result: Result = c.last().unwrap().into();

            match (elf, result) {
                (Hand::Rock, Result::Win) => 8,
                (Hand::Rock, Result::Draw) => 4,
                (Hand::Rock, Result::Lose) => 3,
                (Hand::Paper, Result::Win) => 9,
                (Hand::Paper, Result::Draw) => 5,
                (Hand::Paper, Result::Lose) => 1,
                (Hand::Scissors, Result::Win) => 7,
                (Hand::Scissors, Result::Draw) => 6,
                (Hand::Scissors, Result::Lose) => 2,
            }
        })
        .sum();
    solved_level_2(total);
}
