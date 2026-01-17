use core::f32;

use bevy::math::{Vec2, VectorSpace};

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

struct WaterDroplet {
    position: [i32; 2],
    material_gathered: f32,
    velocity: Vec2,
}

impl WaterDroplet {
    fn random(seed: u32) -> WaterDroplet {
        let x = wang_hash(wang_hash(seed));
        let y = wang_hash(x);
        let x = (x % Heightmap::DIM) as i32;
        let y = (y % Heightmap::DIM) as i32;
        WaterDroplet {
            position: [x, y],
            material_gathered: 0.0,
            velocity: Vec2::ZERO,
        }
    }

    fn move_by(&mut self, dir: [i32; 2]) {
        self.position[0] += dir[0];
        self.position[1] += dir[1];

        let dir = Vec2::new(dir[0] as f32, dir[1] as f32);
        if self.velocity == Vec2::ZERO {}
        if dir == Vec2::ZERO {}
        let angle = (self.velocity.dot(dir) / self.velocity.perp_dot(dir)).acos();
    }

    fn x(&self) -> u32 {
        self.position[0] as u32
    }
    fn y(&self) -> u32 {
        self.position[1] as u32
    }
}

pub fn erode(terrain: &mut Heightmap) {
    for i in 0..10000000 {
        let mut droplet = WaterDroplet::random(i);
        for _ in 0..1000 {
            let height = terrain.get(droplet.x(), droplet.y());
            let mut lowest = f32::INFINITY;
            let mut lowest_dir = None;

            for x_o in [-1, 1] {
                if x_o == -1 && droplet.x() == 0 {
                    continue;
                }
                if x_o == 1 && droplet.x() == Heightmap::DIM - 1 {
                    continue;
                }
                for y_o in [-1, 1] {
                    if y_o == -1 && droplet.y() == 0 {
                        continue;
                    }
                    if y_o == 1 && droplet.y() == Heightmap::DIM - 1 {
                        continue;
                    }

                    let other_height = terrain.get(
                        ((droplet.x() as i32) + x_o) as u32,
                        ((droplet.y() as i32) + y_o) as u32,
                    );
                    if other_height < height - 0.001 && other_height < lowest {
                        lowest_dir = Some([x_o, y_o]);
                        lowest = other_height;
                    }
                }
            }
            if let Some(lowest_dir) = lowest_dir {
                terrain.set(droplet.x(), droplet.y(), height - 0.001);
                droplet.move_by(lowest_dir);
            } else {
                break;
            }
        }
    }
}
