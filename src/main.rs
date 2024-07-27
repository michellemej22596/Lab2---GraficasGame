extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SCALE: usize = 5; // Factor de escalado

#[derive(Clone, Copy)]
enum Cell {
    Alive,
    Dead,
}

struct GameOfLife {
    grid: Vec<Vec<Cell>>,
}

impl GameOfLife {
    fn new() -> GameOfLife {
        let mut grid = vec![vec![Cell::Dead; WIDTH]; HEIGHT];
        
        // Inicializar patrones
        // Filas y columnas de patrones para llenar la pantalla
        for i in 0..WIDTH/20 {
            for j in 0..HEIGHT/20 {
                GameOfLife::initialize_block(&mut grid, 10 * i, 10 * j);
                GameOfLife::initialize_blinker(&mut grid, 10 * i + 5, 10 * j);
                GameOfLife::initialize_glider(&mut grid, 10 * i, 10 * j + 5);
                GameOfLife::initialize_pulsar(&mut grid, 10 * i + 5, 10 * j + 5);
                GameOfLife::initialize_beacon(&mut grid, 10 * i, 10 * j + 10);
                GameOfLife::initialize_toad(&mut grid, 10 * i + 10, 10 * j);
                GameOfLife::initialize_lwss(&mut grid, 10 * i + 15, 10 * j + 5);
                GameOfLife::initialize_mwss(&mut grid, 10 * i + 20, 10 * j);
                GameOfLife::initialize_hwss(&mut grid, 10 * i + 5, 10 * j + 15);
                GameOfLife::initialize_loaf(&mut grid, 10 * i + 25, 10 * j + 5);
                GameOfLife::initialize_boat(&mut grid, 10 * i, 10 * j + 20);
                GameOfLife::initialize_tub(&mut grid, 10 * i + 5, 10 * j + 25);
                GameOfLife::initialize_pentadecathlon(&mut grid, 10 * i + 10, 10 * j + 15);
                GameOfLife::initialize_guns(&mut grid, 10 * i + 5, 10 * j + 15);
            }
        }
        
        GameOfLife { grid }
    }

    fn initialize_block(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![(x, y), (x+1, y), (x, y+1), (x+1, y+1)];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_blinker(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![(x, y), (x+1, y), (x+2, y)];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_glider(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![(x, y), (x+1, y+1), (x+2, y+1), (x, y+2), (x+1, y+2)];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_pulsar(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x + 2, y), (x + 3, y), (x + 4, y), (x + 8, y), (x + 9, y), (x + 10, y),
            (x, y + 2), (x, y + 3), (x, y + 4), (x, y + 8), (x, y + 9), (x, y + 10),
            (x + 5, y + 2), (x + 5, y + 3), (x + 5, y + 4), (x + 5, y + 8), (x + 5, y + 9), (x + 5, y + 10),
            (x + 7, y + 2), (x + 7, y + 3), (x + 7, y + 4), (x + 7, y + 8), (x + 7, y + 9), (x + 7, y + 10),
            (x + 2, y + 5), (x + 3, y + 5), (x + 4, y + 5), (x + 8, y + 5), (x + 9, y + 5), (x + 10, y + 5),
            (x + 2, y + 7), (x + 3, y + 7), (x + 4, y + 7), (x + 8, y + 7), (x + 9, y + 7), (x + 10, y + 7),
            (x + 2, y + 12), (x + 3, y + 12), (x + 4, y + 12), (x + 8, y + 12), (x + 9, y + 12), (x + 10, y + 12),
            (x, y + 13), (x, y + 14), (x, y + 15), (x, y + 19), (x, y + 20), (x, y + 21),
            (x + 5, y + 13), (x + 5, y + 14), (x + 5, y + 15), (x + 5, y + 19), (x + 5, y + 20), (x + 5, y + 21),
            (x + 7, y + 13), (x + 7, y + 14), (x + 7, y + 15), (x + 7, y + 19), (x + 7, y + 20), (x + 7, y + 21),
            (x + 2, y + 17), (x + 3, y + 17), (x + 4, y + 17), (x + 8, y + 17), (x + 9, y + 17), (x + 10, y + 17)
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_beacon(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1),
            (x + 2, y + 2), (x + 3, y + 2), (x + 2, y + 3), (x + 3, y + 3)
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_toad(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x + 1, y), (x + 2, y), (x + 3, y),
            (x, y + 1), (x + 1, y + 1), (x + 2, y + 1)
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_guns(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
        (1, 5), (1, 6), (2, 5), (2, 6), (11, 5), (11, 6), (11, 7), (12, 4),
        (12, 8), (13, 3), (13, 9), (14, 3), (14, 9), (15, 6), (16, 4), (16, 8),
        (17, 5), (17, 6), (17, 7), (18, 6), (21, 3), (21, 4), (21, 5), (22, 3),
        (22, 4), (22, 5), (23, 2), (23, 6), (25, 1), (25, 2), (25, 6), (25, 7),
        (35, 3), (35, 4), (36, 3),(36,4)
    ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_lwss(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x, y + 1), (x, y + 3),
            (x + 1, y), (x + 1, y + 4),
            (x + 2, y), (x + 2, y + 4),
            (x + 3, y), (x + 3, y + 1), (x + 3, y + 2), (x + 3, y + 3),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_mwss(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x, y + 1), (x, y + 2), (x, y + 3), (x, y + 4),
            (x + 1, y), (x + 1, y + 4),
            (x + 2, y), (x + 2, y + 4),
            (x + 3, y), (x + 3, y + 1), (x + 3, y + 2), (x + 3, y + 3),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_hwss(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x, y + 1), (x, y + 2), (x, y + 3), (x, y + 4),
            (x + 1, y), (x + 1, y + 4),
            (x + 2, y), (x + 2, y + 4),
            (x + 3, y), (x + 3, y + 1), (x + 3, y + 2), (x + 3, y + 3),
            (x + 4, y + 2),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_pentadecathlon(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x + 1, y), (x + 2, y), (x + 3, y), (x + 4, y), (x + 5, y),
            (x, y + 1), (x + 6, y + 1),
            (x + 1, y + 2), (x + 2, y + 2), (x + 3, y + 2), (x + 4, y + 2), (x + 5, y + 2),
            (x, y + 3), (x + 6, y + 3),
            (x + 1, y + 4), (x + 2, y + 4), (x + 3, y + 4), (x + 4, y + 4), (x + 5, y + 4),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_tub(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x + 1, y), (x, y + 1), (x + 2, y + 1), (x + 1, y + 2),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }

    fn initialize_boat(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x, y), (x + 1, y), (x, y + 1), (x + 2, y + 1), (x + 1, y + 2),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }
    

    
    fn initialize_loaf(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
        let pattern = vec![
            (x + 1, y), (x + 2, y), (x, y + 1), (x + 3, y + 1),
            (x, y + 2), (x + 3, y + 2), (x + 1, y + 3), (x + 2, y + 3),
        ];
        for (px, py) in pattern {
            grid[py][px] = Cell::Alive;
        }
    }
    

    fn count_live_neighbors(&self, x: usize, y: usize) -> usize {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1), (0, -1),
            (0, 1), (1, -1), (1, 0), (1, 1)
        ];
        let mut live_neighbors = 0;

        for (dx, dy) in directions.iter() {
            let nx = ((x as isize + dx + WIDTH as isize) % WIDTH as isize) as usize;
            let ny = ((y as isize + dy + HEIGHT as isize) % HEIGHT as isize) as usize;
            if let Cell::Alive = self.grid[ny][nx] {
                live_neighbors += 1;
            }
        }
        live_neighbors
    }

    fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbors = self.count_live_neighbors(x, y);
                new_grid[y][x] = match (self.grid[y][x], live_neighbors) {
                    (Cell::Alive, n) if n < 2 || n > 3 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
            }
        }
        self.grid = new_grid;
    }

    fn render(&self, buffer: &mut [u32]) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = match self.grid[y][x] {
                    Cell::Alive => 0xFFFFFF,
                    Cell::Dead => 0x000000,
                };
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        let idx = (y * SCALE + dy) * WIDTH * SCALE + (x * SCALE + dx);
                        buffer[idx] = color;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut game = GameOfLife::new();

    let window_width = WIDTH * SCALE;
    let window_height = HEIGHT * SCALE;

    let mut window = Window::new(
        "Game of Life - Conway",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; window_width * window_height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.update();
        game.render(&mut buffer);
        window.update_with_buffer(&buffer, window_width, window_height).unwrap();
        std::thread::sleep(Duration::from_millis(75)); // Ajusta el valor según sea necesario
    }
}
