#[path = "./runtime/camera_controller.rs"]
mod camera_controller;

use bevy::pbr::*;
use bevy::prelude::*;

use camera_controller::*;
use puma_physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_plugins(PumaPhysicsPlugin)
        .add_systems(Startup, start)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle{
            transform: Transform::from_xyz(5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default()
    ));
    commands.spawn(TextBundle{
        text: Text::from_section(
            "Operation Instructions:\n\nW/A/S/D: Move around.\nSpace/Shift: Move up and down.\nEnter: Reset rigidbody back to zero.\nMouse: Drag right botton to look around.",
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            }),
        ..default()
    });
    commands.spawn(TextBundle{
        text: Text::from_section(
            "Tips:\n\nYellow: Joints.\nCyan: Visualizing all the individual forces.\nFuchsia: Visualizing the resultant force.\n\nArrows represent forces\nStraight lines represent torque.",
            TextStyle {
                font_size: 16.0,
                color: Color::WHITE,
                ..default()
            }),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()},    
        ..default()
    });
}
