pub mod behaviors;
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
    }

    selected
}

#[macroquad::main(window_conf)]
async fn main() {
    let (sw, sh) = (screen_width() / SCALE, screen_height() / SCALE);

    let mut cells = Vec::new();

    let length = (sw * sh) as i32;
    (0..length).for_each(|_| {
        cells.push(AIR);
    });

    let mut world = World {
        cells,
        width: sw as i32,
        height: sh as i32,
        tick: true,
    };

    // brush radius
    let r = 3;

    let mut selected = CT::Sand;

    loop {
        selected = change_selected(selected);
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let (mx, my) = (
                ((SCALE * (mouse_x / SCALE).round()) / SCALE) as i32,
                ((SCALE * (mouse_y / SCALE).round()) / SCALE) as i32,
            );
            for y in my - r..my + r {
                for x in mx - r..mx + r {
                    let i = world.get_idx(x, y);
                    if i > 0 && i < length as usize {
                        if world.tick {
                            let mut cc = Controller {
                                x,
                                y,
                                world: &mut world,
                            };

                            cc.set_rel_cell(Cell::new(selected, false, true), 0, 0);
                        }
                    }
                }
            }
        }

        clear_background(SKY);

        world.tick();
        draw_text(
            &format!("selected: {}      fps: {}", selected.to_str(), get_fps()),
            2.0,
            20.0,
            30.0,
            GREEN,
        );

        next_frame().await
    }
}
