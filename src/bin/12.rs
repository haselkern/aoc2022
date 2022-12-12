use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/12");

fn main() {
    let mut hill = Hill::from_input();
    hill.mark_dijkstra();
    solved_level_1(hill.level1());
    solved_level_2(hill.level2());
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Node {
    chr: char,
    visited: bool,
    distance: usize,
    position: Position,
}

#[derive(Debug)]
struct Hill {
    nodes: Vec<Vec<Node>>,
    start: Position,
    end: Position,
}

impl Hill {
    fn level1(&self) -> usize {
        self.get(self.start).distance
    }

    fn level2(&self) -> usize {
        self.nodes
            .iter()
            .flat_map(|row| row.iter())
            .filter(|n| n.chr == 'a')
            .map(|n| n.distance)
            .min()
            .unwrap()
    }

    // Mark each node with its distance to the end.
    fn mark_dijkstra(&mut self) {
        self.get_mut(self.end).distance = 0;

        loop {
            // Find unvisited node with minimal distance
            let subject = self
                .nodes
                .iter()
                .flat_map(|row| row.iter())
                .filter(|n| !n.visited)
                .filter(|n| n.distance < usize::MAX)
                .min_by_key(|n| n.distance)
                .cloned();
            // If there is no such node then we are done.
            let Some(subject) = subject else { return };

            self.get_mut(subject.position).visited = true;

            // Find and update neighbors
            let mut neighbors = Vec::with_capacity(4);
            if subject.position.x > 0 {
                neighbors.push(Position::new(subject.position.x - 1, subject.position.y));
            }
            if subject.position.x < self.nodes[0].len() - 1 {
                neighbors.push(Position::new(subject.position.x + 1, subject.position.y));
            }
            if subject.position.y > 0 {
                neighbors.push(Position::new(subject.position.x, subject.position.y - 1));
            }
            if subject.position.y < self.nodes.len() - 1 {
                neighbors.push(Position::new(subject.position.x, subject.position.y + 1));
            }
            let neighbors = neighbors
                .into_iter()
                .filter(|&n| !self.get(n).visited)
                .filter(|&n| climbable(self.get(n).chr, subject.chr))
                .collect_vec();

            let new_distance = subject.distance + 1;
            for neighbor in neighbors {
                let mut neighbor = self.get_mut(neighbor);
                neighbor.distance = neighbor.distance.min(new_distance);
            }
        }
    }

    fn get_mut(&mut self, p: Position) -> &mut Node {
        &mut self.nodes[p.y][p.x]
    }

    fn get(&self, p: Position) -> &Node {
        &self.nodes[p.y][p.x]
    }

    fn from_input() -> Self {
        let mut start = None;
        let mut end = None;

        let nodes = INPUT
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let position = Position::new(x, y);

                        if c == 'S' {
                            start = Some(position);
                        } else if c == 'E' {
                            end = Some(position);
                        }

                        Node {
                            chr: c,
                            visited: false,
                            distance: usize::MAX,
                            position,
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            nodes,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }
}

fn climbable(from: char, to: char) -> bool {
    let from = convert(from);
    let to = convert(to);
    if to < from {
        return true;
    }

    from.abs_diff(to) <= 1
}

fn convert(c: char) -> u32 {
    let c = match c {
        'S' => 'a',
        'E' => 'z',
        c => c,
    };
    c as u32
}
