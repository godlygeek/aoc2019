use std::fs;

fn read_program() -> Vec<usize> {
    let text = fs::read_to_string("input.txt").expect("Reading input failed");
    let mut program = Vec::new();
    for elem in text.split(",") {
        program.push(elem.trim().parse().unwrap());
    }
    program
}

fn handle_opcode(memory: &mut Vec<usize>, pc: usize) -> Option<usize> {
    match memory[pc] {
        1 => {
            let op1_addr = memory[pc + 1];
            let op2_addr = memory[pc + 2];
            let dst_addr = memory[pc + 3];
            memory[dst_addr] = memory[op1_addr] + memory[op2_addr];
            Some(pc + 4)
        }
        2 => {
            let op1_addr = memory[pc + 1];
            let op2_addr = memory[pc + 2];
            let dst_addr = memory[pc + 3];
            memory[dst_addr] = memory[op1_addr] * memory[op2_addr];
            Some(pc + 4)
        }
        99 => None,
        _ => {
            panic!("Invalid instruction {} at pc {}", memory[pc], pc);
        }
    }
}

fn part1(program: &Vec<usize>) {
    let mut memory = program.clone();
    memory[1] = 12;
    memory[2] = 2;
    let mut pc = 0;
    loop {
        match handle_opcode(&mut memory, pc) {
            Some(next_pc) => pc = next_pc,
            None => break
        }
    }
    println!("Part 1: {}", memory[0]);
}

fn part2(program: &Vec<usize>) {
    for i in 0..100 {
        for j in 0..100 {
            let mut memory = program.clone();
            memory[1] = i;
            memory[2] = j;
            let mut pc = 0;
            loop {
                match handle_opcode(&mut memory, pc) {
                    Some(next_pc) => pc = next_pc,
                    None => break
                }
            }
            if memory[0] == 19690720 {
                println!("Part 2: {}", 100 * i + j);
                return;
            }
        }
    }
    panic!("No valid solution found!");
}

fn main() {
    let program = read_program();
    part1(&program);
    part2(&program);
}
