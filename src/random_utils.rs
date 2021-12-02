use macroquad::prelude::{rand, vec4, Vec4};

pub fn random() -> f32 {
    rand::gen_range(-1.0, 1.0)
}

pub fn chance(n: u8) -> bool {
    rand::gen_range(0, n) == 0
}

pub fn inv_chance(n: u8) -> bool {
    rand::gen_range(0, n) != 0
}

pub fn rand_int(low: i32, high: i32) -> i32 {
    rand::gen_range(low, high)
}

pub fn vorticity_check(i: f32) -> bool {
    i > rand::gen_range(0.0, 1.0)
}

pub fn flammability_check(i: f32) -> bool {
    i > rand::gen_range(0.0, 1.0)
}

pub fn inertial_res_check(i: f32) -> bool {
    i < rand::gen_range(0.0, 1.0)
}

pub fn random_color() -> Vec4 {
    vec4(
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        rand::gen_range(0.0, 1.0),
        0.0,
    )
}

pub fn random_dir() -> i32 {
    // if (rand::rand() as i32) < 0 {
    //     return -1;
    // } else {
    //     return 1;
    // }

    if rand::gen_range(0, 10000) % 2 == 0 {
        -1
    } else {
        1
    }
}

// pub fn burn(color: Vec4) -> Vec4 {
//     color - 1.0
// }
