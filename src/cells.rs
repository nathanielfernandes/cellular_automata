use crate::behaviors::*;
use crate::cell_properties::State;
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
    GunPowder = 6,
    Fire = 7,
    Oil = 8,
    Wood = 9,
    Lava = 10,
    Life = 11,
    Stone = 12,
}

#[derive(Clone, Copy)]

pub struct Cell {
    pub identity: CT,

    //  flags
    pub tick: bool,
    pub active: bool,
    pub is_wet: bool,

    // movement
    pub inertial_res: f32,
    pub velocity: Vec2,
    pub terminal_v: f32,
    pub viscosity: u8, // higher == faster?
    pub vorticity: f32,
    pub mass: f32,

    // physical
    pub health: f32,
    pub flammability: f32,
    pub variation: f32,
    pub heat: f32,
    pub boiling_point: f32,
    pub melting_point: f32,
    pub conductivity: f32,
    pub color: Vec4,
    pub state: State,
}

impl Cell {
    pub fn new(identity: CT, tick: bool, active: bool, _velocity: Vec2) -> Self {
        Cell {
            identity,
            health: if identity == CT::Air {
                AIR.health
            } else {
                identity.get_health()
            },
            tick,
            active,
            ..Cell::default(identity)
        }
    }

    pub fn default(identity: CT) -> Self {
        Cell {
            identity,
            health: identity.get_health(),
            inertial_res: identity.get_inertial_res(),
            variation: identity.get_varation(),
            tick: false,
            active: true,
            vorticity: identity.get_vorticity(),
            flammability: identity.get_flammability(),
            mass: identity.get_mass(),
            viscosity: identity.get_viscosity(),
            color: identity.get_color(),
            is_wet: false,
            velocity: Vec2::ZERO,
            terminal_v: identity.get_terminal_v(),
            heat: identity.get_heat(),
            boiling_point: identity.get_boiling_point(),
            melting_point: identity.get_melting_point(),
            conductivity: identity.get_conductivity(),
            state: identity.get_state(),
        }
    }

    pub fn step(&self, ctrl: Controller) {
        match self.identity {
            CT::Air => false, //solid_step(*self, ctrl),
            CT::Fire => fire_step(*self, ctrl),
            CT::Lava => lava_step(*self, ctrl),
            _ => match self.state {
                State::Gas => gas_step(*self, ctrl),
                State::Solid => solid_step(*self, ctrl),
                State::FallingSolid => falling_solid_step(*self, ctrl),
                State::Liquid => liquid_step(*self, ctrl),
                State::Static => false,
            },
        };

        // self.identity.step(*self, check_cell);
    }

    pub fn draw(&self, x: i32, y: i32) {
        self.identity.draw(x, y, *self)
    }

    // pub fn tex(&self) -> Vec<u8> {
    //     self.identity.tex(*self)
    // }
    // pub fn color(&self) -> macroquad::prelude::Color {
    //     self.identity.color(*self)
    // }

    pub fn debug_draw(&self, x: i32, y: i32) {
        if !self.active || self.is_immovable() {
            self.identity.draw(x, y, *self)
        } else if !self.is_static() {
            draw_poly(x as f32 * SCALE, y as f32 * SCALE, 4, SSCALE, 45.0, GREEN);
        }
    }

    pub fn wake(&mut self) {
        self.active = true;
    }

    pub fn try_wake(&mut self) {
        if !self.is_static() && inertial_res_check(self.inertial_res) {
            self.wake();
        }
    }

    pub fn burn(color: Vec4) -> Vec4 {
        color * 0.90
    }

    pub fn is_boiling(&self) -> bool {
        self.heat >= self.boiling_point
    }

    pub fn is_melting(&self) -> bool {
        self.heat >= self.melting_point
    }

    pub fn is_freezing(&self) -> bool {
        self.heat < self.melting_point
    }

    pub fn is_immovable(&self) -> bool {
        self.state == State::Solid || self.state == State::Static
    }

    pub fn is_solid(&self) -> bool {
        self.state == State::Solid
            || self.state == State::FallingSolid
            || self.state == State::Static
    }
    pub fn is_liquid(&self) -> bool {
        self.state == State::Liquid
    }
    pub fn is_gas(&self) -> bool {
        self.state == State::Gas
    }
    pub fn is_static(&self) -> bool {
        self.state == State::Static
    }
    pub fn is_emissive(&self) -> bool {
        match self.identity {
            CT::Fire => true,
            CT::Smoke => true,
            CT::Lava => true,
            _ => false,
        }
    }
    // pub fn check_state_change(&self) -> Cell {

    // }

    pub fn force_boil(&self) -> Cell {
        return match self.identity {
            CT::Water => Cell::new(CT::Smoke, self.tick, true, Vec2::ZERO),
            CT::Oil => Cell::new(CT::Fire, self.tick, true, Vec2::ZERO),
            CT::Wood => Cell::new(CT::Fire, self.tick, true, Vec2::ZERO),
            CT::GunPowder => Cell::new(CT::Fire, self.tick, true, Vec2::ZERO),
            _ => *self,
        };
    }

    pub fn force_melt(&self) -> Cell {
        return match self.identity {
            CT::Wood => Cell::new(CT::Fire, self.tick, true, Vec2::ZERO),
            // CT::Lava =>
            _ => *self,
        };
    }

    pub fn force_freeze(&self) -> Cell {
        return match self.identity {
            CT::Lava => Cell {
                heat: self.heat,
                ..Cell::new(CT::Stone, self.tick, true, Vec2::ZERO)
            },

            // ,
            _ => *self,
        };
    }

    pub fn heat_transform(&self) -> Cell {
        if self.is_boiling() {
            return self.force_boil();
        } else if self.is_melting() {
            return self.force_melt();
        } else if self.is_freezing() {
            return self.force_freeze();
        }

        *self
    }

    pub fn draw_info(&self) {
        draw_text(
            &format!("identity: {}", self.identity.to_str()),
            2.0,
            50.0,
            25.0,
            WHITE,
        );
        draw_text(&format!("active: {}", self.active,), 2.0, 70.0, 25.0, WHITE);
        draw_text(&format!("tick: {}", self.tick,), 2.0, 90.0, 25.0, WHITE);
        draw_text(&format!("is wet: {}", self.is_wet), 2.0, 110.0, 25.0, WHITE);
        draw_text(&format!("health: {}", self.health), 2.0, 130.0, 25.0, WHITE);
        draw_text(&format!("heat: {}", self.heat), 2.0, 150.0, 25.0, WHITE);
    }
}

impl CT {
    // pub fn bloom(&self)

    pub fn draw(&self, x: i32, y: i32, cell: Cell) {
        if cell.identity == CT::Air {
            return;
        }

        let a;
        if cell.is_gas() {
            a = cell.health as f32 / cell.identity.get_health() as f32;
        } else {
            if cell.active && cell.identity != CT::Bedrock {
                a = 0.8
            } else {
                a = 1.0
            }
        }

        draw_poly(
            x as f32 * SCALE,
            y as f32 * SCALE,
            4,
            SSCALE,
            45.0,
            Color::from_vec(vec4(
                &cell.color[0] + &cell.variation,
                &cell.color[1] + &cell.variation,
                &cell.color[2] + &cell.variation,
                a,
            )),
        )

        // draw_poly(
        //     x as f32 * SCALE,
        //     y as f32 * SCALE,
        //     4,
        //     SSCALE,
        //     45.0,
        //     Color::from_vec(vec4(
        //         cell.heat / 100.0,
        //         1.0 - (cell.heat / 100.0),
        //         0.0,
        //         // &cell.color[2] + &cell.variation,
        //         a,
        //     )),
        // )
    }

    pub fn wet(&self, color: Vec4, flammability: f32, health: f32) -> (Vec4, f32, f32) {
        match self {
            CT::Water => (color * 0.8, flammability * 0.5, (health * 1.1)),
            CT::Oil => (color * 0.5, flammability * 2.5, (health * 0.8)),
            CT::Lava => (color * 0.1, flammability * 10.0, (health * 0.3)),
            _ => (color, flammability, health),
        }
    }

    // pub fn tex(&self, cell: Cell) -> Vec<u8> {
    //     let c = match self {
    //         CT::Air => SKY,
    //         CT::Bedrock => BEDROCK_COLOR,
    //         CT::Sand => SAND_COLOR,
    //         CT::Water => WATER_COLOR,
    //         CT::Smoke => SMOKE_COLOR,
    //         CT::GunPowder => GUNPOWDER_COLOR,
    //         CT::Fire => FIRE_COLOR,
    //         CT::Oil => OIL_COLOR,
    //     }
    //     .to_vec();

    //     let a;
    //     if cell.identity == CT::Smoke {
    //         a = cell.health as f32 / 255.0;
    //     } else {
    //         if cell.active && cell.identity != CT::Bedrock {
    //             a = 0.8
    //         } else {
    //             a = 1.0
    //         }
    //     }
    //     let var = cell.variation;
    //     vec![
    //         ((c[0] + var) * 255.0) as u8,
    //         ((c[1] + var) * 255.0) as u8,
    //         ((c[2] + var) * 255.0) as u8,
    //         (a * 255.0) as u8,
    //     ]
    // }

    // pub fn color(&self, cell: Cell) -> macroquad::prelude::Color {
    //     let c = match self {
    //         CT::Air => SKY,
    //         CT::Bedrock => BEDROCK_COLOR,
    //         CT::Sand => SAND_COLOR,
    //         CT::Water => WATER_COLOR,
    //         CT::Smoke => SMOKE_COLOR,
    //         CT::GunPowder => GUNPOWDER_COLOR,
    //         CT::Fire => FIRE_COLOR,
    //         CT::Oil => OIL_COLOR,
    //     }
    //     .to_vec();

    //     let a;
    //     if cell.identity == CT::Smoke {
    //         a = cell.health as f32 / 255.0;
    //     } else {
    //         if cell.active && cell.identity != CT::Bedrock {
    //             a = 0.8
    //         } else {
    //             a = 1.0
    //         }
    //     }
    //     let var = cell.variation;
    //     Color::from_vec(vec4(c[0] + var, c[1] + var, c[2] + var, a))
    // }
}
