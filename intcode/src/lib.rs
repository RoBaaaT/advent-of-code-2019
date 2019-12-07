use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn load_tape(input: File) -> Vec<i64> {
    let reader = BufReader::new(&input);
    let mut tape = Vec::new();
    for opcode in reader.split(b',') {
        let op_str = String::from_utf8(opcode.unwrap()).unwrap();
        let op: i64 = op_str.trim_end().parse().unwrap();
        tape.push(op);
    }
    tape
}

pub fn execute_intcode(memory: &[i64]) -> Vec<i64> {
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