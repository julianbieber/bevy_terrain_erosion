use core::f32;

use bevy::math::Vec2;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};

use crate::heightmap::Heightmap;

struct WaterDroplet {
    position: [i32; 2],
    material_gathered: f32,
    velocity: Vec2,
}

impl WaterDroplet {
    fn new(position: [i32; 2]) -> WaterDroplet {
        WaterDroplet {
            position,
            material_gathered: 0.0,
            velocity: Vec2::ZERO,
        }
    }

    fn capacity(&self) -> f32 {
        if self.velocity == Vec2::ZERO {
            return 1.0;
        }
        self.velocity.length_recip().clamp(0.0, 0.12)
    }

    /// returns the amount of material to add/remove from the terrain
    fn move_by(&mut self, dir: [i32; 2], height_diff: f32) -> f32 {
        self.position[0] += dir[0];
        self.position[1] += dir[1];

        let dir = Vec2::new(dir[0] as f32, dir[1] as f32);
        let angle_scale = angle_between(self.velocity, dir);
        if angle_scale > 0.5 {
            self.velocity += dir;
        }
        if angle_scale <= 0.5 {
            self.velocity = dir;
        }
        if height_diff < 0.0 {
            self.velocity *= 1.1;
        } else {
            self.velocity *= 0.9;
        }

        if angle_scale > 0.5 {
            let amount = self.velocity.length();
            if self.material_gathered < self.capacity() {
                let offset = self.capacity() * amount;
                self.material_gathered += offset;
                -offset
            } else {
                let drop = self.material_gathered * 0.1;
                self.material_gathered -= drop;
                drop
            }
        } else {
            let r = self.material_gathered;
            self.material_gathered = 0.0;
            r
        }
    }

    fn x(&self) -> u32 {
        self.position[0] as u32
    }
    fn y(&self) -> u32 {
        self.position[1] as u32
    }
}

fn angle_between(main: Vec2, dir: Vec2) -> f32 {
    if main == Vec2::ZERO {
        return 1.0;
    }
    if dir == Vec2::ZERO {
        return 1.0;
    }
    let main = main.normalize();
    let dir = dir.normalize();
    let angle = main.dot(dir).acos();
    let angle_scale = (f32::consts::PI - angle) / f32::consts::PI;
    angle_scale
}

pub fn erode(terrain: &mut Heightmap) {
    let mut rng = rand::rng();
    for _ in 0..100 {
        let position = sample_pos_weighted(
            &mut rng,
            &terrain.values,
            Heightmap::DIM as i32,
            Heightmap::DIM as i32,
        );
        let mut droplet = WaterDroplet::new(position);
        for _ in 0..1000 {
            let x = droplet.x();
            let y = droplet.y();
            let height = terrain.get(x, y);
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
                let height_change = droplet.move_by(lowest_dir, height - lowest);
                assert!(!height.is_infinite());
                assert!(!height_change.is_nan());
                terrain.set(x, y, height + height_change);
            } else {
                terrain.set(droplet.x(), droplet.y(), height + droplet.material_gathered);
                if droplet.velocity.length_squared() < 0.01 {
                    break;
                }
            }
        }
    }
}

pub fn sample_pos_weighted(
    rng: &mut impl Rng,
    heights: &[f32], // len = width * height
    width: i32,
    height: i32,
) -> [i32; 2] {
    assert_eq!(heights.len(), (width * height) as usize);

    // Turn heights into strictly positive weights.
    // (If heights can be 0/negative, shift + clamp.)
    let min_h = heights.iter().copied().fold(f32::INFINITY, f32::min);

    let weights: Vec<f32> = heights
        .iter()
        .map(|&h| (h - min_h + 1e-6)) // ensures > 0
        .collect();

    let dist =
        WeightedIndex::new(&weights).expect("all weights must be non-negative, not all zero");
    let idx = dist.sample(rng);

    let x = (idx as i32) % width;
    let y = (idx as i32) / width;
    [x, y]
}

#[cfg(test)]
mod test {
    use bevy::math::Vec2;

    use crate::erosion::angle_between;

    #[test]
    fn test_angle_between() {
        dbg!(angle_between(Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)));
        dbg!(angle_between(Vec2::new(0.0, 1.0), Vec2::new(0.5, 0.5)));
        dbg!(angle_between(Vec2::new(0.0, 1.0), Vec2::new(0.0, 1.0)));
        dbg!(angle_between(Vec2::new(0.0, 1.0), Vec2::new(0.0, -1.0)));

        assert!(false);
    }
}
