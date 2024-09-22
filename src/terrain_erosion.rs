use std::ops::ControlFlow;

use rand::seq::SliceRandom;
use rand::{rngs::SmallRng, Rng};

use crate::terrain_gen::FullWorld;

pub fn apply_erosion(world: &mut FullWorld, rng: &mut SmallRng) {
    let (mut min, mut max) = world.range();

    let mut x: isize = rng.gen_range(min..max);
    let mut z: isize = rng.gen_range(min..max);
    let mut y: u8 = 63;

    let mut carried_weigth = 0.0;

    for _ in 0..1000 {
        if let ControlFlow::Break(_) = fun_name(world, x, &mut y, z, &mut carried_weigth) {
            break;
        }
        let options: Vec<_> = [
            (x - 1, z),
            (x + 1, z),
            (x, z - 1),
            (x, z + 1),
            //
            (x - 1, z - 1),
            (x - 1, z + 1),
            (x + 1, z - 1),
            (x + 1, z + 1),
        ]
        .into_iter()
        .map(|(o_x, o_z)| world.get_strength(o_x, y, o_z).map(|v| (*v, (o_x, o_z))))
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .filter(|v| v.0 < 0.001)
        .map(|v| v.1)
        .collect();
        if let Some((new_x, new_z)) = options.choose(rng) {
            x = *new_x;
            z = *new_z;
        } else {
            break;
        }
    }

    if let Some(strength) = world.get_strength(x, y, z) {
        *strength += carried_weigth;
    }
}

fn fun_name(
    world: &mut FullWorld,
    x: isize,
    y: &mut u8,
    z: isize,
    carried_weigth: &mut f32,
) -> ControlFlow<()> {
    if let Some(strength) = world.get_strength(x, *y - 1, z) {
        if *strength > 0.0 {
            let added_carry_strength = *strength;
            *strength -= added_carry_strength;
            *carried_weigth += added_carry_strength;
            // if carried_weigth > 1.0 {
            //     let lost_carried_weight = carried_weigth - 1.0;
            //     carried_weigth -= lost_carried_weight;
            //     *strength += lost_carried_weight;
            // }
            // if carried_weigth > 0.5 {
            //     let lost_carried_weight = rng.gen_range(0.0..carried_weigth);
            //     carried_weigth -= lost_carried_weight;
            //     *strength += lost_carried_weight;
            // }
        } else {
            if *y == 1 {
                return ControlFlow::Break(());
            }
            *y -= 1;
        }
    }
    ControlFlow::Continue(())
}
