use std::str::FromStr;

use aoc2022::*;

const INPUT: &str = include_str!("../../input/07");

fn main() {
    let tree = Tree::from_input();
    solved_level_1(tree.part1());
    solved_level_2(tree.part2());
}

#[derive(Debug)]
struct Tree(Vec<Node>);

impl Tree {
    fn from_input() -> Self {
        let mut tree = vec![Node::directory(0)];
        let mut current = 0;

        // Skip 1 skips the initial "cd /".
        let input = INPUT.lines().filter_map(|l| l.parse::<Line>().ok()).skip(1);

        for line in input {
            match line {
                Line::CommandUp => current = tree[current].parent(),
                Line::CommandInto => {
                    let handle = tree.len();
                    tree.push(Node::directory(current));
                    tree[current].add_child(handle);
                    current = handle;
                }
                Line::File(size) => {
                    let handle = tree.len();
                    tree.push(Node::file(current, size));
                    tree[current].add_child(handle);
                }
            }
        }

        Self(tree)
    }

    fn dir_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, node)| match node {
                Node::Directory { .. } => Some(i),
                Node::File { .. } => None,
            })
            .map(|i| self.sum_size(i))
    }

    fn part1(&self) -> usize {
        self.dir_sizes().filter(|s| *s <= 100000).sum()
    }

    fn part2(&self) -> usize {
        let used = self.sum_size(0);
        let unused = 70000000 - used;
        let needed = 30000000 - unused;

        self.dir_sizes().filter(|s| *s >= needed).min().unwrap()
    }

    fn sum_size(&self, i: usize) -> usize {
        match &self.0[i] {
            Node::Directory { children, .. } => children.iter().map(|c| self.sum_size(*c)).sum(),
            Node::File { size, .. } => *size,
        }
    }
}

#[derive(Debug)]
enum Node {
    Directory { parent: usize, children: Vec<usize> },
    File { parent: usize, size: usize },
}

impl Node {
    fn add_child(&mut self, child: usize) {
        match self {
            Node::Directory { children, .. } => children.push(child),
            Node::File { .. } => panic!("cannot add child to file"),
        }
    }
    fn directory(parent: usize) -> Self {
        Self::Directory {
            parent,
            children: Vec::new(),
        }
    }
    fn file(parent: usize, size: usize) -> Self {
        Self::File { parent, size }
    }
    fn parent(&self) -> usize {
        match self {
            Node::Directory { parent, .. } => *parent,
            Node::File { parent, .. } => *parent,
        }
    }
}

#[derive(Debug)]
enum Line {
    CommandUp,
    CommandInto,
    File(usize),
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "$ cd .." {
            Ok(Self::CommandUp)
        } else if s.starts_with("$ cd ") {
            Ok(Self::CommandInto)
        } else if s == "$ ls" || s.starts_with("dir") {
            // Ignore ls and directories
            Err(())
        } else {
            let size: usize = s.split(' ').next().unwrap().parse().unwrap();
            Ok(Self::File(size))
        }
    }
}
