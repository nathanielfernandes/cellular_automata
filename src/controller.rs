use macroquad::prelude::Vec2;

use crate::cells::*;
use crate::constants::*;
use crate::world::World;

pub struct Controller<'sandy> {
    pub x: i32,
    pub y: i32,
    pub world: &'sandy mut World,
}

impl<'sandy> Controller<'sandy> {
    pub fn wake_cell(&mut self, x: i32, y: i32) {
        if !self.world.outof_bounds(x, y) {
            let i = self.world.get_idx(x, y);
            self.world.cells[i].wake();
        }
    }

    pub fn wake_cells(&mut self, x: i32, y: i32) {
        self.wake_cell(x - 1, y);
        self.wake_cell(x + 1, y);
        self.wake_cell(x, y - 1);
        self.wake_cell(x, y + 1);

        // diagonal
        // self.wake_cell(x - 1, y - 1);
        // self.wake_cell(x + 1, y + 1);
        // self.wake_cell(x + 1, y - 1);
        // self.wake_cell(x - 1, y + 1);
    }

    pub fn sleep_cell(&mut self) {
        let i = self.world.get_idx(self.x, self.y);
        self.world.cells[i].active = false;
        self.world.cells[i].velocity = Vec2::ZERO;

        // let mut cell = self.world.cells[i];

        // if !self.world.cells[i].has_moved {
        //     self.world.cells[i].active = false;
        // } else {
        //     self.world.cells[i].has_moved = false;
        // }
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
        // self.world.cells[i].has_moved = true;

        self.wake_cells(x, y);
    }

    pub fn set_rel_cell_no_tick(&mut self, cell: Cell, delta_x: i32, delta_y: i32) {
        let (x, y) = (self.x + delta_x, self.y + delta_y);

        if self.world.outof_bounds(x, y) {
            return;
        }

        let i = self.world.get_idx(x, y);
        self.world.cells[i] = cell;
        // self.world.cells[i].tick = !self.world.tick;
        // self.world.cells[i].has_moved = true;

        // self.wake_cells(x, y);
    }

    pub fn swap_rel_cells(&mut self, delta_x: i32, delta_y: i32) {
        let swapee = self.get_rel_cell(delta_x, delta_y);
        let swaper = self.get_rel_cell(0, 0);

        self.set_rel_cell(swapee, 0, 0);
        self.set_rel_cell(swaper, delta_x, delta_y);
    }
}
