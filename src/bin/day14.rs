use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Instr<'a> {
    // mask = 000001001011XX1XX100X0001011X0001101
    Mask(&'a str),
    // mem[27137] = 1696
    Write { addr: i64, val: i64 },
}

fn parse_line(ln: &str) -> Instr {
    // mask = 1000011X10X10X1X1100101X00X011010001
    if ln.starts_with("mask") {
        return Instr::Mask(&ln["mask = ".len()..]);
    }

    assert!(ln.starts_with("mem["));

    // mem[20727] = 25071621
    let parts: Vec<_> = ln.split(" = ").collect();
    Instr::Write {
        addr: parts[0][4..parts[0].len() - 1].parse().unwrap(),
        val: parts[1].parse().unwrap(),
    }
}

fn solve_part1(instrs: &[Instr]) -> i64 {
    let mut mask = "";
    let mut mem: HashMap<i64, i64> = HashMap::new();

    for i in instrs {
        match i {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Write { addr, val } => {
                // mask = 1000011X10X10X1X1100101X00X011010001
                let set_mask: i64 = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .filter(|(_i, c)| *c == '1')
                    .map(|(i, _c)| 2_i64.pow(i as u32))
                    .sum();
                let clear_mask: i64 = mask
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| match c {
                        '0' => 0,
                        _ => 2_i64.pow(i as u32),
                    })
                    .sum();
                let new_val = (val & clear_mask) | set_mask;
                mem.insert(*addr, new_val);
            }
        }
    }
    mem.values().sum()
}

fn solve_part2(instrs: &[Instr]) -> i64 {
    let mut mask = "";
    let mut mem: HashMap<i64, i64> = HashMap::new();

    for i in instrs {
        match i {
            Instr::Mask(new_mask) => mask = new_mask,
            Instr::Write { addr, val } => {
                for addr_float in floating(*addr, 0, mask.as_bytes()) {
                    mem.insert(addr_float, *val);
                }
            }
        }
    }
    mem.values().sum()
}

fn clear_bit(bit: i64, index: usize) -> i64 {
    bit & (!2_i64.pow(index as u32))
}
fn set_bit(bit: i64, index: usize) -> i64 {
    bit | (2_i64.pow(index as u32))
}

fn floating(val: i64, index: usize, mask: &[u8]) -> Vec<i64> {
    if index >= mask.len() {
        return vec![val];
    }
    match mask[mask.len() - 1 - index] {
        b'X' => floating(clear_bit(val, index), index + 1, mask)
            .iter()
            .chain(floating(set_bit(val, index), index + 1, mask).iter())
            .cloned()
            .collect(),
        b'1' => floating(set_bit(val, index), index + 1, mask),
        _ => floating(val, index + 1, mask),
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day14.txt").expect("Something went wrong reading the file");

    let instrs: Vec<_> = contents.lines().map(parse_line).collect();

    println!("Part 1: {}", solve_part1(&instrs));
    println!("Part 2: {}", solve_part2(&instrs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents =
            fs::read_to_string("inputs/day14.txt").expect("Something went wrong reading the file");
        let instrs: Vec<_> = contents.lines().map(parse_line).collect();

        assert_eq!(solve_part1(&instrs), 2346881602152);
    }

    #[test]
    fn test_part2() {
        let contents =
            fs::read_to_string("inputs/day14.txt").expect("Something went wrong reading the file");
        let instrs: Vec<_> = contents.lines().map(parse_line).collect();

        assert_eq!(solve_part2(&instrs), 3885232834169);
    }
}
