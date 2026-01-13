use core::f32;

use crate::heightmap::Heightmap;

#[inline]
pub fn wang_hash(mut x: u32) -> u32 {
    x = (x ^ 61) ^ (x >> 16);
    x = x.wrapping_add(x << 3);
    x ^= x >> 4;
    x = x.wrapping_mul(0x27d4_eb2d);
    x ^= x >> 15;
    x
}

pub fn erode(terrain: &mut Heightmap) {
    for i in 0..10000000 {
        let x = wang_hash(wang_hash(i));
        let y = wang_hash(x);
        let mut x = (x % Heightmap::DIM) as i32;
        let mut y = (y % Heightmap::DIM) as i32;
        for _ in 0..1000 {
            let height = terrain.get(x as u32, y as u32);
            let mut lowest = f32::INFINITY;
            let mut lowest_dir = None;

            for x_o in [-1, 1] {
                if x_o == -1 && x == 0 {
                    continue;
                }
                if x_o == 1 && x as u32 == Heightmap::DIM - 1 {
                    continue;
                }
                for y_o in [-1, 1] {
                    if y_o == -1 && y == 0 {
                        continue;
                    }
                    if y_o == 1 && y as u32 == Heightmap::DIM - 1 {
                        continue;
                    }

                    let other_height =
                        terrain.get(((x as i32) + x_o) as u32, ((y as i32) + y_o) as u32);
                    if other_height < height - 0.001 && other_height < lowest {
                        lowest_dir = Some([x_o, y_o]);
                        lowest = other_height;
                    }
                }
            }
            if let Some(lowest_dir) = lowest_dir {
                terrain.set(x as u32, y as u32, height - 0.001);
                x += lowest_dir[0];
                y += lowest_dir[1];
            } else {
                break;
            }
        }
    }
}
