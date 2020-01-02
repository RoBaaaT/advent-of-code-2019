use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fmt;
use std::result::Result;


const GRID_SIZE: usize = 5;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Bug
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            Tile::Empty => ".",
            Tile::Bug => "#"
        })?;
        Ok(())
    }
}

#[derive(PartialEq)]
struct Grid {
    tiles: [Tile; GRID_SIZE * GRID_SIZE]
}

impl Grid {
    fn from_file(file: File) -> Grid {
        let input = BufReader::new(&file);
        let mut tiles = [Tile::Empty; GRID_SIZE * GRID_SIZE];
        let mut y = 0;
        for line in input.lines() {
            assert!(y < GRID_SIZE);
            let mut x = 0;
            for tile in line.unwrap().chars() {
                assert!(x < GRID_SIZE);
                tiles[y * GRID_SIZE + x] = match tile {
                    '.' => Tile::Empty,
                    '#' => Tile::Bug,
                    _ => panic!("Unknown tile '{}'", tile)
                };
                x += 1;
            }
            y += 1;
        }
        Grid { tiles: tiles }
    }

    fn step(&self) -> Grid {
        let mut tiles = self.tiles;
        for i in 0..GRID_SIZE * GRID_SIZE {
            let mut adjacent_count = 0;
            if i >= GRID_SIZE && self.tiles[i - GRID_SIZE] == Tile::Bug {
                adjacent_count += 1;
            }
            if i < GRID_SIZE * (GRID_SIZE - 1) && self.tiles[i + GRID_SIZE] == Tile::Bug {
                adjacent_count += 1;
            }
            if i % GRID_SIZE > 0 && self.tiles[i - 1] == Tile::Bug {
                adjacent_count += 1;
            }
            if i % GRID_SIZE < GRID_SIZE - 1 && self.tiles[i + 1] == Tile::Bug {
                adjacent_count += 1;
            }
            if self.tiles[i] == Tile::Bug && adjacent_count != 1 {
                tiles[i] = Tile::Empty;
            } else if self.tiles[i] == Tile::Empty && (adjacent_count == 1 || adjacent_count == 2) {
                tiles[i] = Tile::Bug;
            }
        }
        Grid { tiles: tiles }
    }

    fn biodiversity(&self) -> usize {
        let mut result = 0;
        for i in 0..GRID_SIZE * GRID_SIZE {
            if self.tiles[i] == Tile::Bug {
                result += 1 << i;
            }
        }
        result
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        for (i, tile) in self.tiles.iter().enumerate() {
            write!(f, "{}", *tile)?;
            if i % GRID_SIZE == GRID_SIZE - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let input_file = File::open("input.txt").unwrap();
    let grid = Grid::from_file(input_file);
    let mut prev_grids = vec![grid];
    loop {
        let grid = prev_grids.last().unwrap().step();
        if let Some(_) = prev_grids.iter().find(|&g| *g == grid) {
            prev_grids.push(grid);
            break;
        }
        prev_grids.push(grid);
    }
    println!("{}", prev_grids.last().unwrap());
    println!("Part 1: {}", prev_grids.last().unwrap().biodiversity());
}