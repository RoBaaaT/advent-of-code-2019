use std::fs::File;
use std::ops::Range;
use intcode::*;

fn amp(memory: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let mut memory = memory.clone();
    let mut prev_output = 0;
    for phase in phase_settings {
        let mut output = VecOutput::new();
        memory = execute_intcode(&memory, &mut VecInput::new(vec![phase.clone(), prev_output]), &mut output);
        prev_output = output.values()[0];
    }
    prev_output
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
}