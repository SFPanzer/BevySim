mod rigidbody;

use bevy::prelude::*;
use bevy::ecs::entity::Entity;
use rigidbody::Rigidbody;

pub struct DistanceJointPlugin;

impl Plugin for DistanceJointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_distance_joint);
    }
}

#[derive(Component, Debug)]
pub struct DistanceJoint {
    pub target: Entity,
    pub min_squared_distance: f32,
    pub max_squared_distance: f32,
    pub local_frame0: Vec3,
    pub local_frame1: Vec3,
    pub stiffness: f32
}

impl Default for DistanceJoint {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            min_squared_distance: 0.0,
            max_squared_distance: 5.0,
            local_frame0: Vec3::ZERO,
            local_frame1: Vec3::ZERO,
            stiffness: 10.0,
        }
    }
}

fn update_distance_joint(
    mut query: Query<(&mut Transform, &mut Rigidbody, &mut DistanceJoint)>,
) {
    println!("is_empty: {}", query.is_empty());
    /*
    for (mut transform, mut rigidbody, mut distance_joint) in &mut query.iter_mut() {
        let joint_position0 = distance_joint.local_frame0; // TODO
        let joint_position1 = transform.translation + distance_joint.local_frame1;

        // Calculate distance.
        let direction = (joint_position0 - joint_position1).normalize();
        let squared_distance = (joint_position0 - joint_position1).length_squared();
        let mut deformation: f32 = 0.0;
        println!("squared_distance: {}", squared_distance);
        if squared_distance > distance_joint.max_squared_distance{
            deformation = squared_distance - distance_joint.max_squared_distance;
            println!("{}", deformation)
        } else if squared_distance < distance_joint.min_squared_distance {
            deformation = squared_distance - distance_joint.min_squared_distance;
        }

        // Add force.
        rigidbody.force += deformation * distance_joint.stiffness * direction;
    }*/
}