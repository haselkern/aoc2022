use std::collections::HashMap;

use aoc2022::*;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/18");

fn main() {
    let mut drop = input();
    solved_level_1(count_surface(&drop, Material::Air));
    flood(&mut drop);
    solved_level_2(count_surface(&drop, Material::Water));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Material {
    Lava,
    Water,
    Air,
}

type Voxel = (i64, i64, i64);

type Droplet = HashMap<Voxel, Material>;

const NEIGHBORS: [Voxel; 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn flood(drop: &mut Droplet) {
    let (mut min_x, mut max_x) = drop
        .keys()
        .map(|&(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (mut min_y, mut max_y) = drop
        .keys()
        .map(|&(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let (mut min_z, mut max_z) = drop
        .keys()
        .map(|&(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    // Expand the domain a bit to submerge the entire droplet.
    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;
    min_z -= 1;
    max_z += 1;

    let mut front = vec![(min_x, min_y, min_z)];
    while let Some((x, y, z)) = front.pop() {
        if x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z {
            // Out of bounds
            continue;
        }

        if drop.get(&(x, y, z)).is_some() {
            // Voxel already taken.
            continue;
        }

        drop.insert((x, y, z), Material::Water);
        for (nx, ny, nz) in NEIGHBORS {
            front.push((x + nx, y + ny, z + nz));
        }
    }
}

fn count_surface(drop: &Droplet, with: Material) -> usize {
    let mut sum = 0;
    for &(x, y, z) in drop.iter().filter_map(|(v, m)| match m {
        Material::Lava => Some(v),
        _ => None,
    }) {
        let mut surface = 0;
        for &(nx, ny, nz) in &NEIGHBORS {
            let nx = nx + x;
            let ny = ny + y;
            let nz = nz + z;
            let neighbor = drop.get(&(nx, ny, nz)).copied().unwrap_or(Material::Air);
            if neighbor == with {
                surface += 1;
            }
        }
        sum += surface;
    }

    sum
}

fn input() -> Droplet {
    INPUT
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|n| n.parse().unwrap());
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            let z = parts.next().unwrap();
            (x, y, z)
        })
        .map(|v| (v, Material::Lava))
        .collect()
}
