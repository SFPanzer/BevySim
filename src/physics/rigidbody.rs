use bevy::prelude::*;

pub struct RigidBodyPlugin;

pub const GRAVITY_ACCELERATION: Vec3 = Vec3::new(0.0, -9.8, 0.0);

impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_rigidbody);
        app.add_systems(Update, reset);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
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
            drag: 0.5,
            angular_drag: 0.5,
        }
    }
}

#[derive(Bundle)]
pub struct CuboidRigidBodyBoundle {
    pub pbr_bundle: PbrBundle,
    pub rigidbody: Rigidbody,
}

fn cuboid_inertia(mass: f32, width: f32, height: f32) -> f32 {
    return (width * width + height * height) * mass / 12.0;
}

impl CuboidRigidBodyBoundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        material: Handle<StandardMaterial>,
        size: Vec3,
        density: f32,
    ) -> Self {
        let mass = size.x * size.y * size.z * density;
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Cuboid::new(size.x, size.y, size.z)),
                material: material,
                ..default()
            },
            rigidbody: Rigidbody {
                mass: mass,
                inertia: Mat3::from_cols_array(&[
                    cuboid_inertia(mass, size.y, size.z),
                    0.0,
                    0.0,
                    0.0,
                    cuboid_inertia(mass, size.x, size.z),
                    0.0,
                    0.0,
                    0.0,
                    cuboid_inertia(mass, size.x, size.y),
                ])
                .inverse(),
                ..default()
            },
        }
    }
}

impl Rigidbody {
    pub fn add_force(
        &mut self,
        transform: Transform,
        local_space_attach_point: Vec3,
        force: Vec3,
        gizmos: Option<&mut Gizmos>,
    ) {
        eprintln!("Add force: force: {}", force);
        let rotate_axis = 0.02 * 0.5 * self.angular_velocity;
        let quat_next_frame = (transform.rotation
            + Quat::from_xyzw(rotate_axis.x, rotate_axis.y, rotate_axis.z, 0.0)
                * transform.rotation)
            .normalize();
        let rotated_attach_point = quat_next_frame * local_space_attach_point;
        let torque = rotated_attach_point.cross(force);

        if let Some(gizmos) = gizmos {
            gizmos.arrow(
                rotated_attach_point + transform.translation,
                rotated_attach_point + transform.translation + force * 0.1,
                Color::CYAN,
            );
            gizmos.line(
                transform.translation,
                transform.translation + torque * 0.1,
                Color::CYAN,
            );
        }
        self.force += force;
        self.torque += torque;
    }
}

fn update_rigidbody(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(&mut Transform, &mut Rigidbody)>,
) {
    let delta_time = f32::min(time.delta_seconds(), 0.02);
    for (mut transform, mut rigidbody) in &mut query.iter_mut() {
        let gravity_force = GRAVITY_ACCELERATION * rigidbody.mass;
        rigidbody.force += gravity_force; // Add gravity force.

        // Update translation.
        let drag_force = rigidbody.drag * -rigidbody.velocity;
        rigidbody.force += drag_force;
        let delta_velocity = rigidbody.force * delta_time / rigidbody.mass;
        rigidbody.velocity += delta_velocity;
        rigidbody.velocity = Vec3::min(rigidbody.velocity, Vec3::ONE * 5.0);
        transform.translation += delta_time * rigidbody.velocity;

        // Update rotation.
        let angular_drag_torque = rigidbody.angular_drag * -rigidbody.angular_velocity;
        rigidbody.torque += angular_drag_torque;
        let delta_angular_velocity = delta_time * rigidbody.inertia * rigidbody.torque;
        rigidbody.angular_velocity += delta_angular_velocity;
        let rotate_axis = delta_time * 0.5 * rigidbody.angular_velocity;
        transform.rotation = (transform.rotation
            + Quat::from_xyzw(rotate_axis.x, rotate_axis.y, rotate_axis.z, 0.0)
                * transform.rotation)
            .normalize();
        // Force visualize.
        gizmos.arrow(
            transform.translation,
            transform.translation + rigidbody.force * 0.1,
            Color::FUCHSIA,
        );
        gizmos.line(
            transform.translation,
            transform.translation + rigidbody.torque * 0.1,
            Color::FUCHSIA,
        );

        rigidbody.force = Vec3::ZERO;
        rigidbody.torque = Vec3::ZERO;
    }
}

fn reset(
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Rigidbody), With<Rigidbody>>,
) {
    if key_input.just_pressed(KeyCode::Enter) {
        eprintln!("Reset!");
        for (mut transform, mut rigidbody) in &mut query.iter_mut() {
            eprintln!("Translation: {}", transform.translation);
            rigidbody.velocity = Vec3::ZERO;
            rigidbody.angular_velocity = Vec3::ZERO;
            rigidbody.force = Vec3::ZERO;
            rigidbody.torque = Vec3::ZERO;
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
    }
}
