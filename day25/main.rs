use std::cell::RefCell;
use std::fs::File;
use std::fmt;
use std::rc::Rc;
use regex::Regex;
use lazy_static::lazy_static;
use intcode::*;
use RoomId::{Unknown, Id};
use Command::{Move, Take, Drop};

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn try_parse(s: &str) -> Result<Direction, String> {
        match s {
            "north" => Ok(Direction::North),
            "east" => Ok(Direction::East),
            "south" => Ok(Direction::South),
            "west" => Ok(Direction::West),
            _ => Err(String::from("invalid direction"))
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East
        }
    }

    fn from_usize(u: usize) -> Result<Direction, String> {
        match u {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err(String::from("invalid id"))
        }
    }
}

impl std::string::ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::North => String::from("north"),
            Direction::East => String::from("east"),
            Direction::South => String::from("south"),
            Direction::West => String::from("west")
        }
    }
}

#[derive(Clone, Debug)]
enum Command {
    Move(Direction),
    Take(String),
    Drop(String)
}

impl Into<String> for Command {
    fn into(self) -> String {
        match self {
            Command::Move(dir) => dir.to_string(),
            Command::Take(item) => format!("take {}", item),
            Command::Drop(item) => format!("drop {}", item)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum RoomId {
    Unknown,
    Id(usize)
}

#[derive(Clone, Debug)]
struct Room {
    name: String,
    description: String,
    doors: [Option<RoomId>; 4],
    items: Vec<String>
}

impl Room {
    fn empty() -> Room {
        Room { name: String::new(), description: String::new(), doors: [None; 4], items: Vec::new() }
    }
}

struct Map {
    rooms: Vec<Room>,
    current_room: Option<usize>,
    last_command: Option<Command>,
    items: Vec<String>,
    state: SolverState
}

#[derive(Copy, Clone)]
enum SecSolverState {
    DropTake,
    Try
}

enum SolverState {
    Exploration,
    Navigation,
    Security(usize, SecSolverState)
}

impl Map {
    fn new() -> Map {
        Map { rooms: Vec::new(), current_room: None, last_command: None, items: Vec::new(),
            state: SolverState::Exploration }
    }

    fn direction_to(&self, room_id: usize) -> Direction {
        let start_id = if let Some(id) = self.current_room { id } else { panic!("No current room!") };
        let mut unexplored_ids: Vec<usize> = (0..self.rooms.len()).collect();
        unexplored_ids.remove(room_id);
        let mut explore_ids = vec![room_id];
        while unexplored_ids.len() > 0 {
            let mut new_explore_ids = Vec::new();
            for i in 0..explore_ids.len() {
                for (dir, door) in self.rooms[explore_ids[i]].doors.iter().enumerate() {
                    if let Some(Id(next_id)) = door {
                        if let Some(j) = unexplored_ids.iter().position(|&id| id == *next_id) {
                            unexplored_ids.remove(j);
                            new_explore_ids.push(*next_id);
                            if *next_id == start_id {
                                return Direction::from_usize(dir).unwrap().opposite();
                            }
                        }
                    }
                }
            }
            explore_ids = new_explore_ids;
        }
        panic!("No path found!");
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.current_room {
            None => {},
            Some(id) => writeln!(f, "Currently in {}", self.rooms[id].name)?
        }
        match &self.last_command {
            None => {},
            Some(command) => writeln!(f, "Last command: {:?}", command)?
        }
        for (id, room) in self.rooms.iter().enumerate() {
            writeln!(f, "Room {}: {} ({})", id, room.name, room.description)?;
            for (dir, door) in room.doors.iter().enumerate() {
                if let Some(room_id) = door {
                    let room_name = if let Id(room_id) = room_id { &self.rooms[*room_id].name } else { "Unknown room" };
                    writeln!(f, "\t{:?}: {}", Direction::from_usize(dir).unwrap(), room_name)?;
                }
            }
        }
        writeln!(f, "Items: {:?}", self.items)?;
        Ok(())
    }
}

struct Day25Input {
    map: Rc<RefCell<Map>>,
    last_input: String,
    read_offset: usize
}

enum OutputState {
    RoomName,
    RoomDescription,
    DoorsHereLead,
    DirectionList,
    ItemList,
    Done
}

struct Day25Output {
    map: Rc<RefCell<Map>>,
    buf: String,
    current_room: Room,
    state: OutputState,
    was_ejected: bool
}

impl Day25Input {
    fn new(map: &Rc<RefCell<Map>>) -> Day25Input {
        Day25Input { map: Rc::clone(map), last_input: String::new(), read_offset: 0 }
    }

    fn get_next_input(map: &mut Map) -> Command {
        //println!("{:?}", map);

        let current_room_id = map.current_room.unwrap();
        match map.state {
            SolverState::Exploration => { // explore the map & collect items
                if let Some(item) = map.rooms[current_room_id].items.pop() {
                    if item != "giant electromagnet" && item != "infinite loop" && item != "photons" && item != "molten lava"
                            && item != "escape pod" {
                        map.items.push(item.clone());
                        return Take(item);
                    }
                }
                let direction = map.rooms[current_room_id].doors.iter().enumerate()
                    .find_map(|(dir, door)| if let Some(Unknown) = door { Some(Direction::from_usize(dir).unwrap()) } else { None });
                if map.rooms[current_room_id].name != "Security Checkpoint" {
                    if let Some(dir) = direction {
                        return Move(dir);
                    }
                }
                let target_room = map.rooms.iter().enumerate().find_map(|(id, room)| {
                    if room.name != "Security Checkpoint" &&
                            room.doors.iter().any(|door| if let Some(Unknown) = door { true } else { false }) {
                        Some(id)
                    } else {
                        None
                    }
                });
                let dir = if let Some(id) = target_room {
                    map.direction_to(id)
                } else {
                    map.state = SolverState::Navigation;
                    return Day25Input::get_next_input(map);
                };
                return Move(dir);
            },
            SolverState::Navigation => { // navigate to the security checkpoint
                let security_room_id = map.rooms.iter().enumerate().find_map(|(id, room)| if room.name == "Security Checkpoint" { Some(id) } else { None }).unwrap();
                if current_room_id == security_room_id {
                    map.state = SolverState::Security(0, SecSolverState::Try);
                    return Day25Input::get_next_input(map);
                } else {
                    return Move(map.direction_to(security_room_id));
                }
            },
            SolverState::Security(i, state) => { // solve the puzzle to get through the security checkpoint
                return match state {
                    SecSolverState::DropTake => {
                        map.state = SolverState::Security(i, SecSolverState::Try);
                        let gray_code = !(i ^ (i >> 1));
                        let prev_gray_code = !((i - 1) ^ ((i - 1) >> 1));
                        let mut diff = gray_code ^ prev_gray_code;
                        let mut bit = 0;
                        while diff > 0 {
                            bit += 1;
                            diff >>= 1;
                        }
                        bit -= 1;
                        if gray_code & (1 << bit) > 0 {
                            Take(map.items[bit].clone())
                        } else {
                            Drop(map.items[bit].clone())
                        }
                    },
                    SecSolverState::Try => {
                        map.state = SolverState::Security(i + 1, SecSolverState::DropTake);
                        let dir = map.rooms[current_room_id].doors.iter().enumerate()
                            .find_map(|(dir, door)| if let Some(Unknown) = door { Some(Direction::from_usize(dir).unwrap()) } else { None }).unwrap();
                        Move(dir)
                    }
                };
            }
        }
    }
}

impl Day25Output {
    fn new(map: &Rc<RefCell<Map>>) -> Day25Output {
        Day25Output { map: Rc::clone(map), buf: String::new(), current_room: Room::empty(),
            state: OutputState::RoomName, was_ejected: false }
    }

    fn finish_current_room(&mut self) {
        let mut map = self.map.borrow_mut();
        if !map.rooms.iter().any(|r| r.name == self.current_room.name) {
            map.rooms.push(self.current_room.clone());
        }
        let current_id = map.rooms.iter().enumerate()
            .find_map(|(id, r)| if r.name == self.current_room.name { Some(id) } else { None }).unwrap();
        if let Some(Move(dir)) = map.last_command.clone() {
            if let Some(prev_id) = map.current_room {
                map.rooms[prev_id].doors[dir as usize] = Some(Id(current_id));
                map.rooms[current_id].doors[dir.opposite() as usize] = Some(Id(prev_id));
            }
        }
        map.current_room = Some(current_id);
        self.current_room = Room::empty();
    }

    fn handle_output(&mut self) {
        lazy_static! {
            static ref NAME_RE: Regex = Regex::new(r"== (?P<name>.*) ==").unwrap();
            static ref LIST_RE: Regex = Regex::new(r"- (?P<item>.*)").unwrap();
        }
        //println!("{}", self.buf);

        let last_command = self.map.borrow().last_command.clone();
        if &self.buf == "Command?" {
            match last_command {
                Some(Move(_)) | None => {
                    if !self.was_ejected {
                        self.finish_current_room();
                    } else {
                        self.was_ejected = false;
                    }
                },
                Some(Take(_)) => {},
                Some(Drop(_)) => {}
            }
            self.state = OutputState::RoomName;
        } else {
            match last_command {
                Some(Move(_)) | None => {
                    match self.state {
                        OutputState::RoomName => {
                            if self.buf == "Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly." {
                                self.state = OutputState::Done;
                            } else {
                                self.current_room.name = NAME_RE.captures(&self.buf).unwrap()["name"].to_string();
                                self.state = OutputState::RoomDescription;
                            }
                        },
                        OutputState::RoomDescription => {
                            self.current_room.description = self.buf.clone();
                            self.state = OutputState::DoorsHereLead;
                        },
                        OutputState::DoorsHereLead => {
                            assert_eq!(&self.buf, "Doors here lead:");
                            self.state = OutputState::DirectionList;
                        },
                        OutputState::DirectionList => {
                            if self.current_room.name == "Pressure-Sensitive Floor"
                                    && self.buf.starts_with("A loud, robotic voice says") {
                                self.was_ejected = true;
                                self.state = OutputState::RoomName;
                            } else if let Some(captures) = LIST_RE.captures(&self.buf) {
                                if let Some(dir) = captures.name("item") {
                                    if let Ok(dir) = Direction::try_parse(dir.as_str()) {
                                        self.current_room.doors[dir as usize] = Some(Unknown);
                                    } else { panic!("Invalid direction!"); }
                                } else { panic!("Invalid list item!"); }
                            } else if &self.buf == "Items here:" {
                                self.state = OutputState::ItemList;
                            } else { panic!("Unexpected output: {}", self.buf); }
                        },
                        OutputState::ItemList => {
                            if let Some(captures) = LIST_RE.captures(&self.buf) {
                                if let Some(item) = captures.name("item") {
                                    self.current_room.items.push(item.as_str().to_string());
                                } else { panic!("Invalid list item!"); }
                            } else { panic!("Unexpected output: {}", self.buf); }
                        },
                        OutputState::Done => {
                            println!("Part 1: {}", self.buf);
                        }
                    }
                },
                Some(Take(item)) => {
                    assert!(self.buf.starts_with("You take the "), "Unexpected output after taking {}: {}", item, self.buf);
                },
                Some(Drop(item)) => {
                    assert!(self.buf.starts_with("You drop the "), "Unexpected output after dropping {}: {}", item, self.buf);
                }
            }
        }
    }
}

impl Input for Day25Input {
    fn get_next(&mut self) -> i64 {
        if self.read_offset >= self.last_input.chars().count() {
            let command = Day25Input::get_next_input(&mut self.map.borrow_mut());
            self.map.borrow_mut().last_command = Some(command.clone());
            self.last_input = command.into();
            self.last_input.push('\n');
            self.read_offset = 0;
        }
        let result = self.last_input.chars().nth(self.read_offset);
        self.read_offset += 1;
        result.unwrap() as i64
    }
}

impl Output for Day25Output {
    fn output(&mut self, value: i64) {
        let c = value as u8 as char;
        if c != '\n' {
            self.buf.push(c);
        } else {
            if !self.buf.trim().is_empty() { self.handle_output(); }
            self.buf.clear();
        }
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let tape = load_tape(input_file);

    let map = Rc::new(RefCell::new(Map::new()));
    let mut input = Day25Input::new(&map);
    let mut output = Day25Output::new(&map);
    execute_intcode(&tape, &mut input, &mut output);
}