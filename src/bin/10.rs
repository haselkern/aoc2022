use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/10");

fn main() {
    let solution: i64 = Cpu::load(input())
        .filter(|(cycle, _x)| [20, 60, 100, 140, 180, 220].contains(cycle))
        .map(|(cycle, x)| cycle * x)
        .sum();
    solved_level_1(solution);

    Cpu::load(input()).for_each(|(cycle, x)| {
        let screen_x = (cycle - 1) % 40;

        if screen_x.abs_diff(x) <= 1 {
            print!("█");
        } else {
            print!(" ");
        }

        if screen_x == 39 {
            println!();
        }
    });
}

struct Cpu {
    x: i64,
    instruction_stack: Vec<Instruction>,
    cycle: i64,
}

impl Cpu {
    fn load<I>(it: I) -> Self
    where
        I: IntoIterator<Item = Instruction>,
    {
        let mut instructions = it.into_iter().collect_vec();
        instructions.reverse();
        Self {
            x: 1,
            instruction_stack: instructions,
            cycle: 0,
        }
    }
}

impl Iterator for Cpu {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        self.cycle += 1;

        let Some(op) = self.instruction_stack.pop() else {
            return None;
        };

        let result = (self.cycle, self.x);

        match op {
            Instruction::Nop => {}
            Instruction::AddX(n) => self.instruction_stack.push(Instruction::DoAddX(n)),
            Instruction::DoAddX(n) => self.x += n,
        }

        Some(result)
    }
}

enum Instruction {
    Nop,
    AddX(i64),
    DoAddX(i64),
}

fn input() -> impl Iterator<Item = Instruction> {
    INPUT.lines().map(|line| {
        if line.starts_with("addx") {
            let n = line.split(' ').nth(1).unwrap();
            Instruction::AddX(n.parse().unwrap())
        } else {
            Instruction::Nop
        }
    })
}
