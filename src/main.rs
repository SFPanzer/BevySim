#[path = "./runtime/camera_controller.rs"]
mod camera_controller;

use rand::Rng;
use bevy::pbr::*;
use bevy::prelude::*;

use camera_controller::*;
use physics::distance_joint::*;
use physics::rigidbody::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_plugins(RigidBodyPlugin)
        .add_plugins(DistanceJointPlugin)
        .add_systems(Startup, start)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rnd = rand::thread_rng();
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2400.0,
            ..default()
        },
        transform: Transform::from_xyz(-2., 3., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

    // Spawn rigid chain.
    let mut rigid0 = Entity::PLACEHOLDER;
    let mut rigid1 = commands
        .spawn(CuboidRigidBodyBoundle::new(
            &mut meshes,
            materials.add(Color::rgb_u8(0, 255, 255)),
            Vec3::new(rnd.gen_range(1.0..=2.0), rnd.gen_range(1.0..=2.0), rnd.gen_range(1.0..=2.0)),
            1.0,
        ))
        .id();
    commands.spawn(DistanceJoint {
        actor0: None,
        actor1: Some(rigid1),
        local_frame1: Vec3::new(rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0)),
        ..default()
    });
    
    for _ in 0..2 {
        rigid0 = rigid1;
        rigid1 = commands
            .spawn(CuboidRigidBodyBoundle::new(
                &mut meshes,
                materials.add(Color::rgb_u8(0, 255, 255)),
                Vec3::new(rnd.gen_range(1.0..=2.0), rnd.gen_range(1.0..=2.0), rnd.gen_range(1.0..=2.0)),
                1.0,
            ))
            .id();
        commands.spawn(DistanceJoint {
            actor0: Some(rigid0),
            actor1: Some(rigid1),
            local_frame0: Vec3::new(rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0)),
            local_frame1: Vec3::new(rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0), rnd.gen_range(-2.0..=2.0)),
            ..default()
        });
    }
}
