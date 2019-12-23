#![feature(drain_filter)]
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use std::slice::Iter;
use intcode::*;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Unknown,
    Start,
    Free,
    Wall,
    Oxygen
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn to_command(&self) -> i64 {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }

    fn offset(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y)
        }
    }

    fn all() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
        DIRECTIONS.iter()
    }
}

const MAP_SIZE: usize = 42;

struct Map {
    tiles: Vec<Tile>,
    x: usize,
    y: usize,
    next_direction: Direction
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut y = 0;
        for (i, tile) in self.tiles.iter().enumerate() {
            if self.x == i % MAP_SIZE && self.y == y {
                write!(f, "X")?;
            } else {
                match tile {
                    Tile::Unknown => write!(f, ".")?,
                    Tile::Start => write!(f, "S")?,
                    Tile::Free => write!(f, " ")?,
                    Tile::Wall => write!(f, "#")?,
                    Tile::Oxygen => write!(f, "0")?
                }
            }
            if i % MAP_SIZE == MAP_SIZE - 1 {
                write!(f, "\n")?;
                y += 1;
            }
        }
        Ok(())
    }
}

impl Map {
    fn new() -> Map {
        let mut result = Map { tiles: vec![Tile::Unknown; MAP_SIZE * MAP_SIZE], x: MAP_SIZE / 2, y: MAP_SIZE / 2,
            next_direction: Direction::North };
        result.set(result.x, result.y, Tile::Start);
        result
    }

    fn set(&mut self, x: usize, y: usize, value: Tile) {
        self.tiles[y * MAP_SIZE + x] = value;
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * MAP_SIZE + x]
    }

    fn get_direction_to_next_unknown(&self) -> Direction {
        let mut shortest_dir = Direction::North;
        let mut shortest_dist = std::usize::MAX;
        for dir in Direction::all() {
            if let Some(result) = self.get_shortest_path_to(self.x, self.y, Tile::Unknown, *dir) {
                if result < shortest_dist {
                    shortest_dist = result;
                    shortest_dir = *dir;
                }
            }
        }
        shortest_dir
    }

    fn unknown_reachable(&self) -> bool {
        let mut shortest_dist = std::usize::MAX;
        for dir in Direction::all() {
            if let Some(result) = self.get_shortest_path_to(self.x, self.y, Tile::Unknown, *dir) {
                if result < shortest_dist {
                    shortest_dist = result;
                }
            }
        }
        shortest_dist != std::usize::MAX
    }

    fn get_shortest_distance_to(&self, start_x: usize, start_y: usize, target: Tile) -> usize {
        let mut shortest_dist = std::usize::MAX;
        for dir in Direction::all() {
            if let Some(result) = self.get_shortest_path_to(start_x, start_y, target, *dir) {
                if result < shortest_dist {
                    shortest_dist = result;
                }
            }
        }
        shortest_dist
    }

    fn adjacent_position(a_x: usize, a_y: usize, b_x: usize, b_y: usize) -> bool {
        (a_x == b_x && (a_y + 1 == b_y || a_y == b_y + 1)) || (a_y == b_y && (a_x + 1 == b_x || a_x == b_x + 1))
    }

    fn get_shortest_path_to(&self, start_x: usize, start_y: usize, target: Tile, dir: Direction) -> Option<usize> {
        let mut positions: Vec<(usize, usize)> = (0..self.tiles.len()).map(|i| (i % MAP_SIZE, i / MAP_SIZE)).collect();
        positions.remove(start_y * MAP_SIZE + start_x);
        let (dir_x, dir_y) = dir.offset(start_x, start_y);
        let mut to_visit = vec![(dir_x, dir_y, 1)];
        positions.retain(|(x, y)| *x != dir_x || *y != dir_y);
        while to_visit.len() > 0 {
            to_visit.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());
            let mut new_to_visit = Vec::new();
            for (x, y, len) in &to_visit {
                let tile = self.get(*x, *y);
                if tile == target {
                    return Some(*len);
                }
                match tile {
                    Tile::Free | Tile::Oxygen | Tile::Unknown | Tile::Start => {
                        let mut new_visits: Vec<(usize, usize, usize)> = positions
                            .drain_filter(|(pos_x, pos_y)| Map::adjacent_position(*x, *y, *pos_x, *pos_y))
                            .map(|(pos_x, pos_y)| (pos_x, pos_y, len + 1)).collect();
                        new_to_visit.append(&mut new_visits);
                    },
                    Tile::Wall => {}
                }
            }
            to_visit = new_to_visit;
        }
        None
    }

    fn get_flooding_time(&self) -> usize {
        let mut positions: Vec<(usize, usize)> = self.tiles.iter().enumerate()
            .filter_map(|(i, tile)| if *tile == Tile::Free || *tile == Tile::Start { Some((i % MAP_SIZE, i / MAP_SIZE)) } else { None }).collect();
        let mut flooded_positions = vec![self.tiles.iter().enumerate()
            .find_map(|(i, tile)| if *tile == Tile::Oxygen { Some((i % MAP_SIZE, i / MAP_SIZE)) } else { None }).unwrap()];
        let mut time = 0;
        while positions.len() > 0 {
            let mut new_flooded_positions = Vec::new();
            for (x, y) in &flooded_positions {
                let mut newly_flooded: Vec<(usize, usize)> = positions
                    .drain_filter(|(pos_x, pos_y)| Map::adjacent_position(*x, *y, *pos_x, *pos_y)).collect();
                new_flooded_positions.append(&mut newly_flooded);
            }
            flooded_positions = new_flooded_positions;
            time += 1;
        }
        time
    }
}

struct MappingInput {
    map: Rc<RefCell<Map>>
}

struct MappingOutput {
    map: Rc<RefCell<Map>>
}

impl Input for MappingInput {
    fn get_next(&mut self) -> i64 {
        self.map.borrow().next_direction.to_command()
    }
}

impl Output for MappingOutput {
    fn output(&mut self, value: i64) {
        let mut map = self.map.borrow_mut();
        match value {
            0 => {
                let (x, y) = map.next_direction.offset(map.x, map.y);
                map.set(x, y, Tile::Wall);
            },
            1 | 2 => {
                let (x, y) = map.next_direction.offset(map.x, map.y);
                map.x = x;
                map.y = y;
                map.set(x, y, if value == 1 { Tile::Free } else { Tile::Oxygen });
                if !map.unknown_reachable() {            
                    println!("{}", map);
                    println!("Part 1: {}", map.get_shortest_distance_to(MAP_SIZE / 2, MAP_SIZE / 2, Tile::Oxygen));
                    println!("Part 2: {}", map.get_flooding_time());
                    std::process::exit(0);
                }
            }
            _ => panic!("Unexpected output value")
        }
        map.next_direction = map.get_direction_to_next_unknown();
        //println!("{}", map);
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let map = Rc::new(RefCell::new(Map::new()));
    let mut input = MappingInput { map: Rc::clone(&map) };
    let mut output = MappingOutput { map: Rc::clone(&map) };

    execute_intcode(&tape, &mut input, &mut output);
}