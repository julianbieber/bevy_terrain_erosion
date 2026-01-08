use bevy::{asset::RenderAssetUsages, image::Image, math::Vec2, render::render_resource::Extent3d};
use noiz::{
    Noise, SampleableFor, ScalableNoise, SeedableNoise,
    cells::OrthoGrid,
    curves::Smoothstep,
    prelude::{
        FractalLayers, LayeredNoise, MixCellGradients, Normed, Octave, Persistence, QuickGradients,
    },
};

pub fn create_heightmap() -> Heightmap {
    let mut m = Heightmap::zero();

    let mut noise = Noise::from(LayeredNoise::new(
        Normed::<f32>::default(),
        Persistence(0.9),
        FractalLayers {
            // The layer to do fbm with
            layer: Octave::<MixCellGradients<OrthoGrid, Smoothstep, QuickGradients>>::default(),
            // How much to change the scale by between each repetition of the layer
            lacunarity: 2.4,
            // How many repetitions to do
            amount: 30,
        },
    ));
    noise.set_frequency(0.000000001);
    noise.set_seed(42);
    for y in 0..Heightmap::DIM {
        for x in 0..Heightmap::DIM {
            let v = Vec2::new(x as f32 * 0.3, y as f32 * 0.3);
            let h: f32 = noise.sample(v);
            m.set(x, y, h);
        }
    }
    m
}

pub struct Heightmap {
    values: Vec<f32>,
}

impl Heightmap {
    pub const DIM: u32 = 128 * 16;
    fn zero() -> Heightmap {
        Heightmap {
            values: vec![0.0; (Self::DIM * Self::DIM) as usize],
        }
    }
    pub fn image(&self) -> Image {
        let data: Vec<u8> = self.values.iter().flat_map(|v| v.to_le_bytes()).collect();
        Image::new(
            Extent3d {
                width: Self::DIM,
                height: Self::DIM,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            data,
            bevy::render::render_resource::TextureFormat::R32Float,
            RenderAssetUsages::all(),
        )
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        assert!(x < Self::DIM);
        assert!(y < Self::DIM);

        let index = x as usize * (Self::DIM as usize) + y as usize;

        assert!(index < (Self::DIM * Self::DIM) as usize);

        self.values[index]
    }

    pub fn set(&mut self, x: u32, y: u32, h: f32) {
        assert!(x < Self::DIM);
        assert!(y < Self::DIM);

        let index = x as usize * (Self::DIM as usize) + y as usize;

        assert!(index < (Self::DIM * Self::DIM) as usize);

        self.values[index] = h;
    }
}
