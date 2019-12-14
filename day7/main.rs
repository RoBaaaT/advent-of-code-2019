use std::fs::File;
use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;
use std::thread;
use intcode::*;

struct IOLink {
    buffer: Arc<RwLock<VecDeque<i64>>>
}

impl IOLink {
    fn new() -> IOLink {
        IOLink { buffer: Arc::new(RwLock::new(VecDeque::new())) }
    }

    fn from_slice(values: &[i64]) -> IOLink {
        let result = Self::new();
        for value in values { result.buffer.write().unwrap().push_back(*value); }
        result
    }

    fn output(&self) -> IOLinkOutput {
        IOLinkOutput { buffer: self.buffer.clone() }
    }

    fn input(&self) -> IOLinkInput {
        IOLinkInput { buffer: self.buffer.clone() }
    }
}

struct IOLinkOutput {
    buffer: Arc<RwLock<VecDeque<i64>>>
}

struct IOLinkInput {
    buffer: Arc<RwLock<VecDeque<i64>>>
}

impl Output for IOLinkOutput {
    fn output(&mut self, value: i64) {
        self.buffer.write().unwrap().push_back(value);
    }
}

impl Input for IOLinkInput {
    fn get_next(&mut self) -> i64 {
        loop {
            if let Some(value) = self.buffer.write().unwrap().pop_front() {
                break value;
            }
        }
    }
}


fn amp(memory: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let mut links = Vec::new();
    for (i, phase) in phase_settings.iter().enumerate() {
        links.push(IOLink::from_slice(&[*phase]));
        if i == 0 {
            links[0].buffer.write().unwrap().push_back(0);
        }
    }
    let mut threads = Vec::new();
    for i in 0..links.len() {
        let next_link = if i == links.len() - 1 { 0 } else { i + 1 };
        let thread_memory = memory.clone();
        let mut input = links[i].input();
        let mut output = links[next_link].output();
        threads.push(thread::spawn(move || {
            execute_intcode(&thread_memory, &mut input, &mut output);
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    let result: i64 = *links[0].buffer.read().unwrap().back().unwrap();
    result
}

fn find_highest_signal(memory: &Vec<i64>, phase_range: Range<i64>) -> i64 {
    let mut highest_signal = 0;
    let mut phase_settings = vec![0, 0, 0, 0, 0];
    for phase_a in phase_range.clone() {
        phase_settings[0] = phase_a;
        for phase_b in phase_range.clone() {
            if phase_b == phase_a {
                continue;
            }
            phase_settings[1] = phase_b;
            for phase_c in phase_range.clone() {
                if phase_c == phase_a || phase_c == phase_b {
                    continue;
                }
                phase_settings[2] = phase_c;
                for phase_d in phase_range.clone() {
                    if phase_d == phase_a || phase_d == phase_b || phase_d == phase_c {
                        continue;
                    }
                    phase_settings[3] = phase_d;
                    for phase_e in phase_range.clone() {
                        if phase_e == phase_a || phase_e == phase_b || phase_e == phase_c || phase_e == phase_d {
                            continue;
                        }
                        phase_settings[4] = phase_e;
                        let out = amp(&memory, &phase_settings);
                        if out > highest_signal {
                            highest_signal = out;
                        }
                    }
                }
            }
        }
    }
    highest_signal
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let memory = load_tape(input_file);
    println!("Part 1: {}", find_highest_signal(&memory, 0..5));
    println!("Part 2: {}", find_highest_signal(&memory, 5..10));
}