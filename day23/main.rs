use std::fs::File;
use intcode::*;
use std::sync::{Arc, RwLock};
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Packet {
    x: i64,
    y: i64
}

struct Network {
    buffers: Vec<Arc<RwLock<VecDeque<Packet>>>>,
    nat_packet: Arc<RwLock<Option<Packet>>>
}

impl Network {
    fn new(size: usize) -> Network {
        Network { buffers: (0..size).map(|_| Arc::new(RwLock::new(VecDeque::new()))).collect(),
            nat_packet: Arc::new(RwLock::new(None)) }
    }

    fn output(&self) -> NICOutput {
        NICOutput { buffers: self.buffers.clone(), nat_packet: self.nat_packet.clone(), state: OutputState::Address }
    }

    fn input(&self, id: usize) -> NICInput {
        NICInput { id: id, id_assigned: false, last_packet: None, buffer: self.buffers[id].clone() }
    }
}

enum OutputState {
    Address,
    X(usize),
    Y(usize, i64)
}

struct NICOutput {
    buffers: Vec<Arc<RwLock<VecDeque<Packet>>>>,
    nat_packet: Arc<RwLock<Option<Packet>>>,
    state: OutputState
}

struct NICInput {
    id: usize,
    id_assigned: bool,
    last_packet: Option<Packet>,
    buffer: Arc<RwLock<VecDeque<Packet>>>
}

impl Output for NICOutput {
    fn output(&mut self, value: i64) {
        match self.state {
            OutputState::Address => self.state = OutputState::X(value as usize),
            OutputState::X(address) => self.state = OutputState::Y(address, value),
            OutputState::Y(address, x) => {
                self.state = OutputState::Address;
                if address == 255 {
                    *self.nat_packet.write().unwrap() = Some(Packet { x: x, y: value });
                } else {
                    self.buffers[address].write().unwrap().push_back(Packet { x: x, y: value });
                }
            }
        }
    }
}

impl Input for NICInput {
    fn get_next(&mut self) -> i64 {
        if self.id_assigned {
            if let Some(packet) = self.last_packet {
                let result = packet.y;
                self.last_packet = None;
                result
            } else if let Some(packet) = self.buffer.write().unwrap().pop_front() {
                self.last_packet = Some(packet);
                packet.x
            } else {
                -1
            }
        } else {
            self.id_assigned = true;
            self.id as i64
        }
    }
}


fn network_part1(memory: &Vec<i64>, size: usize) -> i64 {
    let network = Network::new(size);
    let mut computers = Vec::new();
    for i in 0..size {
        computers.push((Memory::new(&memory), 0, network.input(i), network.output()));
    }
    loop {
        for (memory, address, input, output) in &mut computers {
            execute_instruction(memory, input, output, address);
            if let Some(packet) = *network.nat_packet.read().unwrap() {
                return packet.y;
            }
        }
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);
    println!("Part 1: {}", network_part1(&tape, 50));
}