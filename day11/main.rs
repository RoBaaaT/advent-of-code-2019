use std::fs::File;
use intcode::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::result::Result;

struct RobotInput {
    map: Rc<RefCell<Map>>
}

struct RobotOutput {
    map: Rc<RefCell<Map>>,
    color_output: bool
}

const MAP_SIZE: usize = 100;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Map {
    values: [bool; MAP_SIZE * MAP_SIZE],
    painted: [bool; MAP_SIZE * MAP_SIZE],
    robot_x: usize,
    robot_y: usize,
    robot_direction: Direction
}

impl Map {
    fn new() -> Map {
        Map { values: [false; MAP_SIZE * MAP_SIZE], painted: [false; MAP_SIZE * MAP_SIZE],
            robot_x: MAP_SIZE / 2, robot_y: MAP_SIZE / 2, robot_direction: Direction::Up }
    }

    fn current_cell(&self) -> bool {
        self.values[self.robot_y * MAP_SIZE + self.robot_x]
    }

    fn paint_cell(&mut self, value: bool) {
        self.values[self.robot_y * MAP_SIZE + self.robot_x] = value;
        self.painted[self.robot_y * MAP_SIZE + self.robot_x] = true;
    }

    fn move_forward(&mut self) {
        match &self.robot_direction {
            Direction::Up => self.robot_y -= 1,
            Direction::Down => self.robot_y += 1,
            Direction::Left => self.robot_x -= 1,
            Direction::Right => self.robot_x += 1
        }
    }

    fn painted_count(&self) -> usize {
        self.painted.iter().filter(|&x| *x).count()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, cell) in self.values.iter().enumerate() {
            write!(f, "{}", if *cell { "#" } else { " " })?;
            if i % MAP_SIZE == MAP_SIZE - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Input for RobotInput {
    fn get_next(&mut self) -> i64 {
        if self.map.borrow().current_cell() { 1 } else { 0 }
    }
}

impl Output for RobotOutput {
    fn output(&mut self, value: i64) {
        if self.color_output {
            // receive color output
            match value {
                0 => self.map.borrow_mut().paint_cell(false),
                1 => self.map.borrow_mut().paint_cell(true),
                _ => panic!("received invalid color")
            }
        } else {
            // receive direction output
            let current_direction = self.map.borrow().robot_direction;
            match value {
                0 => {
                    self.map.borrow_mut().robot_direction = match current_direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up
                    };
                },
                1 => {
                    self.map.borrow_mut().robot_direction = match current_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down
                    };
                },
                _ => panic!("received invalid direction")
            }
            self.map.borrow_mut().move_forward();
        }
        self.color_output = !self.color_output;
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let map = Rc::new(RefCell::new(Map::new()));
    let mut input = RobotInput { map: Rc::clone(&map) };
    let mut output = RobotOutput { map: Rc::clone(&map), color_output: true };
    execute_intcode(&tape, &mut input, &mut output);
    println!("Part 1:");
    println!("{}", map.borrow());
    println!("{}", map.borrow().painted_count());

    let map = Rc::new(RefCell::new(Map::new()));
    map.borrow_mut().paint_cell(true);
    let mut input = RobotInput { map: Rc::clone(&map) };
    let mut output = RobotOutput { map: Rc::clone(&map), color_output: true };
    execute_intcode(&tape, &mut input, &mut output);
    println!("Part 2:");
    println!("{}", map.borrow());
}