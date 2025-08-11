use rand::{Rng, rng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct GameState {
    pub grid: [[u32; 4]; 4],
    pub score: u32,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
            grid: [[0; 4]; 4],
            score: 0,
        };
        state.add_random_tile();
        state.add_random_tile();
        state
    }

    pub fn add_random_tile(&mut self) {
        let mut rng = rng();
        let mut empty_positions = Vec::new();

        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j] == 0 {
                    empty_positions.push((i, j));
                }
            }
        }

        if !empty_positions.is_empty() {
            let &(i, j) = empty_positions
                .get(rng.random_range(0..empty_positions.len()))
                .unwrap();
            self.grid[i][j] = if rng.random::<f32>() < 0.9 { 2 } else { 4 };
        }
    }

    pub fn move_tiles(&mut self, direction: Direction) {
        let moved = match direction {
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
        };
        if moved {
            self.add_random_tile();
        }
    }

    fn move_left(&mut self) -> bool {
        let mut moved = false;
        for row in self.grid.iter_mut() {
            let mut new_row = [0; 4];
            let mut index = 0;
            let mut prev: Option<u32> = None;

            for &tile in row.iter().filter(|&&t| t != 0) {
                if let Some(p) = prev {
                    if p == tile {
                        new_row[index] = tile * 2;
                        self.score += tile * 2;
                        prev = None;
                        index += 1;
                        moved = true;
                    } else {
                        new_row[index] = p;
                        prev = Some(tile);
                        index += 1;
                    }
                } else {
                    prev = Some(tile);
                }
            }

            if let Some(p) = prev {
                new_row[index] = p;
            }

            if *row != new_row {
                moved = true;
                *row = new_row;
            }
        }
        moved
    }

    fn move_right(&mut self) -> bool {
        self.rotate_grid();
        self.rotate_grid();
        let moved = self.move_left();
        self.rotate_grid();
        self.rotate_grid();
        moved
    }

    fn move_up(&mut self) -> bool {
        self.rotate_grid();
        self.rotate_grid();
        self.rotate_grid();
        let moved = self.move_left();
        self.rotate_grid();
        moved
    }

    fn move_down(&mut self) -> bool {
        self.rotate_grid();
        let moved = self.move_left();
        self.rotate_grid();
        self.rotate_grid();
        self.rotate_grid();
        moved
    }

    fn rotate_grid(&mut self) {
        let mut new_grid = [[0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                new_grid[j][3 - i] = self.grid[i][j];
            }
        }
        self.grid = new_grid;
    }

    pub fn is_game_over(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.grid[i][j] == 0 {
                    return false;
                }
                if j < 3 && self.grid[i][j] == self.grid[i][j + 1] {
                    return false;
                }
                if i < 3 && self.grid[i][j] == self.grid[i + 1][j] {
                    return false;
                }
            }
        }
        true
    }
}

pub fn print_game(game: &GameState) {
    println!("Score: {}", game.score);
    println!();

    for row in &game.grid {
        for &tile in row {
            let s = if tile == 0 {
                "    ".to_string()
            } else {
                format!("{:4}", tile)
            };
            print!("{} ", s);
        }
        println!();
    }
    println!();
}
