pub mod behaviors;
pub mod cell_properties;
pub mod cells;
pub mod constants;
pub mod controller;
pub mod random_utils;
pub mod world;

use crate::cells::*;
use crate::constants::*;
use crate::controller::*;
use crate::world::*;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cellular Automata".to_owned(),
        window_width: 1920,
        window_height: 1080,
        ..Default::default()
    }
}

fn change_selected(selected: CT) -> CT {
    if is_key_down(KeyCode::Key1) {
        return CT::Sand;
    } else if is_key_down(KeyCode::Key2) {
        return CT::Water;
    } else if is_key_down(KeyCode::Key3) {
        return CT::Bedrock;
    } else if is_key_down(KeyCode::Key4) {
        return CT::Smoke;
    } else if is_key_down(KeyCode::Key5) {
        return CT::Air;
    } else if is_key_down(KeyCode::Key6) {
        return CT::GunPowder;
    } else if is_key_down(KeyCode::Key7) {
        return CT::Fire;
    } else if is_key_down(KeyCode::Key8) {
        return CT::Oil;
    } else if is_key_down(KeyCode::Key9) {
        return CT::Wood;
    } else if is_key_down(KeyCode::Key0) {
        return CT::Lava;
    }

    selected
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();

    // brush radius
    let mut r = 3;

    let mut selected = CT::Sand;
    // let sky_color = Color::from_vec(SKY);
    let mut pause = false;
    let mut skip = false;

    let mut tps: f64 = 120.0;
    // let mut last_time = 0.0_f64;
    // let mut last_pos: Vec2 = Vec2::ZERO;

    loop {
        selected = change_selected(selected);

        if pause {
            skip = false;
        }

        if is_key_pressed(KeyCode::Equal) {
            r += 1;
        }

        if is_key_pressed(KeyCode::Minus) {
            r -= 1;
        }

        if is_key_down(KeyCode::Up) {
            if tps < get_fps() as f64 {
                tps += 1.0;
            }
            world.tps = tps;
        }

        if is_key_down(KeyCode::Down) {
            if tps > 0.0 {
                tps -= 1.0;
            }
            world.tps = tps;
        }

        if is_key_pressed(KeyCode::Right) {
            skip = true;
        }

        if is_key_pressed(KeyCode::Space) {
            pause = !pause;
        }
        let (mouse_x, mouse_y) = mouse_position();
        let (mx, my) = (
            ((SCALE * (mouse_x / SCALE).round()) / SCALE) as i32,
            ((SCALE * (mouse_y / SCALE).round()) / SCALE) as i32,
        );

        // let curr_pos = vec2(mouse_x, mouse_y);
        if is_mouse_button_down(MouseButton::Left) {
            for y in my - r..my + r {
                for x in mx - r..mx + r {
                    let i = world.get_idx(x, y);
                    if i > 0 && i < world.length as usize {
                        if world.tick {
                            let mut cc = Controller {
                                x,
                                y,
                                world: &mut world,
                            };

                            //(last_pos - curr_pos) * -0.1
                            cc.set_rel_cell(Cell::new(selected, false, true, Vec2::ZERO), 0, 0);
                        }
                    }
                }
            }
        }
        // last_pos = curr_pos;

        // clear_background(sky_color);

        // if get_time() - last_time > (1.0 / tps) {
        //     world.tick(pause && !skip, 1);
        //     last_time = get_time();
        // }
        world.tick(pause && !skip, 1);

        // world.tick(pause && !skip, 1, tps);

        world.draw();
        world.safe_get_cell(mx, my).draw_info();

        draw_text(
            &format!(
                "selected: {}   \nbrush: {}   paused: {}   tps: {}   fps: {}",
                selected.to_str(),
                r,
                pause,
                tps,
                get_fps()
            ),
            2.0,
            20.0,
            30.0,
            GREEN,
        );

        next_frame().await
    }
}
