use crate::constants::*;
use crate::random_utils::{chance, random_color};
use crate::CT;
use macroquad::prelude::Vec4;
use macroquad::{prelude::Color, rand};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Solid = 0,
    Liquid = 1,
    Gas = 2,
    Static = 3,
}

impl CT {
    pub fn get_state(&self) -> State {
        match self {
            CT::Air => State::Gas,
            CT::Bedrock => State::Static,
            CT::Sand => State::Solid,
            CT::Water => State::Liquid,
            CT::Smoke => State::Gas,
            CT::GunPowder => State::Solid,
            CT::Fire => State::Gas,
            CT::Oil => State::Liquid,
            CT::Wood => State::Solid,
            CT::Life => State::Solid,
            CT::Lava => State::Liquid,
            CT::Stone => State::Solid,
        }
    }

    pub fn get_varation(&self) -> f32 {
        match self {
            CT::Air => 0.0,
            CT::Bedrock => 0.0,
            CT::Sand => rand::gen_range(-0.05, 0.05),
            CT::Water => 0.0,
            CT::Smoke => rand::gen_range(-0.2, 0.2),
            CT::GunPowder => rand::gen_range(-0.1, 0.1),
            CT::Fire => rand::gen_range(-0.2, 0.2),
            CT::Oil => 0.0,
            CT::Wood => rand::gen_range(-0.02, 0.02),
            CT::Stone => rand::gen_range(-0.03, 0.03),
            _ => 0.0,
        }
    }

    pub fn get_inertial_res(&self) -> f32 {
        match self {
            CT::Bedrock => 1.0,
            CT::Sand => -1.0,
            CT::GunPowder => 0.8,
            _ => 0.0,
        }
    }

    pub fn get_vorticity(&self) -> f32 {
        match self {
            CT::Smoke => 0.2,
            CT::Fire => 0.4,
            _ => 0.0,
        }
    }

    pub fn is_gas(&self) -> bool {
        match self {
            CT::Air => true,
            CT::Smoke => true,
            CT::Fire => true,
            _ => false,
        }
    }

    pub fn is_liquid(&self) -> bool {
        match self {
            CT::Water => true,
            CT::Oil => true,
            CT::Lava => true,
            _ => false,
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            CT::Bedrock => true,
            CT::Sand => true,
            CT::GunPowder => true,
            CT::Wood => true,
            CT::Life => true,
            _ => false,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CT::Air => "Air",
            CT::Bedrock => "Bedrock",
            CT::Sand => "Sand",
            CT::Water => "Water",
            CT::Smoke => "Smoke",
            CT::GunPowder => "Gun powder",
            CT::Fire => "Fire",
            CT::Oil => "Oil",
            CT::Wood => "Wood",
            CT::Life => "Life",
            CT::Lava => "Lava",
            CT::Stone => "Stone",
        }
    }

    pub fn get_color(&self) -> Vec4 {
        match self {
            CT::Air => SKY,
            CT::Bedrock => BEDROCK_COLOR,
            CT::Sand => SAND_COLOR,
            CT::Water => WATER_COLOR,
            CT::Smoke => SMOKE_COLOR,
            CT::GunPowder => GUNPOWDER_COLOR,
            CT::Fire => {
                if chance(10) {
                    FIRE_COLOR_0
                } else {
                    FIRE_COLOR
                }
            }
            CT::Oil => OIL_COLOR,
            CT::Wood => WOOD_COLOR,
            CT::Life => LIFE_COLOR,
            CT::Lava => FIRE_COLOR,
            CT::Stone => STONE_COLOR,
        }
    }

    pub fn is_static(&self) -> bool {
        match self {
            CT::Air => false,
            CT::Bedrock => true,
            _ => false,
        }
    }

    pub fn is_immovable(&self) -> bool {
        match self {
            CT::Air => true,
            CT::Bedrock => true,
            CT::Wood => true,
            _ => false,
        }
    }

    pub fn get_health(&self) -> f32 {
        match self {
            CT::Air => 255.0,
            CT::Bedrock => 255.0,
            CT::Sand => 35.0,
            CT::Water => 20.0,
            CT::Smoke => 255.0,
            CT::GunPowder => 1.0,
            CT::Fire => 10.0,
            CT::Oil => 50.0,
            CT::Wood => 120.0,
            CT::Life => 10.0,
            CT::Lava => 40.0,
            CT::Stone => 200.0,
        }
    }
    pub fn get_flammability(&self) -> f32 {
        match self {
            CT::Smoke => 0.00000000001,
            CT::GunPowder => 1.0,
            CT::Fire => 0.00000000001,
            CT::Oil => 0.9,
            CT::Wood => 1.0,
            _ => 0.0,
        }
    }

    pub fn get_mass(&self) -> f32 {
        match self {
            CT::Water => 1.0,
            CT::Oil => 0.7,
            CT::Sand => 0.3,
            CT::Lava => 3.0,
            _ => 0.0,
        }
    }

    pub fn get_viscosity(&self) -> u8 {
        match self {
            CT::Water => 25,
            CT::Oil => 15,
            CT::Lava => 10,
            _ => 0,
        }
    }

    pub fn get_terminal_v(&self) -> f32 {
        match self {
            CT::Sand => 5.0,
            _ => 1.0,
        }
    }

    pub fn is_emissive(&self) -> bool {
        match self {
            CT::Fire => true,
            CT::Smoke => true,
            CT::Lava => true,
            // CT::Water => true,
            _ => false,
        }
    }

    pub fn get_heat(&self) -> f32 {
        match self {
            CT::Fire => 400.0,
            CT::Smoke => 101.0,
            CT::Lava => 3000.0,
            CT::Water => 28.0,
            CT::Oil => 40.0,
            CT::Wood => 10.0,
            CT::Air => 28.0,
            _ => 1.0,
        }
    }
    pub fn get_conductivity(&self) -> f32 {
        match self {
            CT::Fire => 0.9,
            CT::Smoke => 0.4,
            CT::Lava => 0.9,
            CT::Water => 1.0,
            CT::Oil => 0.2,
            CT::Wood => 0.5,
            CT::GunPowder => 0.8,
            CT::Air => 0.4,
            _ => 0.0,
        }
    }
    pub fn get_boiling_point(&self) -> f32 {
        match self {
            // CT::Fire => 1000.0,
            // CT::Smoke => 1000.0,
            // CT::Lava => 10000.0,
            CT::Water => 100.0,
            CT::Oil => 80.0,
            _ => 100000000.0,
        }
    }

    pub fn get_melting_point(&self) -> f32 {
        match self {
            CT::Water => 0.0,
            CT::Wood => 400.0,
            CT::GunPowder => 50.0,
            CT::Oil => 80.0,
            CT::Lava => 1000.0,
            _ => 100000000.0,
        }
    }

    pub fn applies_heat(&self) -> bool {
        match self {
            CT::Fire => true,
            CT::Smoke => true,
            CT::Lava => true,
            _ => false,
        }
    }
}
