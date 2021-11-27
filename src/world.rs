use macroquad::prelude::{
    draw_texture, draw_texture_ex, screen_height, screen_width, vec2, DrawTextureParams, Texture2D,
    WHITE,
};

use crate::cells::*;
use crate::constants::{AIR, SCALE};
use crate::controller::*;

// static mut TEXTURE: [u8; 1920 * 1080 * 4] = [0; 1920 * 1080 * 4];

pub struct World {
    pub cells: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub tick: bool,
    pub length: i32,
    // pub texture: [u8; 1920 * 1080 * 4], // 8294400
}

impl World {
    pub fn new() -> Self {
        let (sw, sh) = (screen_width() / SCALE, screen_height() / SCALE);

        let mut cells = Vec::new();

        let length = (sw * sh) as i32;

        (0..length).for_each(|_| {
            cells.push(AIR);
        });

        World {
            cells,
            width: sw as i32,
            height: sh as i32,
            tick: true,
            length,
            // texture: TEXTURE,
        }
    }

    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + self.width * y) as usize
    }

    pub fn get_coords(&self, i: usize) -> (i32, i32) {
        (i as i32 % self.width, i as i32 / self.width)
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_idx(x, y)]
    }

    pub fn outof_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || x > self.width - 1 || y < 0 || y > self.height - 1
    }

    // pub fn tick(&mut self) {
    //     (0..self.height).for_each(|y| {
    //         (0..self.width).for_each(|x| {
    //             let i = self.get_idx(x, y);
    //             let cell = self.cells[i];
    //             if !cell.identity.is_static() && cell.active && self.tick != cell.tick {
    //                 cell.step(Controller { x, y, world: self });
    //             }

    //             let b = i * 4 * 5;
    //             for j in 0..5 {
    //                 for k in 0..5 {
    //                     let p = (i / *self.width as usize) + k;

    //                     // println!("p {:?}", p)
    //                     unsafe {
    //                         TEXTURE[b + p] = tex[0];
    //                         TEXTURE[b + p + 1] = tex[1];
    //                         TEXTURE[b + p + 2] = tex[2];
    //                         TEXTURE[b + p + 3] = tex[3];
    //                     };
    //                 }
    //             }
    //         });
    //     });
    //     // (0..self.length as usize).for_each(|i| {
    //     //     let cell = self.cells[i];

    //     //     if !cell.identity.is_static() && cell.active && self.tick != cell.tick {
    //     //         let (x, y) = self.get_coords(i);
    //     //         cell.step(Controller { x, y, world: self });
    //     //     }
    //     //     let tex = cell.tex();
    //     //     // println!("i {:?}", i);

    //     //     let b = i * 4 * 5;
    //     //     for j in 0..5 {
    //     //         for k in 0..5 {
    //     //             let p = (i /  * self.width as usize) + k;

    //     //             // println!("p {:?}", p)
    //     //             unsafe {
    //     //                 TEXTURE[b + p] = tex[0];
    //     //                 TEXTURE[b + p + 1] = tex[1];
    //     //                 TEXTURE[b + p + 2] = tex[2];
    //     //                 TEXTURE[b + p + 3] = tex[3];
    //     //             };
    //     //         }
    //     //     }
    //     // });

    //     unsafe {
    //         let texture = Texture2D::from_rgba8(
    //             1920, 1080,
    //             // (self.width as i32) as u16,
    //             // (self.height as i32) as u16,
    //             &TEXTURE,
    //         );
    //         draw_texture(texture, 0.0, 0.0, WHITE);
    //     }
    //     // draw_texture_ex(
    //     //     texture,
    //     //     0.0,
    //     //     0.0,
    //     //     WHITE,
    //     //     DrawTextureParams {
    //     //         dest_size: Some(vec2(1920.0, 1080.0)),
    //     //         source: None,
    //     //         rotation: 0.0,
    //     //         flip_x: false,
    //     //         flip_y: false,
    //     //         pivot: None,
    //     //     },
    //     // );

    //     self.tick = !self.tick;
    // }

    pub fn tick(&mut self) {
        // let mut buffer: Vec<u8> = Vec::new();
        // let mut temp_buffer: Vec<u8> = Vec::new();

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let cell = self.get_cell(x, y);
                if !cell.identity.is_static() && cell.active && self.tick != cell.tick {
                    cell.step(Controller { x, y, world: self });
                }

                // self.texture[]

                // buffer.extend_from_slice(&cell.tex());
                // let pixel = cell.tex();
                // (0..5).for_each(|_| temp_buffer.extend_from_slice(&pixel));

                // cell.debug_draw(x, y);

                cell.draw(x, y);
            });

            // (0..5).for_each(|_| buffer.extend_from_slice(&temp_buffer));
            // temp_buffer.clear();
        });

        // let texture = Texture2D::from_rgba8(
        //     (self.width * SCALE as i32) as u16,
        //     (self.height * SCALE as i32) as u16,
        //     &buffer,
        // );
        // // draw_texture(texture, 0.0, 0.0, WHITE);
        // draw_texture_ex(
        //     texture,
        //     0.0,
        //     0.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(vec2(1920.0, 1080.0)),
        //         source: None,
        //         rotation: 0.0,
        //         flip_x: false,
        //         flip_y: false,
        //         pivot: None,
        //     },
        // );

        self.tick = !self.tick;
    }
}
