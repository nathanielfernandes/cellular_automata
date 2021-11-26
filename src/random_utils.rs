use macroquad::rand;

pub fn random() -> f32 {
    rand::gen_range(-1.0, 1.0)
}

pub fn chance(n: i32) -> bool {
    rand::gen_range(0, n) == 0
}

pub fn rand_int(low: i32, high: i32) -> i32 {
    rand::gen_range(low, high)
}

pub fn random_dir() -> i32 {
    if rand::gen_range(0, 500) % 2 == 0 {
        -1
    } else {
        1
    }
}
