use crate::cells::*;
use crate::constants::*;
use crate::controller::*;
use crate::random_utils::*;

impl CT {
    pub fn step(&self, cell: Cell, ctrl: Controller) -> bool {
        // returns if stepped
        return match self {
            CT::Air => false,
            CT::Bedrock => false,
            CT::Sand => falling_solid_step(cell, ctrl),
            CT::Water => liquid_step(cell, ctrl),
            CT::Smoke => gas_step(cell, ctrl),
        };
    }
}

pub fn falling_solid_step(cell: Cell, mut ctrl: Controller) -> bool {
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

    let dir = random_dir();
    let c = ctrl.get_rel_cell(dir, 1);
    if c.identity.is_gas() {
        ctrl.set_rel_cell(c, 0, 0);
        ctrl.set_rel_cell(cell, dir, 1);

        return true;
    };

    ctrl.sleep_cell();

    return false;
}

pub fn liquid_step(cell: Cell, mut ctrl: Controller) -> bool {
    let below = ctrl.get_rel_cell(0, 1);
    if below.identity.is_gas() {
        ctrl.set_rel_cell(below, 0, 0);
        ctrl.set_rel_cell(cell, 0, 1);
        return true;
    };

    let mut water_check = |dir: i32| {
        let c = ctrl.get_rel_cell(dir, 1);
        if c.identity.is_gas() {
            ctrl.set_rel_cell(c, 0, 0);
            ctrl.set_rel_cell(cell, dir, 1);
            return true;
        } else {
            let mut found = 0;
            for i in 1..6 {
                let spot = i * dir;
                if ctrl.get_rel_cell(spot, 0).identity.is_gas() {
                    found = spot;
                    if ctrl.get_rel_cell(spot, 1).identity.is_gas() {
                        break;
                    }
                } else {
                    break;
                }
            }
            if found != 0 {
                let c = ctrl.get_rel_cell(found, 0);
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

    if water_check(d * -1) {
        return true;
    }

    if below.identity.is_solid() {
        ctrl.sleep_cell();
    }

    false
}

pub fn gas_step(cell: Cell, mut ctrl: Controller) -> bool {
    let d = random_dir();
    let v;
    if chance(5) {
        v = 1;
    } else {
        v = -1;
    }

    let c;
    if cell.health > 0 {
        c = Cell {
            health: cell.health - 1,
            ..cell
        };
    } else {
        c = AIR;
    }

    let above_d = ctrl.get_rel_cell(d, v);
    if !above_d.identity.is_solid() && above_d.identity.is_not(&CT::Smoke) {
        ctrl.set_rel_cell(above_d, 0, 0);
        ctrl.set_rel_cell(c, d, v);
        return true;
    }

    let above = ctrl.get_rel_cell(0, v);
    if !above.identity.is_solid() && above.identity.is_not(&CT::Smoke) {
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
