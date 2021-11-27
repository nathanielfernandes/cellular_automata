use crate::constants::*;
use crate::controller::*;
use crate::random_utils::inertial_res_check;

use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]

pub enum CT {
    Air = 0,
    Bedrock = 1,
    Sand = 2,
    Water = 3,
    Smoke = 4,
}

#[derive(Clone, Copy)]

pub struct Cell {
    pub identity: CT,
    pub health: u8,
    pub inertial_res: f32,
    pub variation: f32,
    pub tick: bool,
    pub active: bool,
    pub has_moved: bool, // has moved last tick
}
impl Cell {
    pub fn new(identity: CT, tick: bool, active: bool) -> Self {
        Cell {
            identity,
            health: identity.get_health(),
            inertial_res: identity.get_inertial_res(),
            variation: identity.get_varation(),
            tick,
            active,
            has_moved: identity.get_moved(),
        }
    }

    pub fn step(&self, check_cell: Controller) {
        self.identity.step(*self, check_cell);
    }

    pub fn draw(&self, x: i32, y: i32) {
        self.identity.draw(x, y, *self)
    }

    pub fn tex(&self) -> Vec<u8> {
        self.identity.tex(*self)
    }

    pub fn debug_draw(&self, x: i32, y: i32) {
        if !self.active {
            self.identity.draw(x, y, *self)
        } else {
            draw_poly(x as f32 * SCALE, y as f32 * SCALE, 4, SSCALE, 45.0, GREEN);
        }
    }

    pub fn wake(&mut self) {
        if !self.identity.is_static() {
            //&& inertial_res_check(self.inertial_res) {
            self.has_moved = true;
            self.active = true;
        }
    }
}

impl CT {
    pub fn draw(&self, x: i32, y: i32, cell: Cell) {
        let c = match self {
            CT::Air => return,
            CT::Bedrock => BEDROCK_COLOR,
            CT::Sand => SAND_COLOR,
            CT::Water => WATER_COLOR,
            CT::Smoke => SMOKE_COLOR,
        }
        .to_vec();

        let a;
        if cell.identity == CT::Smoke {
            a = cell.health as f32 / 255.0;
        } else {
            if cell.active && cell.identity != CT::Bedrock {
                a = 0.8
            } else {
                a = 1.0
            }
        }

        let var = cell.variation;

        draw_poly(
            x as f32 * SCALE,
            y as f32 * SCALE,
            4,
            SSCALE,
            45.0,
            Color::from_vec(vec4(c[0] + var, c[1] + var, c[2] + var, a)),
        )
    }

    pub fn tex(&self, cell: Cell) -> Vec<u8> {
        let c = match self {
            CT::Air => SKY,
            CT::Bedrock => BEDROCK_COLOR,
            CT::Sand => SAND_COLOR,
            CT::Water => WATER_COLOR,
            CT::Smoke => SMOKE_COLOR,
        }
        .to_vec();

        let a;
        if cell.identity == CT::Smoke {
            a = cell.health as f32 / 255.0;
        } else {
            if cell.active && cell.identity != CT::Bedrock {
                a = 0.8
            } else {
                a = 1.0
            }
        }
        let var = cell.variation;
        vec![
            ((c[0] + var) * 255.0) as u8,
            ((c[1] + var) * 255.0) as u8,
            ((c[2] + var) * 255.0) as u8,
            (a * 255.0) as u8,
        ]
    }

    pub fn get_varation(&self) -> f32 {
        match self {
            CT::Air => 0.0,
            CT::Bedrock => 0.0,
            CT::Sand => rand::gen_range(-0.05, 0.05),
            CT::Water => 0.0,
            CT::Smoke => rand::gen_range(-0.2, 0.2),
        }
    }

    pub fn get_health(&self) -> u8 {
        match self {
            CT::Air => 255,
            CT::Bedrock => 255,
            CT::Sand => 35,
            CT::Water => 20,
            CT::Smoke => 255,
        }
    }

    pub fn get_inertial_res(&self) -> f32 {
        match self {
            CT::Air => 0.0,
            CT::Bedrock => 1.0,
            CT::Sand => 0.1,
            CT::Water => 0.0,
            CT::Smoke => 0.0,
        }
    }

    pub fn get_moved(&self) -> bool {
        match self {
            CT::Air => true,
            CT::Bedrock => false,
            CT::Sand => true,
            CT::Water => true,
            CT::Smoke => true,
        }
    }

    pub fn is_gas(&self) -> bool {
        match self {
            CT::Air => true,
            CT::Bedrock => false,
            CT::Sand => false,
            CT::Water => false,
            CT::Smoke => true,
        }
    }

    pub fn is_liquid(&self) -> bool {
        match self {
            CT::Air => false,
            CT::Bedrock => false,
            CT::Sand => false,
            CT::Water => true,
            CT::Smoke => false,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            CT::Air => false,
            CT::Bedrock => true,
            CT::Sand => true,
            CT::Water => false,
            CT::Smoke => false,
        }
    }

    pub fn is_not(&self, cell_type: &CT) -> bool {
        self != cell_type
    }

    pub fn to_str(&self) -> &str {
        match self {
            CT::Air => "air",
            CT::Bedrock => "bedrock",
            CT::Sand => "sand",
            CT::Water => "water",
            CT::Smoke => "smoke",
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            CT::Air => true,
            CT::Bedrock => true,
            CT::Sand => false,
            CT::Water => false,
            CT::Smoke => false,
        }
    }
}
