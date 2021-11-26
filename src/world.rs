use crate::cells::*;
use crate::controller::*;

pub struct World {
    pub cells: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub tick: bool,
}

impl World {
    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + self.width * y) as usize
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_idx(x, y)]
    }

    pub fn outof_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1
    }

    pub fn tick(&mut self) {
        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                let cell = self.get_cell(x, y);
                if cell.identity != CT::Air {
                    if cell.active && self.tick != cell.tick && cell.identity != CT::Bedrock {
                        cell.step(Controller { x, y, world: self });
                    }
                    // cell.debug_draw(x, y);
                    cell.draw(x, y);
                }
            });
        });

        self.tick = !self.tick;
    }
}
