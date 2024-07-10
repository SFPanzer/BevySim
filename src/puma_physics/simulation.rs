use bevy::prelude::*;

use crate::phase::pose_integrate_phase;

pub struct PumaPhysicsPlugin;

#[derive(Resource)]
pub struct PumaPhysicsSettings {
    pub delta_time : f32,
    pub gravity_acceleration : Vec3
}

impl Plugin for PumaPhysicsPlugin {
    fn build(&self,app: &mut App) {
        app.insert_resource(PumaPhysicsSettings {
            delta_time: 0.02,
            gravity_acceleration : Vec3::new(0.0, -9.8, 0.0)
        });
        app.add_systems(FixedUpdate, pose_integrate_phase);
    }
}