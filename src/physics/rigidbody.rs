use bevy::prelude::*;

pub struct RigidBodyPlugin;

pub const GRAVITY_ACCELERATION: Vec3 = Vec3::new(0.0, -9.8, 0.0);

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigidbody);
        app.add_systems(Update, reset);
    }
}

#[derive(Component, Debug)]
pub struct Rigidbody {
    pub mass: f32,
    pub inertia: Mat3,
    pub force: Vec3,
    pub torque: Vec3,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub drag: f32,
    pub angular_drag: f32,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            mass: 1.0,
            inertia: Mat3::IDENTITY,
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            drag: 0.1,
            angular_drag: 0.1,
        }
    }
}

fn update_rigidbody(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Rigidbody)>,
) {
    let delta_time = time.delta_seconds();
    for (mut transform, mut rigidbody) in &mut query.iter_mut() {
        let gravity_force = GRAVITY_ACCELERATION * rigidbody.mass;

        rigidbody.force += gravity_force;   // Add gravity force.
        update_rigidbody_translation(delta_time, &mut *transform, &mut *rigidbody);
        rigidbody.force = Vec3::ZERO;
        update_rigidbody_rotation(delta_time, &mut *transform, &mut *rigidbody);
        rigidbody.torque = Vec3::ZERO;
    }
}

fn update_rigidbody_translation(
    delta_time: f32,
    transform: &mut Transform,
    rigidbody: &mut Rigidbody,
) {
    rigidbody.force += -rigidbody.drag * rigidbody.velocity;
    rigidbody.velocity += rigidbody.force * delta_time / rigidbody.mass;
    transform.translation += delta_time * rigidbody.velocity;
}

fn update_rigidbody_rotation(
    delta_time: f32,
    transform: &mut Transform,
    rigidbody: &mut Rigidbody,
) {
    rigidbody.torque += rigidbody.angular_drag * rigidbody.angular_velocity;
    rigidbody.angular_velocity += delta_time * rigidbody.inertia * rigidbody.torque;
    let quaternion_xyz = delta_time * 0.5 * rigidbody.angular_velocity;
    transform.rotation = (transform.rotation + transform.rotation * Quat::from_xyzw(
        quaternion_xyz.x, quaternion_xyz.y, quaternion_xyz.z, 0.0)).normalize();
}

fn reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rigidbody), With<Rigidbody>>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        println!("Reset!");
        for (mut transform, mut rigidbody) in &mut query.iter_mut() {
            rigidbody.velocity = Vec3::ZERO;
            rigidbody.angular_velocity = Vec3::ZERO;
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
    }
}