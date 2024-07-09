use bevy::prelude::*;

use crate::phase::pose_integrate_phase;

pub struct PumaPhysicsPlugin;

impl Plugin for PumaPhysicsPlugin {
    fn build(&self,app: &mut App) {
        app.add_systems(Update, pose_integrate_phase);
    }
}