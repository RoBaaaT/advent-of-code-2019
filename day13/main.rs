use std::fs::File;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use intcode::*;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

#[derive(Copy, Clone)]
enum InputState {
    X,
    Y,
    Tile
}

const SCREEN_WIDTH: usize = 46;
const SCREEN_HEIGHT: usize = 26;

struct ArcadeScreen {
    tiles: Vec<Tile>,
    score: i64
}

impl ArcadeScreen {
    fn new() -> ArcadeScreen {
        ArcadeScreen { tiles: vec![Tile::Empty; SCREEN_WIDTH * SCREEN_HEIGHT], score: 0 }
    }
}

impl fmt::Display for ArcadeScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, tile) in self.tiles.iter().enumerate() {
            match tile {
                Tile::Empty => write!(f, " ")?,
                Tile::Wall => write!(f, "■")?,
                Tile::Block => write!(f, "#")?,
                Tile::Paddle => write!(f, "_")?,
                Tile::Ball => write!(f, "O")?
            }
            if i % SCREEN_WIDTH == SCREEN_WIDTH - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "Score: {}", self.score)?;
        Ok(())
    }
}

struct ArcadeScreenInput {
    screen: Rc<RefCell<ArcadeScreen>>,
    next_tile_x: i64,
    next_tile_y: i64,
    input_state: InputState
}

impl ArcadeScreenInput {
    fn new(screen: Rc<RefCell<ArcadeScreen>>) -> ArcadeScreenInput {
        ArcadeScreenInput { screen: screen, next_tile_x: 0, next_tile_y: 0, input_state: InputState::X }
    }
}

impl Output for ArcadeScreenInput {
    fn output(&mut self, value: i64) {
        match self.input_state {
            InputState::X => {
                self.next_tile_x = value;
                self.input_state = InputState::Y;
            },
            InputState::Y => {
                self.next_tile_y = value;
                self.input_state = InputState::Tile;
            },
            InputState::Tile => {
                if self.next_tile_x == -1 && self.next_tile_y == 0 {
                    self.screen.borrow_mut().score = value;
                } else {
                    assert!((self.next_tile_x as usize) < SCREEN_WIDTH);
                    assert!((self.next_tile_y as usize) < SCREEN_HEIGHT);
                    self.screen.borrow_mut().tiles[self.next_tile_y as usize * SCREEN_WIDTH + self.next_tile_x as usize] = match value {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        _ => panic!("invalid tile")
                    };
                }
                self.input_state = InputState::X;
            }
        }
    }
}

struct ArcadeInput {
    screen: Rc<RefCell<ArcadeScreen>>
}

impl ArcadeInput {
    fn new(screen: Rc<RefCell<ArcadeScreen>>) -> ArcadeInput {
        ArcadeInput { screen: screen }
    }
}

impl Input for ArcadeInput {
    fn get_next(&mut self) -> i64 {
        let screen = self.screen.borrow();
        let position = screen.tiles.iter().position(|&tile| tile == Tile::Ball).unwrap();
        let paddle_position = screen.tiles.iter().position(|&tile| tile == Tile::Paddle).unwrap();
        let mut command = 0;
        let x = (position % SCREEN_WIDTH) as i64;
        let paddle_x = (paddle_position % SCREEN_WIDTH) as i64;
        if paddle_x < x {
            command = 1;
        } else if paddle_x > x {
            command = -1;
        }
        command
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let mut tape = load_tape(input_file);
    let screen = Rc::new(RefCell::new(ArcadeScreen::new()));
    let mut input = ArcadeScreenInput::new(Rc::clone(&screen));
    execute_intcode(&tape, &mut StdInput, &mut input);
    println!("{}", screen.borrow());
    println!("Part 1: {}", screen.borrow().tiles.iter().filter(|&tile| *tile == Tile::Block).count());

    tape[0] = 2;
    let screen = Rc::new(RefCell::new(ArcadeScreen::new()));
    let mut input = ArcadeScreenInput::new(Rc::clone(&screen));
    let mut output = ArcadeInput::new(Rc::clone(&screen));
    execute_intcode(&tape, &mut output, &mut input);
    println!("{}", screen.borrow());
}