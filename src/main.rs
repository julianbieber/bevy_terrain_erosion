mod terrain_erosion;
mod terrain_gen;
mod terrain_mesh;
mod terrain_shader;

use bevy::{color::palettes::css::RED, pbr::ExtendedMaterial, prelude::*};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use indicatif::ProgressBar;
use rand::{rngs::SmallRng, SeedableRng};
use terrain_erosion::apply_erosion;
use terrain_gen::FullWorld;
use terrain_mesh::blocky;
use terrain_shader::TerrainMaterial;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, TerrainMaterial>,
        >::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_mesh_debug)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>>,
) {
    let mut w = FullWorld::new();
    let e = 1000000;
    let pb = ProgressBar::new(e);
    let mut rng = SmallRng::from_seed([0; 32]);
    for _ in pb.wrap_iter(0..e) {
        apply_erosion(&mut w, &mut rng);
    }
    let terrains = w.to_entities();

    for (offset, terrain) in terrains {
        let voxels = terrain.discretize();

        let mesh = blocky(&voxels.visible_faces());

        // cube
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: RED.into(),
                    ..Default::default()
                },
                extension: TerrainMaterial { quantize_steps: 3 },
            }),
            transform: Transform::from_xyz(offset.0, 0.0, offset.1),
            ..default()
        });
    }
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 580.0, 1.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
    // camera

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 164.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

fn move_mesh_debug(
    mut v: Query<&mut Transform, With<Handle<ExtendedMaterial<StandardMaterial, TerrainMaterial>>>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::ArrowLeft) {
        for mut m in v.iter_mut() {
            dbg!("move_left");
            m.translation += Vec3::new(-1.0, 0.0, 0.0);
        }
    }
    if keys.pressed(KeyCode::ArrowRight) {
        for mut m in v.iter_mut() {
            dbg!("move_left");
            m.translation += Vec3::new(1.0, 0.0, 0.0);
        }
    }
    if keys.pressed(KeyCode::ArrowUp) {
        for mut m in v.iter_mut() {
            dbg!("move_left");
            m.translation += Vec3::new(0.0, 1.0, 0.0);
        }
    }
    if keys.pressed(KeyCode::ArrowDown) {
        for mut m in v.iter_mut() {
            dbg!("move_left");
            m.translation += Vec3::new(0.0, -1.0, 0.0);
        }
    }
}
