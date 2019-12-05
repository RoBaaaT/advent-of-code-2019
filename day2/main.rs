use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let input = BufReader::new(&input_file);
    let mut tape = Vec::new();
    for opcode in input.split(b',') {
        let op_str = String::from_utf8(opcode.unwrap()).unwrap();
        let op: i64 = op_str.trim_end().parse().unwrap();
        tape.push(op);
    }

    tape[1] = 12;
    tape[2] = 2;

    println!("Part 1: {}", compute_output(&tape)[0]);

    for noun in 0..99 {
        for verb in 0..99 {       
            tape[1] = noun;
            tape[2] = verb;
            if compute_output(&tape)[0] == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}

fn compute_output(memory: &[i64]) -> Vec<i64> {
    let mut tape = memory.to_vec();

    let mut address = 0;
    loop {
        let opcode = tape[address];
        if opcode == 99 {
            break;
        } else if opcode == 1 {
            let param1_address = tape[address + 1] as usize;
            let param2_address = tape[address + 2] as usize;
            let out_address = tape[address + 3] as usize;
            let param1 = tape[param1_address];
            let param2 = tape[param2_address];
            tape[out_address] = param1 + param2;
        } else if opcode == 2 {
            let param1_address = tape[address + 1] as usize;
            let param2_address = tape[address + 2] as usize;
            let out_address = tape[address + 3] as usize;
            let param1 = tape[param1_address];
            let param2 = tape[param2_address];
            tape[out_address] = param1 * param2;
        } else {
            panic!("invalid opcode");
        }
        address += 4;
    }

    tape
}