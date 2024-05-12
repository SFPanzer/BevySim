#[path = "./runtime/camera_controller.rs"]
mod camera_controller;

#[path = "./runtime/rigid_body.rs"]
mod rigid_body;

use bevy::pbr::*;
use bevy::prelude::*;
use camera_controller::{CameraControllerPlugin};
use crate::camera_controller::CameraController;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(Startup, start)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(3., 3., 3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
        },
        CameraController::default(),
    ));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(0, 255, 255)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2400.0,
            ..default()
        },
        transform: Transform::from_xyz(-2., 3., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

