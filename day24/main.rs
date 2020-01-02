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

    fn empty() -> Grid {
        Grid { tiles: [Tile::Empty; GRID_SIZE * GRID_SIZE] }
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

    fn recursive_step(&self, inner_grid: Option<&Grid>, outer_grid: Option<&Grid>) -> Grid {
        let mut tiles = self.tiles;
        for i in 0..GRID_SIZE * GRID_SIZE {
            let mut adjacent_count = 0;
            let x = i % GRID_SIZE;
            let y = i / GRID_SIZE;
            if x == GRID_SIZE / 2 && y == GRID_SIZE / 2 {
                continue;
            }
            // count adjacent bugs
            // up
            if y == GRID_SIZE / 2 + 1 && x == GRID_SIZE / 2 {
                if let Some(inner) = inner_grid {
                    for x in 0..GRID_SIZE {
                        if inner.tiles[GRID_SIZE * (GRID_SIZE - 1) + x] == Tile::Bug { adjacent_count += 1; }
                    }
                }
            } else if y > 0 {
                if self.tiles[i - GRID_SIZE] == Tile::Bug { adjacent_count += 1; }
            } else if let Some(outer) = outer_grid {
                if outer.tiles[GRID_SIZE * (GRID_SIZE / 2 - 1) + GRID_SIZE / 2] == Tile::Bug { adjacent_count += 1; }
            }
            // down
            if y == GRID_SIZE / 2 - 1 && x == GRID_SIZE / 2 {
                if let Some(inner) = inner_grid {
                    for x in 0..GRID_SIZE {
                        if inner.tiles[x] == Tile::Bug { adjacent_count += 1; }
                    }
                }
            } else if y < GRID_SIZE - 1 {
                if self.tiles[i + GRID_SIZE] == Tile::Bug { adjacent_count += 1; }
            } else if let Some(outer) = outer_grid {
                if outer.tiles[GRID_SIZE * (GRID_SIZE / 2 + 1) + GRID_SIZE / 2] == Tile::Bug { adjacent_count += 1; }
            }
            // left
            if x == GRID_SIZE / 2 + 1 && y == GRID_SIZE / 2 {
                if let Some(inner) = inner_grid {
                    for y in 0..GRID_SIZE {
                        if inner.tiles[y * GRID_SIZE + GRID_SIZE - 1] == Tile::Bug { adjacent_count += 1; }
                    }
                }
            } else if x > 0 {
                if self.tiles[i - 1] == Tile::Bug { adjacent_count += 1; }
            } else if let Some(outer) = outer_grid {
                if outer.tiles[GRID_SIZE * (GRID_SIZE / 2) + GRID_SIZE / 2 - 1] == Tile::Bug { adjacent_count += 1; }
            }
            // right
            if x == GRID_SIZE / 2 - 1 && y == GRID_SIZE / 2 {
                if let Some(inner) = inner_grid {
                    for y in 0..GRID_SIZE {
                        if inner.tiles[y * GRID_SIZE] == Tile::Bug { adjacent_count += 1; }
                    }
                }
            } else if x < GRID_SIZE - 1 {
                if self.tiles[i + 1] == Tile::Bug { adjacent_count += 1; }
            } else if let Some(outer) = outer_grid {
                if outer.tiles[GRID_SIZE * (GRID_SIZE / 2) + GRID_SIZE / 2 + 1] == Tile::Bug { adjacent_count += 1; }
            }

            // apply rules for this tile
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

    fn inner_border_empty(&self) -> bool {
        self.tiles[7] == Tile::Empty && self.tiles[17] == Tile::Empty &&
            self.tiles[11] == Tile::Empty && self.tiles[13] == Tile::Empty
    }

    fn outer_border_empty(&self) -> bool {
        self.tiles[0] == Tile::Empty && self.tiles[1] == Tile::Empty &&
            self.tiles[2] == Tile::Empty && self.tiles[3] == Tile::Empty &&
            self.tiles[4] == Tile::Empty && self.tiles[5] == Tile::Empty &&
            self.tiles[9] == Tile::Empty && self.tiles[10] == Tile::Empty &&
            self.tiles[14] == Tile::Empty && self.tiles[15] == Tile::Empty &&
            self.tiles[19] == Tile::Empty && self.tiles[20] == Tile::Empty &&
            self.tiles[21] == Tile::Empty && self.tiles[22] == Tile::Empty &&
            self.tiles[23] == Tile::Empty && self.tiles[24] == Tile::Empty
    }

    fn bug_count(&self) -> usize {
        self.tiles.iter().filter(|&tile| *tile == Tile::Bug).count()
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
    println!("Part 1: {}", prev_grids.last().unwrap().biodiversity());

    let input_file = File::open("input.txt").unwrap();
    let grid = Grid::from_file(input_file);
    let mut recursive_grids = vec![grid];
    for _minute in 0..200 {
        let mut new_recursive_grids = Vec::new();
        if !recursive_grids[0].outer_border_empty() { // new outer grid
            new_recursive_grids.push(Grid::empty().recursive_step(Some(&recursive_grids[0]), None));
        }
        for i in 0..recursive_grids.len() {
            let outer_grid = if i == 0 { None } else { Some(&recursive_grids[i - 1]) };
            let inner_grid = if i == recursive_grids.len() - 1 { None } else { Some(&recursive_grids[i + 1]) };
            new_recursive_grids.push(recursive_grids[i].recursive_step(inner_grid, outer_grid));
        }
        if !recursive_grids.last().unwrap().inner_border_empty() { // new inner grid
            new_recursive_grids.push(Grid::empty().recursive_step(None, Some(&recursive_grids.last().unwrap())));
        }
        recursive_grids = new_recursive_grids;
    }
    println!("Part 2: {}", recursive_grids.iter().fold(0, |acc, grid| acc + grid.bug_count()));
}