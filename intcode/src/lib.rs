use std::io;
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

fn get_param_value(memory: &[i64], address: usize, mode: i64) -> i64 {
    let param_value = memory[address];
    match mode {
        0 => memory[param_value as usize],
        1 => param_value,
        _ => panic!("invalid param mode: {})", mode)
    }
}

pub fn execute_intcode(memory: &[i64]) -> Vec<i64> {
    let mut tape = memory.to_vec();

    let mut address = 0;
    loop {
        let instruction = tape[address];
        let opcode = instruction % 100;
        let mode1 = (instruction / 100) % 10;
        let mode2 = (instruction / 1000) % 10;
        let mode3 = (instruction / 10000) % 10;
        if opcode == 99 {
            break;
        } else if opcode == 1 {
            let param3_value = tape[address + 3];
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            assert!(mode3 == 0, "invalid param 3 mode (only 0 allowed): {}", instruction);
            tape[param3_value as usize] = param1 + param2;
            address += 4;
        } else if opcode == 2 {
            let param3_value = tape[address + 3];
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            assert!(mode3 == 0, "invalid param 3 mode (only 0 allowed): {}", instruction);
            tape[param3_value as usize] = param1 * param2;
            address += 4;
        } else if opcode == 3 {
            assert!(mode1 == 0, "invalid param 1 mode (only 0 allowed): {}", instruction);
            let param1_value = tape[address + 1];
            // read input from stdin
            println!("Waiting for input:");
            let mut input_text = String::new();
            io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
            match input_text.trim().parse::<i64>() {
                Ok(i) => tape[param1_value as usize] = i,
                Err(..) => panic!("invalid input: '{}'", input_text.trim())
            };
            address += 2;
        } else if opcode == 4 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            // write output to stdout
            println!("{}", param1);
            address += 2;
        } else {
            panic!("invalid opcode: {} (full instruction: {}@{})", opcode, instruction, address);
        }
    }

    tape
}