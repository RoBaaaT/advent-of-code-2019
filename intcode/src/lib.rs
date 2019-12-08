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

pub trait Input {
    fn get_next(&mut self) -> i64;
}

pub trait Output {
    fn output(&mut self, value: i64);
}

pub struct StdInput;

impl Input for StdInput {
    fn get_next(&mut self) -> i64 {
        // read input from stdin
        println!("Waiting for input:");
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
        match input_text.trim().parse::<i64>() {
            Ok(i) => i,
            Err(..) => panic!("invalid input: '{}'", input_text.trim())
        }
    }
}

pub struct StdOutput;

impl Output for StdOutput {
    fn output(&mut self, value: i64) {
        // write output to stdout
        println!("{}", value);
    }
}

pub struct VecInput {
    i: usize,
    values: Vec<i64>
}

impl VecInput {
    pub fn new(values: Vec<i64>) -> VecInput {
        VecInput { i: 0, values: values }
    }
}

impl Input for VecInput {
    fn get_next(&mut self) -> i64 {
        if self.i >= self.values.len() {
            panic!("not enough inputs provided to VecInput ({} requested, {} provided)", self.i + 1, self.values.len())
        }
        let result = self.values[self.i];
        self.i = self.i + 1;
        result
    }
}

pub struct VecOutput {
    values: Vec<i64>
}

impl VecOutput {
    pub fn new() -> VecOutput {
        VecOutput { values: Vec::new() }
    }

    pub fn values(&self) -> &Vec<i64> {
        &self.values
    }
}

impl Output for VecOutput {
    fn output(&mut self, value: i64) {
        self.values.push(value);
    }
}

fn get_param_value(memory: &[i64], address: usize, mode: i64) -> i64 {
    let param_value = memory[address];
    match mode {
        0 => memory[param_value as usize],
        1 => param_value,
        _ => panic!("invalid param mode: {})", mode)
    }
}

pub fn execute_intcode<I: Input, O: Output>(memory: &[i64], input: &mut I, output: &mut O) -> Vec<i64> {
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
            tape[param1_value as usize] = input.get_next();
            address += 2;
        } else if opcode == 4 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            output.output(param1);
            address += 2;
        } else if opcode == 5 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            if param1 != 0 {
                address = param2 as usize;
            } else {
                address += 3;
            }
        } else if opcode == 6 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            if param1 == 0 {
                address = param2 as usize;
            } else {
                address += 3;
            }
        } else if opcode == 7 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            let param3_value = tape[address + 3];
            assert!(mode3 == 0, "invalid param 3 mode (only 0 allowed): {}", instruction);
            if param1 < param2 {
                tape[param3_value as usize] = 1;
            } else {
                tape[param3_value as usize] = 0;
            }
            address += 4;
        } else if opcode == 8 {
            let param1 = get_param_value(&tape, address + 1, mode1);
            let param2 = get_param_value(&tape, address + 2, mode2);
            let param3_value = tape[address + 3];
            assert!(mode3 == 0, "invalid param 3 mode (only 0 allowed): {}", instruction);
            if param1 == param2 {
                tape[param3_value as usize] = 1;
            } else {
                tape[param3_value as usize] = 0;
            }
            address += 4;
        } else {
            panic!("invalid opcode: {} (full instruction: {}@{})", opcode, instruction, address);
        }
    }

    tape
}

#[cfg(test)]
mod tests {
    use crate::execute_intcode;

    #[test]
    #[should_panic]
    fn missing_halt() {
        let memory = vec![1, 0, 0, 3];
        execute_intcode(&memory);
    }

    #[test]
    fn add_positional() {
        let mut memory = vec![1, 0, 0, 3, 99];
        memory = execute_intcode(&memory);
        assert_eq!(memory[3], 2);
    }

    #[test]
    fn add_immediate() {
        let mut memory = vec![1101, 1, 1, 3, 99];
        memory = execute_intcode(&memory);
        assert_eq!(memory[3], 2);
    }

    #[test]
    fn multiply_positional() {
        let mut memory = vec![2, 0, 0, 3, 99];
        memory = execute_intcode(&memory);
        assert_eq!(memory[3], 4);
    }

    #[test]
    fn multiply_immediate() {
        let mut memory = vec![1102, 5, 2, 3, 99];
        memory = execute_intcode(&memory);
        assert_eq!(memory[3], 10);
    }
}