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

fn move_pos(pos: Pos, waypoint: Pos, scale: i32) -> Pos {
    (pos.0 + waypoint.0 * scale, pos.1 + waypoint.1 * scale)
}

fn rotate(waypoint: Pos, steps: i32, clockwise: i32) -> Pos {
    let mut x = waypoint.0;
    let mut y = waypoint.1;

    for _ in 1..=steps {
        let new_x = y * clockwise;
        let new_y = -x * clockwise;
        x = new_x;
        y = new_y;
    }

    (x, y)
}

fn one_step(pos: Pos, waypoint: Pos, i: u8, num: i32) -> (Pos, Pos) {
    let result = match i {
        N => (pos, move_pos(waypoint, (0, -num), 1)),
        S => (pos, move_pos(waypoint, (0, num), 1)),
        W => (pos, move_pos(waypoint, (-num, 0), 1)),
        E => (pos, move_pos(waypoint, (num, 0), 1)),
        F => (move_pos(pos, waypoint, num), waypoint),
        R => (pos, rotate(waypoint, num / 90, -1)),
        L => (pos, rotate(waypoint, num / 90, 1)),
        _ => panic!("unknown instruction {instr}", instr = i),
    };
    println!("pos={:?} waypoint={:?}", result.0, result.1);
    result
}

fn main() {
    let contents = fs::read_to_string("day12.txt").expect("Something went wrong reading the file");
    let instr: Vec<(u8, i32)> = contents.lines().map(parse_line).collect();

    let init_waypoint: Pos = (10, -1);
    let init_pos: (i32, i32) = (0, 0);

    println!("start: pos={:?} waypoint={:?}", init_pos, init_waypoint);

    let init_pos: (i32, i32) = (0, 0);

    let (pos, _) = instr
        .iter()
        .fold((init_pos, init_waypoint), |(pos, waypoint), (i, num)| {
            one_step(pos, waypoint, *i, *num)
        });

    println!(
        "{}x{} = manhattan {}",
        pos.0,
        pos.1,
        pos.0.abs() + pos.1.abs()
    )
}
