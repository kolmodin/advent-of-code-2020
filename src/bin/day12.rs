use aoc2020::pos2d::Pos;
use std::fs;

const N: u8 = b'N';
const E: u8 = b'E';
const S: u8 = b'S';
const W: u8 = b'W';

const L: u8 = b'L';
const R: u8 = b'R';

const F: u8 = b'F';

fn parse_line(ln: &str) -> (u8, i32) {
    let num = ln[1..].parse::<i32>().unwrap();
    (ln.as_bytes()[0], num)
}

fn eval_part1(instrs: &[(u8, i32)]) -> i32 {
    let mut ship = Pos::origo();
    let mut dir = Pos::east();

    for (instr, num) in instrs {
        match *instr {
            N => ship += Pos::north() * *num,
            S => ship += Pos::south() * *num,
            W => ship += Pos::west() * *num,
            E => ship += Pos::east() * *num,
            F => ship += dir * *num,
            R => dir = rotate(dir, num / 90, false),
            L => dir = rotate(dir, num / 90, true),
            _ => panic!("unknown instruction {instr}", instr = instr),
        }
    }

    ship.manhattan()
}

fn eval_part2(instrs: &[(u8, i32)]) -> i32 {
    let mut waypoint = Pos::new(10, -1);
    let mut ship = Pos::origo();

    for (instr, num) in instrs {
        match *instr {
            N => waypoint += Pos::north() * *num,
            S => waypoint += Pos::south() * *num,
            W => waypoint += Pos::west() * *num,
            E => waypoint += Pos::east() * *num,
            F => ship += waypoint * *num,
            R => waypoint = rotate(waypoint, num / 90, false),
            L => waypoint = rotate(waypoint, num / 90, true),
            _ => panic!("unknown instruction {instr}", instr = instr),
        }
    }
    ship.manhattan()
}

fn rotate(mut dir: Pos, times: i32, left: bool) -> Pos {
    for _ in 0..times {
        if left {
            dir = dir.rotate_left();
        } else {
            dir = dir.rotate_right();
        }
    }
    dir
}

fn main() {
    let contents =
        fs::read_to_string("inputs/day12.txt").expect("Something went wrong reading the file");
    let instrs: Vec<(u8, i32)> = contents.lines().map(parse_line).collect();

    println!("Part 1: {}", eval_part1(&instrs),);
    println!("Part 2: {}", eval_part2(&instrs),);
}
