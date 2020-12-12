use std::fs;

const N: u8 = b'N';
const E: u8 = b'E';
const S: u8 = b'S';
const W: u8 = b'W';

const L: u8 = b'L';
const R: u8 = b'R';

const F: u8 = b'F';

type Pos = (i32, i32);

fn parse_line(ln: &str) -> (u8, i32) {
    println!("{}", ln);
    let num = ln[1..].parse::<i32>().unwrap();
    (ln.as_bytes()[0], num)
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

fn move_pos(pos: (i32, i32), amount: (i32, i32)) -> (i32, i32) {
    (pos.0 + amount.0, pos.1 + amount.1)
}

fn move_in_dir(pos: (i32, i32), num: i32, dir: Direction) -> (i32, i32) {
    let amount = match dir {
        Direction::North => (0, -num),
        Direction::South => (0, num),
        Direction::West => (-num, 0),
        Direction::East => (num, 0),
    };

    move_pos(pos, amount)
}

fn rotate(dir: Direction, steps: i32, sign: i32) -> Direction {
    println!("{:?} {} {}", dir, steps, sign);
    let i = dir as i32;
    let j = i + steps * sign + 4;
    println!("{} {}", i, j);
    match j % 4 {
        0 => Direction::East,
        1 => Direction::South,
        2 => Direction::West,
        3 => Direction::North,
        _ => panic!("oh noes"),
    }
}

fn one_step(pos: Pos, dir: Direction, i: u8, num: i32) -> (Pos, Direction) {
    println!("{} {}", i, num);
    match i {
        N => (move_in_dir(pos, num, Direction::North), dir),
        S => (move_in_dir(pos, num, Direction::South), dir),
        W => (move_in_dir(pos, num, Direction::West), dir),
        E => (move_in_dir(pos, num, Direction::East), dir),
        F => (move_in_dir(pos, num, dir), dir),
        R => (pos, rotate(dir, num / 90, 1)),
        L => (pos, rotate(dir, num / 90, -1)),
        _ => panic!("unknown instruction {instr}", instr = i),
    }
}

fn main() {
    let contents = fs::read_to_string("day12.txt").expect("Something went wrong reading the file");

    let instr: Vec<(u8, i32)> = contents.lines().map(parse_line).collect();

    let init_dir: Direction = Direction::East;

    let init_pos: (i32, i32) = (0, 0);

    let (pos, _) = instr
        .iter()
        .fold((init_pos, init_dir), |(pos, dir), (i, num)| {
            one_step(pos, dir, *i, *num)
        });

    println!(
        "{}x{} = manhattan {}",
        pos.0,
        pos.1,
        pos.0.abs() + pos.1.abs()
    )
}
