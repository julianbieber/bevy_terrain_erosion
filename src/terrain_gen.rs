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

    pub fn range(&self) -> (isize, isize) {
        ((-4 * 64 + 1), (4 * 64))
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

    // If a value is oob return a strength of 0 as if it is air
    pub fn get_strength(&mut self, x: isize, y: u8, z: isize) -> Option<&mut f32> {
        if x < (-4 * 64) || z < (-4 * 64) {
            return None;
        }
        if x >= 4 * 64 || z >= 4 * 64 {
            return None;
        }
        if y >= 64 {
            return None;
        }
        let which_chunk_x = ((x + 4 * 64) / 64) as usize;
        let which_chunk_z = ((z + 4 * 64) / 64) as usize;

        let which_chunk_index = which_chunk_x + which_chunk_z * 8;

        let inside_chunk_x = ((x + 4 * 64) % 64) as usize;
        let inside_chunk_z = ((z + 4 * 64) % 64) as usize;

        let inside_chunk_index = inside_chunk_x + inside_chunk_z * 64;

        Some(&mut self.chunks[which_chunk_index].strengts[inside_chunk_index][y as usize])
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
                    let strength =
                        ((n.get([scale_xz(r_x) + 10.0, scale_xz(y as isize), scale_xz(r_z)]))
                            as f32
                            + 0.1)
                            .max(1.0);

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
                if *y > 0.1 {
                    v |= 1u64 << (i_y);
                }
            }
            s.set_raw(i, v);
        }
        s
    }
}

fn scale_xz(v: isize) -> f64 {
    v as f64 / 64.0
}
