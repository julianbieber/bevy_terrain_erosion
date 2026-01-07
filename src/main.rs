use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureViewDescriptor, TextureViewDimension},
};
use bevy_clipmap::{Clipmap, ClipmapPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ClipmapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, move_camera)
        .run()
}

fn startup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let target = commands.spawn(Camera3d::default()).id();

    commands.spawn(Clipmap {
        half_width: 128,
        levels: 7,
        base_scale: 1.0,
        texel_size: 8.0,
        target,
        color: images.add(create_color()),
        heightmap: images.add(create_heightmap()),
        horizon: images.add(create_horizon()),
        horizon_coeffs: 1,
        min: -1312.5,
        max: 1312.5,
        wireframe: false,
    });
}

fn create_color() -> Image {
    let mut data = Vec::with_capacity(128 * 128 * 4);
    for _ in 0..128 {
        for _ in 0..128 {
            data.push(255);
            data.push(0);
            data.push(0);
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

fn create_heightmap() -> Image {
    let mut data = Vec::with_capacity(128 * 128 * 4);
    for x in 0..128 {
        for y in 0..128 {
            let v = (x * y) as f32;
            data.extend(((v * 0.01).sin().fract() * 1.0f32).to_le_bytes());
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
        bevy::render::render_resource::TextureFormat::R32Float,
        RenderAssetUsages::all(),
    )
}

fn create_horizon() -> Image {
    let mut data = Vec::with_capacity(128 * 128 * 4);
    for _ in 0..128 {
        for _ in 0..128 {
            data.extend(0.0f32.to_le_bytes());
        }
    }
    let mut i = Image::new(
        Extent3d {
            width: 128,
            height: 128,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        data,
        bevy::render::render_resource::TextureFormat::R32Float,
        RenderAssetUsages::all(),
    );

    i.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::D2Array),
        ..Default::default()
    });

    i
}

fn move_camera(mut camera: Single<&mut Transform, With<Camera>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::KeyW) {
        camera.translation.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        camera.translation.z -= 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        camera.translation.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        camera.translation.x -= 1.0;
    }

    if keys.pressed(KeyCode::KeyE) {
        camera.translation.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyC) {
        camera.translation.y -= 1.0;
    }
}
