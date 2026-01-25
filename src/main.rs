use bevy::{
    asset::RenderAssetUsages,
    camera::Exposure,
    color::palettes::css::ALICE_BLUE,
    light::{AtmosphereEnvironmentMapLight, light_consts::lux},
    pbr::{Atmosphere, AtmosphereSettings, ScatteringMedium},
    post_process::bloom::Bloom,
    prelude::*,
    render::render_resource::{Extent3d, TextureViewDescriptor, TextureViewDimension},
};
use bevy_clipmap::{Clipmap, ClipmapPlugin};
use bevy_flycam::prelude::*;

use crate::{
    colormap::create_color,
    erosion::erode,
    heightmap::{Heightmap, create_heightmap},
};

mod colormap;
mod erosion;
mod heightmap;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::KeyE,
            move_descend: KeyCode::KeyQ,
            ..Default::default()
        })
        .insert_resource(ErosionToggle(false))
        .add_plugins(ClipmapPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, move_camera)
        .add_systems(Update, (continue_erosion, toggle_erosion))
        .run()
}

#[derive(Component)]
struct ContinueErosion(Timer);

#[derive(Resource)]
struct ErosionToggle(bool);

fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut scattering_mediums: ResMut<Assets<ScatteringMedium>>,
) {
    let target = commands
        .spawn((
            Camera3d::default(),
            FlyCam,
            Projection::from(PerspectiveProjection {
                fov: 90.0f32.to_radians(),
                ..Default::default()
            }),
            Bloom::default(),
            Atmosphere::earthlike(scattering_mediums.add(ScatteringMedium::default())),
            AtmosphereSettings {
                aerial_view_lut_max_distance: 16384.0,
                ..Default::default()
            },
            AtmosphereEnvironmentMapLight::default(),
            Exposure::SUNLIGHT,
            Transform::from_xyz(0.0, 110.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ))
        .id();

    let mut h = create_heightmap();
    erode(&mut h);
    let c = create_color(&h);

    commands.spawn((
        Clipmap {
            half_width: 256,
            levels: 6,
            base_scale: 1.0,
            texel_size: 8.0,
            target,
            color: images.add(c),
            heightmap: images.add(h.image()),
            horizon: images.add(create_horizon()),
            horizon_coeffs: 1,
            min: -300.5,
            max: 300.5,
            wireframe: false,
        },
        ContinueErosion(Timer::from_seconds(1.0, TimerMode::Repeating)),
        h,
    ));
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            color: ALICE_BLUE.into(),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(100.0, 1000.0, 100.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn toggle_erosion(mut toggle: ResMut<ErosionToggle>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        toggle.0 = !toggle.0;
    }
}

fn continue_erosion(
    mut commands: Commands,
    mut q: Single<(&mut Clipmap, &mut ContinueErosion, &mut Heightmap, Entity)>,
    mut images: ResMut<Assets<Image>>,
    time: Res<Time>,
    toggle: Res<ErosionToggle>,
) {
    q.1.0.tick(time.delta());
    if q.1.0.just_finished() && toggle.0 {
        info!("erosion");
        erode(&mut q.2);
        let new_image = q.2.image();

        commands.entity(q.3).remove::<Clipmap>();
        commands.entity(q.3).insert(Clipmap {
            half_width: 128,
            levels: 7,
            base_scale: 1.0,
            texel_size: 8.0,
            target: q.0.target,
            color: q.0.color.clone(),
            heightmap: images.add(new_image),
            horizon: q.0.horizon.clone(),
            horizon_coeffs: 1,
            min: -300.5,
            max: 300.5,
            wireframe: false,
        });
    }
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
