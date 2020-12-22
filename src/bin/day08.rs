use std::fs;

#[derive(Clone)]
enum Instr {
    NOp,
    Acc(i32),
    Jmp(i32),
}

fn parse_line(ln: &str) -> Instr {
    let parts: Vec<_> = ln.split(' ').collect();
    assert_eq!(parts.len(), 2);
    match parts[0] {
        "nop" => Instr::NOp,
        "acc" => Instr::Acc(parts[1].parse().unwrap()),
        "jmp" => Instr::Jmp(parts[1].parse().unwrap()),
        _ => panic!("unknown instruction {instr}", instr = ln),
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day08.txt").expect("Something went wrong reading the file");

    let instrs: Vec<Instr> = contents.lines().map(|ln| parse_line(ln)).collect();

    for i in 0..instrs.len() {
        let mut my_instrs = instrs.clone();
        if let Instr::Jmp(_) = my_instrs[i] {
            my_instrs[i] = Instr::NOp;
        } else {
            continue;
        }

        let mut visited: Vec<bool> = my_instrs.iter().map(|_| false).collect();

        let mut acc: i32 = 0;
        let mut pc: i32 = 0;
        let mut pc_before_jmp: Vec<i32> = Vec::new();
        loop {
            if visited[pc as usize] {
                println!("second time running pc={}, acc={}", pc, acc);
                break;
            }
            if pc >= (my_instrs.len() as i32) - 1 {
                println!("reached end of program, pc={}, acc={}", pc, acc);
                return;
            }
            visited[pc as usize] = true;
            let instr = my_instrs.get(pc as usize).unwrap();
            match instr {
                Instr::NOp => {
                    pc += 1;
                }
                Instr::Acc(arg) => {
                    acc += arg;
                    pc += 1;
                }
                Instr::Jmp(arg) => {
                    pc_before_jmp.push(pc);
                    pc += arg;
                }
            }
        }
    }
    println!("terminated");
}
