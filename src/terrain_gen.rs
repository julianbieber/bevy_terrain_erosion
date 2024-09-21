use noise::Fbm;
use noise::NoiseFn;
use noise::PerlinSurflet;

use crate::terrain_mesh::VoxelStorage;

pub struct FullWorld {
    chunks: Vec<Terrain>,
}

impl FullWorld {
    pub fn new() -> FullWorld {
        let mut chunks = Vec::new();
        for x in -4..4 {
            for z in -4..4 {
                let c = Terrain::gen(x * 64, z * 64);
                chunks.push(c);
            }
        }
        FullWorld { chunks }
    }

    pub fn to_entities<'a>(&'a self) -> Vec<((f32, f32), &'a Terrain)> {
        self.chunks
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let z = ((i % 8) * 64) as isize - 64 * 4;
                let x = ((i / 8) * 64) as isize - 64 * 4;
                ((x as f32, z as f32), c)
            })
            .collect()
    }
}

pub struct Terrain {
    /// 2 dimensions in the first vector, 1d in the array, transformed into u64 for voxel rendering 1bit per f32 depending on threshold
    strengts: Vec<[f32; 64]>,
}

impl Terrain {
    pub fn gen(x_offset: isize, z_offset: isize) -> Terrain {
        let n = Fbm::<PerlinSurflet>::new(0);
        let mut t = Terrain {
            strengts: vec![[0.0; 64]; 64 * 64],
        };
        for x in 0..64usize {
            for z in 0..64usize {
                let r_x = x as isize + x_offset;
                let r_z = z as isize + z_offset;
                let height =
                    (((n.get([scale_xz(r_x), scale_xz(r_z)]) + 1.0) / 2.0) * 64.0) as usize;
                for y in 0..height {
                    let strength = ((n.get([scale_xz(r_x), scale_xz(y as isize), scale_xz(r_z)])
                        + 1.0)
                        / 4.0) as f32
                        + 0.5;

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

fn scale_xz(v: isize) -> f64 {
    v as f64 / 32.0
}
