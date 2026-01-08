use bevy::{asset::RenderAssetUsages, image::Image, math::Vec3, render::render_resource::Extent3d};

use crate::heightmap::Heightmap;

pub fn create_color(h: &Heightmap) -> Image {
    let mut data = Vec::with_capacity(128 * 128 * 4);
    for y in 0..128 {
        for x in 0..128 {
            let h = h.get(x, y);
            let c = color_gradient(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                h,
            );

            data.push((c.x.clamp(0.0, 1.0) * 255.0) as u8);
            data.push((c.y.clamp(0.0, 1.0) * 255.0) as u8);
            data.push((c.z.clamp(0.0, 1.0) * 255.0) as u8);
            data.push(255);
        }
    }
    Image::new(
        Extent3d {
            width: 128,
            height: 128,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::Rgba8Unorm,
        RenderAssetUsages::all(),
    )
}

fn r_cos(x: Vec3) -> Vec3 {
    Vec3::new(
        x.x.cos() * 0.5 + 0.5,
        x.y.cos() * 0.5 + 0.5,
        x.z.cos() * 0.5 + 0.5,
    )
}

fn color_gradient(offset: Vec3, amplitude: Vec3, frequency: Vec3, x_offset: Vec3, x: f32) -> Vec3 {
    offset + amplitude * r_cos(frequency * x + x_offset)
}
