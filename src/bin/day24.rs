use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z }
    }
    fn origo() -> Self {
        Cube { x: 0, y: 0, z: 0 }
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PosT for Cube {
    fn add_neighbors(&self, counter: &mut NeighborCount<Self>) {
        for dir in &["w", "e", "nw", "ne", "sw", "se"] {
            let coord = *self + lookup(dir);
            let entry = counter.entry(coord).or_insert(0);
            *entry += 1;
        }
    }
}

fn lookup(inp: &str) -> Cube {
    match inp {
        "w" => Cube::new(-1, 1, 0),
        "e" => Cube::new(1, -1, 0),
        "nw" => Cube::new(0, 1, -1),
        "ne" => Cube::new(1, 0, -1),
        "sw" => Cube::new(-1, 0, 1),
        "se" => Cube::new(0, -1, 1),
        _ => panic!("unknown coordinate {inp}", inp = inp),
    }
}

fn parse_line(mut inp: &str) -> Vec<Cube> {
    let mut path = vec![];

    while !inp.is_empty() {
        path.push(match &inp[0..1] {
            "w" => {
                inp = &inp[1..];
                lookup("w")
            }
            "e" => {
                inp = &inp[1..];
                lookup("e")
            }
            _ => {
                let c = lookup(&inp[0..2]);
                inp = &inp[2..];
                c
            }
        });
    }
    path
}

fn toggle(floor: &mut HashSet<Cube>, cube: Cube) {
    match floor.get(&cube) {
        Some(_) => floor.remove(&cube),
        None => floor.insert(cube),
    };
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day24.txt").expect("Something went wrong reading the file");

    let paths: Vec<Vec<Cube>> = contents.lines().map(parse_line).collect();

    let mut floor = HashSet::new();

    for path in paths {
        let coord = path.iter().fold(Cube::origo(), |acc, c| acc + *c);
        toggle(&mut floor, coord);
    }

    println!("Path 1: {}", floor.len());
    println!("Path 2: {}", eval_part2(&contents));
}

// Adapted from day 17.

type NeighborCount<P> = HashMap<P, i32>;
type Alive<P> = HashSet<P>;

trait PosT
where
    Self: Sized + Eq + Hash + Clone,
{
    fn add_neighbors(&self, counter: &mut NeighborCount<Self>);
}

fn map_to_alive(input: &str) -> Alive<Cube> {
    let mut floor = HashSet::new();

    let coords = input
        .lines()
        .map(parse_line)
        .map(|path| path.iter().fold(Cube::origo(), |acc, c| acc + *c));

    for coord in coords {
        toggle(&mut floor, coord);
    }

    floor
}

fn evolve(alive: Alive<Cube>) -> Alive<Cube> {
    /*
    Any black tile with:
      zero or more than 2 black tiles immediately adjacent to it
       -> is flipped to white.
    Any white tile with:
      exactly 2 black tiles immediately adjacent to it
       -> is flipped to black.

    white == dead
    black == alive
    */

    alive_to_neighbors(&alive)
        .into_iter()
        .filter_map(|(pos, count)| {
            let is_black = || alive.contains(&pos);
            let black = Some(pos);
            let white = None;
            match count {
                2 => black,
                1 if is_black() => black,
                _ => white,
            }
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

fn eval_part2(inp: &str) -> usize {
    let mut map: Alive<Cube> = map_to_alive(&inp);
    for _ in 0..100 {
        map = evolve(map);
    }
    map.len()
}
