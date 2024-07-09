use bevy::{prelude::Query, transform::components::Transform};

use crate::entity::Rigidbody;

pub fn pose_integrate_phase(
    mut query: Query<(&mut Transform, &mut Rigidbody)>
) {
    for(mut transform, mut rigidbody) in &mut query.iter_mut(){
        // TODO
    }
}