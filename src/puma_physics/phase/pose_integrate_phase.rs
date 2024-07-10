use bevy::{prelude::{Query, Res}, transform::components::Transform};

use crate::{entity::Rigidbody, simulation::PumaPhysicsSettings};

pub fn pose_integrate_phase(
    physics_settings: Res<PumaPhysicsSettings>,
    mut query: Query<&mut Rigidbody>
) {
    for mut rigidbody in &mut query.iter_mut(){
        let mut new_velocity = rigidbody.linear_velocity();
        new_velocity += physics_settings.delta_time * physics_settings.gravity_acceleration;
        rigidbody.set_linear_velocity(new_velocity);
    }
}