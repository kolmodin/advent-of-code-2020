
use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use std::hash::Hash;

use itertools::iproduct;

type NeighborCount<P> = HashMap<P, i32>;
type Alive<P> = HashSet<P>;

trait PosT
where
    Self: Sized + Eq + Hash + Clone,
{
    fn new(x: i32, y: i32) -> Self;
    fn add_neighbors(&self, counter: &mut NeighborCount<Self>);
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos3 {
    x: i32,
    y: i32,
    z: i32,
}

impl PosT for Pos3 {
    fn new(x: i32, y: i32) -> Pos3 {
        Pos3 { x, y, z: 0 }
    }
    fn add_neighbors(&self, counter: &mut NeighborCount<Pos3>) {
        for (x, y, z) in iproduct!(-1..=1, -1..=1, -1..=1) {
            if x == 0 && y == 0 && z == 0 {
                continue;
            }
            let p = Pos3 {
                x: self.x + x,
                y: self.y + y,
                z: self.z + z,
            };
            let entry = counter.entry(p).or_insert(0);
            *entry += 1;
        }
    }
}
#[derive(Eq, PartialEq, Hash, Clone)]
struct Pos4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl PosT for Pos4 {
    fn new(x: i32, y: i32) -> Pos4 {
        Pos4 { x, y, z: 0, w: 0 }
    }
    fn add_neighbors(&self, counter: &mut NeighborCount<Pos4>) {
        for (x, y, z, w) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
            if x == 0 && y == 0 && z == 0 && w == 0 {
                continue;
            }
            let p = Pos4 {
                x: self.x + x,
                y: self.y + y,
                z: self.z + z,
                w: self.w + w,
            };
            let entry = counter.entry(p).or_insert(0);
            *entry += 1;
        }
    }
}

fn map_to_alive<Pos: PosT>(input: &str) -> Alive<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Pos::new(x as i32, y as i32))
        })
        .collect()
}

fn evolve<Pos: PosT>(alive: Alive<Pos>) -> Alive<Pos> {
    alive_to_neighbors(&alive)
        .into_iter()
        .filter_map(|(pos, count)| match count {
            3 => Some(pos),
            2 if alive.contains(&pos) => Some(pos),
            _ => None,
        })
        .collect()
}

fn alive_to_neighbors<Pos: PosT>(alive: &Alive<Pos>) -> NeighborCount<Pos> {
    let mut hm = HashMap::new();
    for cell in alive.iter() {
        cell.add_neighbors(&mut hm);
    }
    hm
}

fn eval_part1(inp: &str) -> usize {
    let mut map: Alive<Pos3> = map_to_alive(&inp);
    for _ in 0..6 {
        map = evolve(map);
    }
    map.len()
}

fn eval_part2(inp: &str) -> usize {
    let mut map: Alive<Pos4> = map_to_alive(&inp);
    for _ in 0..6 {
        map = evolve(map);
    }
    map.len()
}

fn main() {
    let contents = fs::read_to_string("day17.txt").expect("Something went wrong reading the file");

    println!("Part 1: {}", eval_part1(&contents));
    println!("Part 2: {}", eval_part2(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17() {
        let contents =
            fs::read_to_string("day17.txt").expect("Something went wrong reading the file");
        assert_eq!(eval_part1(&contents), 382);
        assert_eq!(eval_part2(&contents), 2552);
    }
}
