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

fn move_pos(pos: Pos, waypoint: Pos, scale: i32) -> Pos {
    (pos.0 + waypoint.0 * scale, pos.1 + waypoint.1 * scale)
}

fn forward(pos: Pos, num: i32, dir: Direction) -> Pos {
    let amount = match dir {
        Direction::North => (0, -num),
        Direction::South => (0, num),
        Direction::West => (-num, 0),
        Direction::East => (num, 0),
    };

    move_pos(pos, amount, 1)
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

    // let new_dir = match j % 4 {
    //     0 => Direction::East,
    //     1 => Direction::South,
    //     2 => Direction::West,
    //     3 => Direction::North,
    //     _ => panic!("oh noes"),
    // };
    // println!("{:?}", new_dir);
    // new_dir
}

fn main() {
    let contents = fs::read_to_string("day12.txt").expect("Something went wrong reading the file");
//     let contents = "F10
// N3
// F7
// R90
// F11";

    let instr: Vec<(u8, i32)> = contents.lines().map(parse_line).collect();

    // let mut dir: Direction = Direction::East;

    let mut waypoint: Pos = (10, -1);
    let mut pos: (i32, i32) = (0, 0);

    println!("start: pos={:?} waypoint={:?}", pos, waypoint);

    for (i, num) in instr {
        println!("{} {}", i, num);
        match i {
            N => {
                waypoint = move_pos(waypoint, (0, -num), 1);
            }
            S => {
                waypoint = move_pos(waypoint, (0, num), 1);
            }
            W => {
                waypoint = move_pos(waypoint, (-num, 0), 1);
            }
            E => {
                waypoint = move_pos(waypoint, (num, 0), 1);
            }
            F => {
                pos = move_pos(pos, waypoint, num);
            }
            R => {
                waypoint = rotate(waypoint, num / 90, -1);
            }
            L => {
                waypoint = rotate(waypoint, num / 90, 1);
            }
            _ => panic!("unknown instruction {instr}", instr = i),
        }
        println!("pos={:?} waypoint={:?}", pos, waypoint);
    }

    println!(
        "{}x{} = manhattan {}",
        pos.0,
        pos.1,
        pos.0.abs() + pos.1.abs()
    )
}
