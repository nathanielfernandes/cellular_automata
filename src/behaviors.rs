use crate::cells::*;
use crate::constants::*;
use crate::controller::*;
use crate::random_utils::*;
use macroquad::prelude::*;

// impl CT {
// pub fn step(&self, cell: Cell, ctrl: Controller) -> bool {
//     // returns if stepped
//     return match self {
//         CT::Sand => falling_solid_step(cell, ctrl),
//         CT::GunPowder => falling_solid_step(cell, ctrl),
//         CT::Water => liquid_step(cell, ctrl),
//         CT::Wood => solid_step(cell, ctrl),
//         CT::Smoke => gas_step(cell, ctrl),
//         CT::Fire => fire_step(cell, ctrl),
//         CT::Oil => liquid_step(cell, ctrl),
//         CT::Lava => liquid_step(cell, ctrl),
//         CT::Air => solid_step(cell, ctrl),
//         // CT::Air => life_step(cell, ctrl), // only leave on if life
//         _ => false,
//     };
// }
// }

pub fn apply_heat(cell: Cell, mut ctrl: Controller) -> Controller {
    let rate = (1.0 / ctrl.world.tps) as f32;
    let mut heat = |x: i32, y: i32| {
        let og_cell = ctrl.get_rel_cell(0, 0);
        let c = ctrl.get_rel_cell(x, y);
        //c.identity != cell.identity &&
        if og_cell.heat > c.heat && c.conductivity > 0.0 && !c.identity.is_static() {
            let cond = cell.conductivity * c.conductivity;

            // if og_cell.identity != c.identity {
            //     cell.conductivity * c.conductivity
            // } else {
            //     c.conductivity
            // };

            let transfered = (og_cell.heat * cond * rate) / 8.0;

            // c.heat += transfered;
            // c.active = true;
            ctrl.set_rel_cell_no_tick(
                Cell {
                    heat: og_cell.heat - transfered,
                    ..og_cell
                },
                0,
                0,
            );
            ctrl.set_rel_cell_no_tick(
                Cell {
                    heat: c.heat + transfered,
                    ..c
                },
                x,
                y,
            )
        }
    };

    // if cell.identity.applies_heat() {
    heat(0, -1);
    heat(0, 1);
    heat(-1, 0);
    heat(1, 0);

    heat(-1, -1);
    heat(-1, 1);
    heat(1, -1);
    heat(1, 1);
    // }

    ctrl
}

pub fn solid_step(cell: Cell, mut ctrl: Controller) -> bool {
    // ctrl = apply_heat(cell, ctrl);

    if cell.identity == CT::Air {
        let mut cell = ctrl.get_rel_cell(0, 0);

        if cell.heat > 28.0 {
            cell.heat -= 0.05;
        } else {
            cell.heat += 0.05;
        }

        let above = ctrl.get_rel_cell(0, -1);

        if above.identity == CT::Air && above.heat < cell.heat {
            ctrl.set_rel_cell(cell, 0, -1);
            ctrl.set_rel_cell(above, 0, 0);
            return true;
        }

        let dir = random_dir();
        let side = ctrl.get_rel_cell(dir, 0);

        if side.identity == CT::Air && side.heat < cell.heat {
            ctrl.set_rel_cell(cell, dir, 0);
            ctrl.set_rel_cell(side, 0, 0);
            return true;
        }
    }

    return false;
}

pub fn falling_solid_step(cell: Cell, mut ctrl: Controller) -> bool {
    // ctrl = apply_heat(cell, ctrl);
    // let cell = gravity(cell);
    // ctrl.set_rel_cell(cell, 0, 0);

    // let mut ctrl = velocity_step(cell, ctrl);

    let below = ctrl.get_rel_cell(0, 1);
    if below.identity.is_gas() {
        ctrl.set_rel_cell(below, 0, 0);
        ctrl.set_rel_cell(cell, 0, 1);
        return true;
    };

    if below.identity.is_liquid() {
        let dir = random_dir();
        let c = ctrl.get_rel_cell(dir, 0);
        if c.identity.is_gas() {
            ctrl.set_rel_cell(c, 0, 0);
            ctrl.set_rel_cell(below, dir, 0);
            ctrl.set_rel_cell(cell, 0, 1);
        } else {
            ctrl.set_rel_cell(below, 0, 0);
            ctrl.set_rel_cell(cell, 0, 1);
        }

        return true;
    }

    if inertial_res_check(cell.inertial_res) {
        let dir = random_dir();
        let c = ctrl.get_rel_cell(dir, 1);
        if c.identity.is_gas() {
            ctrl.set_rel_cell(c, 0, 0);
            ctrl.set_rel_cell(cell, dir, 1);

            return true;
        };
    }

    ctrl.sleep_cell();

    return false;
}

// pub fn lava_step(cell: Cell, mut ctrl: Controller) -> bool {
//     liquid_step(cell, ctrl)
// }

pub fn liquid_step(cell: Cell, mut ctrl: Controller) -> bool {
    // ctrl = apply_heat(cell, ctrl);
    // let cell = ctrl.get_rel_cell(0, 0);

    let mut wet = |x: i32, y: i32| {
        let to_wet = ctrl.get_rel_cell(x, y);
        if !to_wet.is_wet && to_wet.identity.is_solid() {
            let (color, flammability, health) =
                cell.identity
                    .wet(to_wet.color, to_wet.flammability, to_wet.health);
            ctrl.set_rel_cell(
                Cell {
                    color,
                    flammability,
                    health,
                    is_wet: true,
                    ..to_wet
                },
                x,
                y,
            )
        }
    };

    wet(0, -1);
    wet(0, 1);
    wet(-1, 0);
    wet(1, 0);

    // let mut ctrl = velocity_step(cell, ctrl);

    let below = ctrl.get_rel_cell(0, 1);
    if !below.identity.is_solid() && below.mass < cell.mass {
        ctrl.set_rel_cell(below, 0, 0);
        ctrl.set_rel_cell(cell, 0, 1);
        return true;
    };

    let mut water_check = |dir: i32| {
        let c = ctrl.get_rel_cell(dir, 1);
        if !c.identity.is_solid() && c.mass < cell.mass {
            ctrl.set_rel_cell(c, 0, 0);
            ctrl.set_rel_cell(cell, dir, 1);
            return true;
        } else {
            let mut found = 0;
            for i in 1..(cell.viscosity as i32) {
                let spot = i * dir;

                let cc = ctrl.get_rel_cell(spot, 0);
                if (cc.identity.is_liquid() || cc.identity == CT::Air) && cc.mass <= cell.mass {
                    found = spot;
                    let c1 = ctrl.get_rel_cell(spot, 1);
                    if !c1.identity.is_solid() && c1.mass < cell.mass {
                        break;
                    }
                } else {
                    break;
                }
            }
            if found != 0 {
                let c = ctrl.get_rel_cell(found, 0);
                if c.identity == cell.identity {
                    return true;
                }
                ctrl.set_rel_cell(c, 0, 0);
                ctrl.set_rel_cell(cell, found, 0);
                return true;
            }
        }
        false
    };

    let d = random_dir();
    if water_check(d) {
        return true;
    }

    if water_check(-d) {
        return true;
    }

    // if below.identity.is_solid() {
    ctrl.sleep_cell();
    // }

    false
}

pub fn gas_step(cell: Cell, mut ctrl: Controller) -> bool {
    // ctrl = apply_heat(cell, ctrl);
    // let cell = ctrl.get_rel_cell(0, 0);

    let d = random_dir();
    let v;
    if vorticity_check(cell.vorticity) {
        v = 1;
    } else {
        v = -1;
    }

    let c;
    if cell.health > 0.0 {
        c = Cell {
            health: cell.health - 1.0,
            ..cell
        };
    } else {
        c = Cell {
            heat: cell.heat,
            ..AIR
        };
    }

    let above_d = ctrl.get_rel_cell(d, v);
    if !above_d.identity.is_solid()
        && above_d.identity.is_gas()
        && above_d.identity != cell.identity
    {
        ctrl.set_rel_cell(above_d, 0, 0);
        ctrl.set_rel_cell(c, d, v);
        return true;
    }

    let above = ctrl.get_rel_cell(0, v);
    if !above.identity.is_solid() && above_d.identity.is_gas() && above.identity != cell.identity {
        ctrl.set_rel_cell(above, 0, 0);
        ctrl.set_rel_cell(c, 0, v);
        return true;
    }

    let d = random_dir();
    let side = ctrl.get_rel_cell(d, 0);
    if !side.identity.is_solid() {
        ctrl.set_rel_cell(side, 0, 0);
        ctrl.set_rel_cell(c, d, 0);
        return true;
    }

    return false;
}

pub fn fire_step(cell: Cell, mut ctrl: Controller) -> bool {
    // ctrl = apply_heat(cell, ctrl);
    // let cell = ctrl.get_rel_cell(0, 0);
    let d = random_dir();
    let v;
    if vorticity_check(cell.vorticity) {
        v = 1;
    } else {
        v = -1;
    }

    let c;
    if cell.health > 0.0 {
        c = Cell {
            health: cell.health - 1.0,
            ..cell
        };
    } else {
        c = Cell {
            identity: CT::Smoke,
            health: 100.0,
            variation: CT::Smoke.get_varation(),
            tick: cell.tick,
            heat: 80.0,
            ..Cell::default(CT::Smoke)
        };
    }

    let mut spread = |x: i32, y: i32| {
        let on_fire = ctrl.get_rel_cell(x, y);
        if on_fire.identity != CT::Smoke && flammability_check(on_fire.flammability) {
            if on_fire.health > 0.0 {
                ctrl.set_rel_cell_no_tick(
                    Cell {
                        health: on_fire.health - 1.0,
                        color: Cell::burn(on_fire.color),
                        ..on_fire
                    },
                    x,
                    y,
                );
                ctrl.set_rel_cell_no_tick(
                    Cell {
                        variation: CT::Fire.get_varation(),
                        health: CT::Fire.get_health(),
                        ..cell
                    },
                    0,
                    0,
                );
                return true;
            } else {
                ctrl.set_rel_cell_no_tick(
                    Cell {
                        variation: CT::Fire.get_varation(),
                        health: CT::Fire.get_health(),
                        ..cell
                    },
                    x,
                    y,
                );
                return false;
            }
        }
        return false;
    };

    let mut is_spreading = false;
    if spread(0, -1) {
        is_spreading = true;
    }

    if spread(0, 1) {
        is_spreading = true;
    }

    if spread(-1, 0) {
        is_spreading = true;
    }

    if spread(1, 0) {
        is_spreading = true
    }

    let a = ctrl.get_rel_cell(0, -1);

    if is_spreading && a.identity == CT::Air {
        ctrl.set_rel_cell(cell, 0, -1);
    }

    let a1 = ctrl.get_rel_cell(0, 1);

    if is_spreading && a1.identity == CT::Air {
        ctrl.set_rel_cell(cell, 0, 1);
    }

    if is_spreading {
        return false;
    }

    if !a.identity.is_immovable() && a.flammability == 0.0 {
        ctrl.set_rel_cell(
            Cell {
                identity: CT::Smoke,
                health: 100.0,
                variation: CT::Smoke.get_varation(),
                tick: cell.tick,
                heat: 80.0,
                ..Cell::default(CT::Smoke)
            },
            0,
            0,
        );
        return true;
    }

    if !a1.identity.is_immovable() && a1.flammability == 0.0 {
        ctrl.set_rel_cell(
            Cell {
                identity: CT::Smoke,
                health: 100.0,
                variation: CT::Smoke.get_varation(),
                tick: cell.tick,
                heat: 80.0,
                ..Cell::default(CT::Smoke)
            },
            0,
            0,
        );
        return true;
    }

    let above_d = ctrl.get_rel_cell(d, v);
    if !above_d.identity.is_solid() && above_d.identity.is_gas() && above_d.identity != CT::Smoke {
        ctrl.set_rel_cell(above_d, 0, 0);
        ctrl.set_rel_cell(c, d, v);
        return true;
    }

    let above = ctrl.get_rel_cell(0, v);
    if !above.identity.is_solid() && above.identity.is_gas() && above.identity != CT::Smoke {
        ctrl.set_rel_cell(above, 0, 0);
        ctrl.set_rel_cell(c, 0, v);
        return true;
    }

    let d = random_dir();
    let side = ctrl.get_rel_cell(d, 0);
    if !side.identity.is_solid() {
        ctrl.set_rel_cell(side, 0, 0);
        ctrl.set_rel_cell(c, d, 0);
        return true;
    }

    return false;
}

pub fn life_step(cell: Cell, mut ctrl: Controller) -> bool {
    let mut neighbours = 0;

    if ctrl.get_rel_cell(0, -1).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(0, 1).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(-1, 0).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(1, 0).identity == CT::Life {
        neighbours += 1;
    }

    if ctrl.get_rel_cell(-1, -1).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(1, -1).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(-1, 1).identity == CT::Life {
        neighbours += 1;
    }
    if ctrl.get_rel_cell(1, 1).identity == CT::Life {
        neighbours += 1;
    }

    if cell.identity == CT::Life {
        // Rule 1: Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
        if neighbours < 2 {
            ctrl.set_rel_cell_no_tick(AIR, 0, 0);
            return true;
        }

        // Rule 2: Any live cell with two or three live neighbours lives on to the next generation.
        if neighbours == 2 || neighbours == 3 {
            return true;
        }

        // Rule 3: Any live cell with more than three live neighbours dies, as if by overpopulation.
        if neighbours > 3 {
            ctrl.set_rel_cell_no_tick(AIR, 0, 0);
            return true;
        }
    }
    //  else {
    //     // Rule 4: Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    //     if neighbours == 3 {
    //         ctrl.set_rel_cell_no_tick(LIFE, 0, 0);
    //         return true;
    //     }
    // }

    // All other cells remain in the same state.
    ctrl.set_rel_cell_no_tick(AIR, 0, 0);
    false
}

// if transfer_to != (0, 0) {
//     let t = ctrl.get_rel_cell(transfer_to.0, transfer_to.1);

//     let newts = mass_thingy(cell.mass, t.mass);

//     ctrl.set_rel_cell(
//         Cell {
//             velocity: t.velocity + (cell.velocity / newts),
//             ..t
//         },
//         transfer_to.0,
//         transfer_to.1,
//     );
//     ctrl.set_rel_cell(
//         Cell {
//             velocity: cell.velocity / newts,
//             ..cell
//         },
//         0,
//         0,
//     );
// }
// if hit {
//     ctrl.set_rel_cell(
//         Cell {
//             // velocity: if hit { Vec2::ZERO } else { cell.velocity },
//             ..cell
//         },
//         0,
//         0,
//     );
// }

const GRAVITY: Vec2 = const_vec2!([0.0, 9.8]);

pub fn mass_thingy(mass_1: f32, mass_2: f32) -> f32 {
    mass_1 / mass_2
}

pub fn velocity_step(cell: Cell, mut ctrl: Controller) -> Controller {
    if cell.velocity.y < cell.terminal_v && !ctrl.get_rel_cell(0, 1).identity.is_solid() {
        let tps = ctrl.world.tps;
        let g = (1.0 / tps) as f32 * GRAVITY;
        ctrl.set_rel_cell(
            Cell {
                velocity: cell.velocity + g,
                ..cell
            },
            0,
            0,
        );
    }

    let neg_x = if cell.velocity.x < 0.0 { -1 } else { 1 };
    let neg_y = if cell.velocity.y < 0.0 { -1 } else { 1 };
    let (vx, vy) = (
        cell.velocity.x.abs().round() as i32,
        cell.velocity.y.abs().round() as i32,
    );

    let mut viable = (0, 0);
    // let mut hit = false;

    for y in 0..vy + 1 {
        for x in 0..vx + 1 {
            let (dx, dy) = (x * neg_x, y * neg_y);
            let c = ctrl.get_rel_cell(dx, dy);

            if c.identity.is_gas() {
                viable = (dx, dy)
            } else {
                // hit = true;
                break;
            }
        }
    }

    // if hit {
    //     let mut new_vel = cell.velocity;
    //     let mut tx = ctrl.get_rel_cell(viable.0 + neg_x, viable.1);
    //     if tx.identity.is_solid() {
    //         new_vel.x /= 2.0;
    //         tx.velocity.x = new_vel.x;
    //         ctrl.set_rel_cell(tx, viable.0 + neg_x, viable.1);
    //     }

    //     let mut ty = ctrl.get_rel_cell(viable.0, viable.1 + neg_y);
    //     if ty.identity.is_solid() {
    //         new_vel.y /= 2.0;
    //         ty.velocity.y = new_vel.y;
    //         ctrl.set_rel_cell(ty, viable.0, viable.1 + neg_y);
    //     }

    //     // ctrl.set_rel_cell(
    //     //     Cell {
    //     //         velocity: new_vel,
    //     //         ..cell
    //     //     },
    //     //     0,
    //     //     0,
    //     // );
    // }

    if viable != (0, 0) {
        ctrl.swap_rel_cells(viable.0, viable.1);
        ctrl.x += viable.0;
        ctrl.y += viable.1;
    }

    ctrl
}

pub fn gravity(cell: Cell) -> Cell {
    if cell.velocity.y < cell.terminal_v {
        return Cell {
            velocity: cell.velocity + GRAVITY,
            ..cell
        };
    } else {
        return cell;
    }
}
