mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn get_width(&self) -> u32 {
        self.width
    }
    
    pub fn get_height(&self) -> u32 {
        self.height
    }
    
    pub fn get_cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_nei_count(&self, r: u32, c: u32) -> u8 {
        let mut count = 0;
        
        for dr in [self.height - 1, 0, 1].iter().cloned() {
            for dc in [self.width -1, 0, 1].iter().cloned() {
                if dr == 0 && dc == 0 {
                    continue;
                }

                let row_ = (r + dr) % self.height;
                let col_ = (c + dc) % self.width;
                let idx = self.get_index(row_, col_);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.height {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_nei = self.live_nei_count(row, col);

                let next_cell = match (cell, live_nei) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (other, _) => other,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }
    
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }
    
    pub fn new() -> Universe {
        let w = 100;
        let h = 100;

        let cells = (0..w * h)
            .map(|i| {
                if Math::round(Math::random()) as u8 == 1 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width: w,
            height: h,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


impl Universe {
    pub fn get_bcells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_bcells(&mut self, cells: &[(u32, u32)]) {
        for (r, c) in cells.iter().cloned() {
            let idx = self.get_index(r, c);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let s = if cell == Cell::Dead { '_' } else { 'x' };
                write!(f, "{}", s)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {

    let prefix = "Hello from Local Pkg!";
    let msg = format!("{} {}", prefix, text);

    alert(&msg);
}
