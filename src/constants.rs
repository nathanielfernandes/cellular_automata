use crate::cells::*;
use macroquad::prelude::*;

pub const SCALE: f32 = 5.0;
pub const SSCALE: f32 = 0.7071067812 * SCALE;

pub const SKY: Color = Color {
    r: 209.0 / 255.0,
    g: 231.0 / 255.0,
    b: 255.0 / 255.0,
    a: 1.0,
};

pub const SAND_COLOR: Color = Color {
    r: 255.0 / 255.0,
    g: 201.0 / 255.0,
    b: 25.0 / 255.0,
    a: 1.0,
};

pub const BEDROCK_COLOR: Color = Color {
    r: 28.0 / 255.0,
    g: 28.0 / 255.0,
    b: 28.0 / 255.0,
    a: 1.0,
};

pub const WATER_COLOR: Color = Color {
    r: 0.0 / 255.0,
    g: 136.0 / 255.0,
    b: 255.0 / 255.0,
    a: 1.0,
};

pub const SMOKE_COLOR: Color = Color {
    r: 74.0 / 255.0,
    g: 74.0 / 255.0,
    b: 74.0 / 255.0,
    a: 1.0,
};

pub static AIR: Cell = Cell {
    identity: CT::Air,
    health: 0,
    variation: 0.0,
    tick: false,
    active: false,
};

pub static BEDROCK: Cell = Cell {
    identity: CT::Bedrock,
    health: 255,
    variation: 0.0,
    tick: false,
    active: false,
};
