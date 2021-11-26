use crate::cells::*;
use crate::constants::*;
use crate::world::World;

pub struct Controller<'sandy> {
    pub x: i32,
    pub y: i32,
    pub world: &'sandy mut World,
}

impl<'sandy> Controller<'sandy> {
    pub fn wake_cells(&mut self, x: i32, y: i32) {
        // TODO fix this cringe
        if !self.world.outof_bounds(x - 1, y) {
            let i = self.world.get_idx(x - 1, y);
            self.world.cells[i].active = true;
        }

        if !self.world.outof_bounds(x + 1, y) {
            let i = self.world.get_idx(x + 1, y);
            self.world.cells[i].active = true;
        }

        if !self.world.outof_bounds(x, y - 1) {
            let i = self.world.get_idx(x, y - 1);
            self.world.cells[i].active = true;
        }

        if !self.world.outof_bounds(x, y + 1) {
            let i = self.world.get_idx(x, y + 1);
            self.world.cells[i].active = true;
        }
    }

    pub fn sleep_cell(&mut self) {
        let i = self.world.get_idx(self.x, self.y);
        self.world.cells[i].active = false;
    }

    pub fn get_rel_cell(&mut self, delta_x: i32, delta_y: i32) -> Cell {
        let (x, y) = (self.x + delta_x, self.y + delta_y);
        if self.world.outof_bounds(x, y) {
            return BEDROCK;
        }
        self.world.get_cell(x, y)
    }

    pub fn set_rel_cell(&mut self, cell: Cell, delta_x: i32, delta_y: i32) {
        let (x, y) = (self.x + delta_x, self.y + delta_y);

        if self.world.outof_bounds(x, y) {
            return;
        }

        let i = self.world.get_idx(x, y);
        self.world.cells[i] = cell;
        self.world.cells[i].tick = self.world.tick;
        self.wake_cells(x, y);
    }
}
