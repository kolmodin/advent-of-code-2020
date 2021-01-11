use std::fs;

#[derive(Clone, Copy)]
enum Instr {
    NOp(i32),
    Acc(i32),
    Jmp(i32),
}

enum Exit {
    Terminated(i32),
    Seen(i32),
}

fn parse_line(ln: &str) -> Instr {
    let parts: Vec<_> = ln.split(' ').collect();
    assert_eq!(parts.len(), 2);
    let num = parts[1].parse().unwrap();
    match parts[0] {
        "nop" => Instr::NOp(num),
        "acc" => Instr::Acc(num),
        "jmp" => Instr::Jmp(num),
        _ => panic!("unknown instruction {instr}", instr = ln),
    }
}

fn run(program: &[Instr]) -> Exit {
    let mut acc = 0;
    let mut pc = 0;
    let mut seen = vec![false; program.len()];
    let term_pc = program.len();
    while !seen.get(pc).cloned().unwrap_or(false) && pc != term_pc {
        seen[pc] = true;
        match program[pc] {
            Instr::NOp(_) => pc += 1,
            Instr::Jmp(num) => pc = (pc as i32 + num) as usize,
            Instr::Acc(num) => { 
                acc += num; 
                pc += 1;
            },
        }
    }
    if pc == term_pc {
        Exit::Terminated(acc)
    } else{
        Exit::Seen(acc)
    }
}

fn part1(program: &[Instr]) -> i32 {
    if let Exit::Seen(num) = run(program) {
        num
    } else {
        panic!("unexpected exit code in part1");
    }
}


fn part2(program: &mut [Instr]) -> i32 {
    for i in 0..program.len() {
        let op = program[i];
        let new_op = match op {
            Instr::NOp(num) => Some(Instr::Jmp(num)),
            Instr::Jmp(num) => Some(Instr::NOp(num)),
            _ => None,
        };
        if !new_op.is_some() {
            continue;
        }
        program[i] = new_op.unwrap();
        if let Exit::Terminated(acc) = run(program) {
            return acc;
        }
        program[i] = op;
    }
    panic!("part2: couldn't make program terminate.")
}

fn main() {
    let contents = fs::read_to_string("inputs/day08.txt").expect("Something went wrong reading the file");

    let mut program: Vec<Instr> = contents.lines().map(parse_line).collect();

    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&mut program));
}
