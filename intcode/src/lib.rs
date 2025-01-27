use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::ops::Index;
use std::ops::IndexMut;

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

pub struct StdASCIIInput {
    last_input: String,
    read_offset: usize
}

impl StdASCIIInput {
    pub fn new() -> StdASCIIInput {
        StdASCIIInput { last_input: String::new(), read_offset: 0 }
    }
}

impl Input for StdASCIIInput {
    fn get_next(&mut self) -> i64 {
        if self.read_offset >= self.last_input.chars().count() {
            self.last_input.clear();
            self.read_offset = 0;
            io::stdin().read_line(&mut self.last_input).expect("failed to read from stdin");
            self.last_input = self.last_input.trim().to_string();
            self.last_input.push('\n');
        }
        let result = self.last_input.chars().nth(self.read_offset);
        self.read_offset += 1;
        result.unwrap() as i64
    }
}

pub struct StdOutput;

impl Output for StdOutput {
    fn output(&mut self, value: i64) {
        // write output to stdout
        println!("{}", value);
    }
}

pub struct StdASCIIOutput {
    last_output: i64
}

impl StdASCIIOutput {
    pub fn new() -> StdASCIIOutput {
        StdASCIIOutput { last_output: 0 }
    }

    pub fn last_output(&self) -> i64 {
        self.last_output
    }
}

impl Output for StdASCIIOutput {
    fn output(&mut self, value: i64) {
        // write output to stdout
        self.last_output = value;
        print!("{}", value as u8 as char);
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

pub struct StringInput<'a> {
    chars: std::str::Chars<'a>
}

impl<'a> StringInput<'a> {
    pub fn new(chars: std::str::Chars<'a>) -> StringInput<'a> {
        StringInput { chars: chars }
    }
}

impl<'a> Input for StringInput<'a> {
    fn get_next(&mut self) -> i64 {
        self.chars.next().unwrap() as i64
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

fn get_param_value(memory: &Memory, address: usize, mode: i64) -> i64 {
    let param_value = memory[address];
    match mode {
        0 => memory[param_value as usize],
        1 => param_value,
        2 => memory[(memory.relative_base + param_value) as usize],
        _ => panic!("invalid read param mode: {})", mode)
    }
}

fn set_memory_value(memory: &mut Memory, address: i64, mode: i64, value: i64) {
    let relative_base = memory.relative_base;
    match mode {
        0 => memory[address as usize] = value,
        2 => memory[(relative_base + address) as usize] = value,
        _ => panic!("invalid write param mode: {})", mode)
    };
}

pub struct Memory {
    memory: Vec<i64>,
    relative_base: i64
}

impl Memory {
    pub fn new(memory: &[i64]) -> Memory {
        Memory { memory: memory.to_vec(), relative_base: 0 }
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.memory.len() {
            &self.memory[index]
        } else {
            &0
        }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.memory.len() {
            let required_mem = index - self.memory.len() + 1;
            for _ in 0..required_mem {
                self.memory.push(0);
            }
        }
        &mut self.memory[index]
    }
}

pub fn execute_instruction<I: Input, O: Output>(memory: &mut Memory, input: &mut I, output: &mut O, address: &mut usize) -> bool {
    let instruction = memory[*address];
    let opcode = instruction % 100;
    let mode1 = (instruction / 100) % 10;
    let mode2 = (instruction / 1000) % 10;
    let mode3 = (instruction / 10000) % 10;
    if opcode == 99 {
        return false;
    } else if opcode == 1 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        let out_address = memory[*address + 3];
        set_memory_value(memory, out_address, mode3, param1 + param2);
        *address += 4;
    } else if opcode == 2 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        let out_address = memory[*address + 3];
        set_memory_value(memory, out_address, mode3, param1 * param2);
        *address += 4;
    } else if opcode == 3 {
        let out_address = memory[*address + 1];
        set_memory_value(memory, out_address, mode1, input.get_next());
        *address += 2;
    } else if opcode == 4 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        output.output(param1);
        *address += 2;
    } else if opcode == 5 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        if param1 != 0 {
            *address = param2 as usize;
        } else {
            *address += 3;
        }
    } else if opcode == 6 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        if param1 == 0 {
            *address = param2 as usize;
        } else {
            *address += 3;
        }
    } else if opcode == 7 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        let out_address = memory[*address + 3];
        if param1 < param2 {
            set_memory_value(memory, out_address, mode3, 1);
        } else {
            set_memory_value(memory, out_address, mode3, 0);
        }
        *address += 4;
    } else if opcode == 8 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        let param2 = get_param_value(memory, *address + 2, mode2);
        let out_address = memory[*address + 3];
        if param1 == param2 {
            set_memory_value(memory, out_address, mode3, 1);
        } else {
            set_memory_value(memory, out_address, mode3, 0);
        }
        *address += 4;
    } else if opcode == 9 {
        let param1 = get_param_value(memory, *address + 1, mode1);
        memory.relative_base += param1;
        *address += 2;
    } else {
        panic!("invalid opcode: {} (full instruction: {}@{})", opcode, instruction, *address);
    }
    true
}

pub fn execute_intcode<I: Input, O: Output>(memory: &[i64], input: &mut I, output: &mut O) -> Vec<i64> {
    let mut memory = Memory::new(memory);

    let mut address = 0;
    while execute_instruction(&mut memory, input, output, &mut address) {}

    memory.memory
}

#[cfg(test)]
mod tests {
    use crate::execute_intcode;
    use crate::StdInput;
    use crate::StdOutput;
    use crate::VecOutput;

    #[test]
    #[should_panic]
    fn missing_halt() {
        let memory = vec![1, 0, 0, 3];
        execute_intcode(&memory, &mut StdInput, &mut StdOutput);
    }

    #[test]
    fn add_positional() {
        let mut memory = vec![1, 0, 0, 3, 99];
        memory = execute_intcode(&memory, &mut StdInput, &mut StdOutput);
        assert_eq!(memory[3], 2);
    }

    #[test]
    fn add_immediate() {
        let mut memory = vec![1101, 1, 1, 3, 99];
        memory = execute_intcode(&memory, &mut StdInput, &mut StdOutput);
        assert_eq!(memory[3], 2);
    }

    #[test]
    fn multiply_positional() {
        let mut memory = vec![2, 0, 0, 3, 99];
        memory = execute_intcode(&memory, &mut StdInput, &mut StdOutput);
        assert_eq!(memory[3], 4);
    }

    #[test]
    fn multiply_immediate() {
        let mut memory = vec![1102, 5, 2, 3, 99];
        memory = execute_intcode(&memory, &mut StdInput, &mut StdOutput);
        assert_eq!(memory[3], 10);
    }

    #[test]
    fn self_copy() {
        let memory = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut out = VecOutput::new();
        execute_intcode(&memory, &mut StdInput, &mut out);
        assert_eq!(out.values, memory);
    }

    #[test]
    fn sixteen_digit_number() {
        let memory = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut out = VecOutput::new();
        execute_intcode(&memory, &mut StdInput, &mut out);
        assert_eq!((out.values[0].abs() as f64).log10() as i64, 15);
    }

    #[test]
    fn large_number() {
        let memory = vec![104,1125899906842624,99];
        let mut out = VecOutput::new();
        execute_intcode(&memory, &mut StdInput, &mut out);
        assert_eq!(out.values[0], 1125899906842624);
    }

    #[test]
    fn relative_base() {
        let memory = vec![109,15,109,19,204,-34,99];
        let mut out = VecOutput::new();
        execute_intcode(&memory, &mut StdInput, &mut out);
        assert_eq!(out.values[0], 109);
    }
}