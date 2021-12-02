use crate::{cell_properties::State, cells::*};
use glam::const_vec4;
use macroquad::prelude::*;

pub const SCALE: f32 = 10.0;
pub const SSCALE: f32 = 0.7071067812 * SCALE;

pub const SKY: Vec4 = const_vec4!([209.0 / 255.0, 231.0 / 255.0, 255.0 / 255.0, 1.0]);
// pub const SKY: Vec4 = const_vec4!([227.0 / 255.0, 213.0 / 255.0, 186.0 / 255.0, 1.0]);
pub const DARK_BACK: Color = Color {
    r: 33.0 / 255.0,
    g: 33.0 / 255.0,
    b: 33.0 / 255.0,
    a: 1.0,
};
pub const SAND_COLOR: Vec4 = const_vec4!([255.0 / 255.0, 201.0 / 255.0, 25.0 / 255.0, 1.0]);
pub const BEDROCK_COLOR: Vec4 = const_vec4!([28.0 / 255.0, 28.0 / 255.0, 28.0 / 255.0, 1.0]);
pub const WATER_COLOR: Vec4 = const_vec4!([0.0 / 255.0, 136.0 / 255.0, 255.0 / 255.0, 1.0]);
pub const SMOKE_COLOR: Vec4 = const_vec4!([80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0]);
pub const GUNPOWDER_COLOR: Vec4 = const_vec4!([50.0 / 255.0, 50.0 / 255.0, 50.0 / 255.0, 1.0]);
pub const STONE_COLOR: Vec4 = const_vec4!([56.0 / 255.0, 52.0 / 255.0, 48.0 / 255.0, 1.0]);
pub const FIRE_COLOR: Vec4 = const_vec4!([255.0 / 255.0, 50.0 / 255.0, 14.0 / 255.0, 1.0]);
pub const FIRE_COLOR_0: Vec4 = const_vec4!([255.0 / 255.0, 149.0 / 255.0, 0.0 / 255.0, 1.0]);

pub const OIL_COLOR: Vec4 = const_vec4!([10.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0, 1.0]);
pub const WOOD_COLOR: Vec4 = const_vec4!([120.0 / 255.0, 73.0 / 255.0, 10.0 / 46.0, 1.0]);
pub const LIFE_COLOR: Vec4 = const_vec4!([0.0, 0.0, 0.0, 0.0]);

pub const TRANSPARENT: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.0,
};

pub const BURN: Vec4 = const_vec4!([0.01, 0.01, 0.01, 0.0]);

pub static AIR: Cell = Cell {
    identity: CT::Air,
    health: 10.0,
    inertial_res: 0.0,
    variation: 0.0,
    tick: false,
    active: true,
    vorticity: 0.0,
    flammability: 0.0,
    mass: 0.0,
    viscosity: 0,
    color: SKY,
    is_wet: false,
    velocity: Vec2::ZERO,
    terminal_v: 0.0,
    heat: 28.0,
    boiling_point: 1000000.0,
    melting_point: 1000000.0,
    conductivity: 0.4,
    state: State::Solid,
};

pub static LIFE: Cell = Cell {
    identity: CT::Life,
    health: 10.0,
    inertial_res: 0.0,
    variation: 0.0,
    tick: false,
    active: true,
    vorticity: 0.0,
    flammability: 0.0,
    mass: 0.0,
    viscosity: 0,
    color: LIFE_COLOR,
    is_wet: false,
    velocity: Vec2::ZERO,
    terminal_v: 0.0,
    heat: 0.0,
    boiling_point: 1000000.0,
    melting_point: 1000000.0,
    conductivity: 0.0,
    state: State::Solid,
};

pub static BEDROCK: Cell = Cell {
    identity: CT::Bedrock,
    health: 10000.0,
    inertial_res: 1.0,
    variation: 0.0,
    tick: false,
    active: false,
    vorticity: 0.0,
    flammability: 0.0,
    mass: 0.0,
    viscosity: 0,
    color: BEDROCK_COLOR,
    is_wet: false,
    velocity: Vec2::ZERO,
    terminal_v: 0.0,
    heat: 0.0,
    boiling_point: 1000000.0,
    melting_point: 1000000.0,
    conductivity: 0.0,
    state: State::Static,
};

pub const FRAGMENT_SHADER: &str = "
    #version 100
    precision lowp float;
    varying vec4 color;
    varying vec2 uv;
    uniform sampler2D Texture;
        
    void main() {
        vec4 Color = texture2D(Texture, uv);
        // if(Color[3] != 0.0){

        float Pi = 6.28318530718; // Pi*2

        // GAUSSIAN BLUR SETTINGS {{{
        float Directions = 16.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
        float Quality = 4.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
        float Size = 10.0; // BLUR SIZE (Radius)
        // GAUSSIAN BLUR SETTINGS }}}
    
        vec2 Radius = Size / vec2(1000.0, 1000.0);
        
        // Normalized pixel coordinates (from 0 to 1)
        // vec2 uv = fragCoord/iResolution.xy;
        // Pixel colour
        
        // Blur calculations
        for( float d=0.0; d<Pi; d+=Pi/Directions)
        {
            for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
            {
                Color += texture2D(Texture, uv+vec2(cos(d),sin(d))*Radius*i);		
            }
        }
        
        // Output to screen
        Color /= Quality * Directions - 15.0;

        gl_FragColor = Color ; //* 1.2;
        // } else {
            // gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        // }



        // vec4 texture_color = texture2D(Texture, uv);
        // if(texture_color[3] != 0.0){
        //     gl_FragColor = texture_color;
        // }
    }
";

pub const VERTEX_SHADER: &str = "
    #version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    attribute vec4 color0;
    varying lowp vec2 uv;
    varying lowp vec4 color;
    uniform mat4 Model;
    uniform mat4 Projection;
    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
        color = color0 / 255.0;
        uv = texcoord;
    }
";
