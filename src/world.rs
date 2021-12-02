use crate::behaviors::apply_heat;
use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation};
use macroquad::prelude::*;

use crate::cells::*;
use crate::constants::{
    AIR, BEDROCK, DARK_BACK, FRAGMENT_SHADER, SAND_COLOR, SCALE, SKY, TRANSPARENT, VERTEX_SHADER,
};
use crate::controller::*;

// static mut TEXTURE: [u8; 1920 * 1080 * 4] = [0; 1920 * 1080 * 4];

// static mut IMAGE: macroquad::texture::Image = Image {
//     bytes: vec![],
//     width: 0,
//     height: 0,
// };

pub struct World {
    pub cells: Vec<Cell>,
    pub width: i32,
    pub height: i32,
    pub tick: bool,
    pub length: i32,
    pub buffer: Vec<u8>, // pub texture: [u8; 1920 * 1080 * 4], // 8294400
    pub shader_buffer: RenderTarget,
    pub shader: Material,
    pub camera: Camera2D,
    pub tps: f64,
}

impl World {
    pub fn new() -> Self {
        let (sw, sh) = (screen_width() / SCALE, screen_height() / SCALE);

        let mut cells = Vec::new();

        let length = (sw * sh) as i32;

        (0..length).for_each(|_| {
            cells.push(AIR);
        });

        // unsafe {
        //     IMAGE.width = sw as u16;
        //     IMAGE.height = sw as u16;
        // }

        let shader_buffer = render_target(screen_width() as u32, screen_height() as u32);
        shader_buffer.texture.set_filter(FilterMode::Nearest);
        let shader = load_material(
            &VERTEX_SHADER,
            &FRAGMENT_SHADER,
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        // BlendFactor::One,
                        // BlendFactor::Value(BlendValue::SourceColor),
                        // BlendFactor::One,
                        BlendFactor::Value(BlendValue::DestinationAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        // // BlendFactor::Value(BlendValue::SourceAlpha),
                        // // BlendFactor::Value(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .unwrap();
        let camera = Camera2D {
            render_target: Some(shader_buffer),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height())) // ..Default::default()
        };
        // shader_buffer.set

        World {
            cells,
            width: sw as i32,
            height: sh as i32,
            tick: true,
            length,
            buffer: vec![0; (screen_width() * screen_height() * 4 as f32) as usize], // texture: TEXTURE,
            shader_buffer,
            shader,
            camera,
            tps: 60.0,
        }
    }

    pub fn get_idx(&self, x: i32, y: i32) -> usize {
        (x + self.width * y) as usize
    }

    pub fn get_coords(&self, i: usize) -> (i32, i32) {
        (i as i32 % self.width, i as i32 / self.width)
    }

    // pub fn get_coords_f32(&self, i: usize) -> (f32, f32) {
    //     (i as f32 % self.width as f32, i as f32 / self.width as f32)
    // }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        self.cells[self.get_idx(x, y)]
    }

    pub fn safe_get_cell(&self, x: i32, y: i32) -> Cell {
        if !self.outof_bounds(x, y) {
            return self.cells[self.get_idx(x, y)];
        }

        BEDROCK
    }

    pub fn outof_bounds(&self, x: i32, y: i32) -> bool {
        x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1
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

    // pub fn tick1(&mut self) {
    //     // let mut buffer: Vec<u8> = Vec::new();
    //     // let mut temp_buffer: Vec<u8> = Vec::new();
    //     let s = 4 * SCALE as usize;
    //     let w = (self.width * s as i32) as usize;

    //     (0..self.height).for_each(|y| {
    //         if self.tick {
    //             (0..self.width).for_each(|x| {
    //                 let cell = self.get_cell(x, y);
    //                 if !cell.identity.is_static() && cell.active && self.tick != cell.tick {
    //                     cell.step(Controller { x, y, world: self });
    //                 }

    //                 // unsafe { IMAGE.set_pixel(x as u32, y as u32, cell.color()) }

    //                 let i = self.get_idx(x, y) * s;
    //                 // println!("{:?}", i);
    //                 // if i > 0 {
    //                 //     panic!("asadsd");
    //                 // }
    //                 let pixel = cell.tex();
    //                 for j in 0..SCALE as usize {
    //                     for k in 0..SCALE as usize {
    //                         let b = i + (k * 4) + (j * w);
    //                         self.buffer[b] = pixel[0];
    //                         self.buffer[b + 1] = pixel[1];
    //                         self.buffer[b + 2] = pixel[2];
    //                         self.buffer[b + 3] = pixel[3];
    //                     }
    //                 }
    //                 // self.buffer.set_pixel(x as u32, y as u32, cell.color());
    //                 // self.texture[]

    //                 // buffer.extend_from_slice(&cell.tex());
    //                 // let pixel = cell.tex();
    //                 // (0..5).for_each(|_| temp_buffer.extend_from_slice(&pixel));

    //                 // cell.debug_draw(x, y);

    //                 // cell.draw(x, y);
    //             });
    //         } else {
    //             (1..self.width).for_each(|x| {
    //                 let x = self.width - x;
    //                 let cell = self.get_cell(x, y);
    //                 if !cell.identity.is_static() && cell.active && self.tick != cell.tick {
    //                     cell.step(Controller { x, y, world: self });
    //                 }
    //                 // unsafe { IMAGE.set_pixel(x as u32, y as u32, cell.color()) }
    //                 let i = self.get_idx(x, y) * s;
    //                 let pixel = cell.tex();
    //                 for j in 0..SCALE as usize {
    //                     for k in 0..SCALE as usize {
    //                         let b = i + (k * 4) + (j * w);

    //                         self.buffer[b] = pixel[0];
    //                         self.buffer[b + 1] = pixel[1];
    //                         self.buffer[b + 2] = pixel[2];
    //                         self.buffer[b + 3] = pixel[3];
    //                     }
    //                 }
    //                 // self.texture[]

    //                 // buffer.extend_from_slice(&cell.tex());
    //                 // let pixel = cell.tex();
    //                 // (0..5).for_each(|_| temp_buffer.extend_from_slice(&pixel));

    //                 // cell.debug_draw(x, y);

    //                 // cell.draw(x, y);
    //             });
    //         }

    //         // (0..5).for_each(|_| buffer.extend_from_slice(&temp_buffer));
    //         // temp_buffer.clear();
    //     });
    //     // unsafe {

    //     let texture = Texture2D::from_rgba8(
    //         (self.width * SCALE as i32) as u16,
    //         (self.height * SCALE as i32) as u16,
    //         &self.buffer,
    //     );
    //     // let texture = Texture2D::from_image(&self.buffer);
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
    //     // }
    //     // let texture = Texture2D::from_rgba8(
    //     //     (self.width as i32) as u16,
    //     //     (self.height as i32) as u16,
    //     //     &buffer,
    //     // );
    //     draw_texture(texture, 0.0, 0.0, WHITE);

    //     self.tick = !self.tick;
    // }

    pub fn draw(&self) {
        set_camera(&self.camera);
        clear_background(TRANSPARENT);
        // clear_background(Color::from_vec(SKY));

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                let cell = self.get_cell(x, y);
                if cell.identity.is_emissive() {
                    cell.draw(x, y);
                }
            });
        });

        set_default_camera();
        clear_background(DARK_BACK);
        // clear_background(Color::from_vec(SKY));

        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                self.get_cell(x, y).draw(x, y);
            });
        });

        gl_use_material(self.shader);
        draw_texture_ex(
            self.shader_buffer.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );

        gl_use_default_material();
    }

    pub fn tick_cell(&mut self, x: i32, y: i32, pause: bool) {
        //
        let cell = self.get_cell(x, y);
        apply_heat(cell, Controller { x, y, world: self });

        let cell = self.get_cell(x, y);
        if cell.is_boiling() {
            let i = self.get_idx(x, y);
            self.cells[i] = cell.force_boil();
        }

        let cell = self.get_cell(x, y);

        if !pause && !cell.identity.is_static() && cell.active && self.tick != cell.tick {
            cell.step(Controller { x, y, world: self });
        }

        // cell.debug_draw(x, y);
        // cell.draw(x, y);
    }

    pub fn tick(&mut self, pause: bool, tpf: u8) {
        (0..tpf).for_each(|_| {
            (0..self.height).for_each(|y| {
                if self.tick {
                    (0..self.width).for_each(|x| {
                        self.tick_cell(x, y, pause);
                    });
                } else {
                    (1..self.width).for_each(|x| {
                        self.tick_cell(self.width - x, y, pause);
                    });
                }
            });

            self.tick = !self.tick;
        });
    }
}
