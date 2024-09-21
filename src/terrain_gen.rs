use noise::Fbm;
use noise::NoiseFn;
use noise::PerlinSurflet;

use crate::terrain_mesh::VoxelStorage;
pub struct Terrain {
    /// 2 dimensions in the first vector, 1d in the array, transformed into u64 for voxel rendering 1bit per f32 depending on threshold
    strengts: Vec<[f32; 64]>,
}

impl Terrain {
    pub fn gen() -> Terrain {
        let n = Fbm::<PerlinSurflet>::new(0);
        let mut t = Terrain {
            strengts: vec![[0.0; 64]; 64 * 64],
        };
        for x in 0..64 {
            for z in 0..64 {
                let height = (((n.get([scale_xz(x), scale_xz(z)]) + 1.0) / 2.0) * 64.0) as usize;
                for y in 0..height {
                    let strength =
                        ((n.get([scale_xz(x), scale_xz(y), scale_xz(z)]) + 1.0) / 4.0) as f32 + 0.5;

                    t.strengts[x + z * 64][y] = strength;
                }
            }
        }
        t
    }
    pub fn discretize(&self) -> VoxelStorage {
        let mut s = VoxelStorage::empty();
        for (i, ys) in self.strengts.iter().enumerate() {
            let mut v = 0u64;
            for (i_y, y) in ys.iter().enumerate() {
                if *y > 0.5 {
                    v |= 1u64 << (i_y);
                }
            }
            s.set_raw(i, v);
        }
        s
    }
}

fn scale_xz(v: usize) -> f64 {
    v as f64 / 32.0
}
