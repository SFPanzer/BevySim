use bevy::ecs::entity::Entity;
use bevy::prelude::*;

use super::rigidbody::*;
use super::spring::spring;

pub struct DistanceJointPlugin;

impl Plugin for DistanceJointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_distance_joint);
    }
}

#[derive(Component, Debug)]
pub struct DistanceJoint {
    pub actor0: Option<Entity>,
    pub actor1: Option<Entity>,
    pub min_squared_distance: f32,
    pub max_squared_distance: f32,
    pub local_frame0: Vec3,
    pub local_frame1: Vec3,
    pub stiffness: f32,
    pub damping: f32,
}

impl Default for DistanceJoint {
    fn default() -> Self {
        Self {
            actor0: None,
            actor1: None,
            min_squared_distance: 0.0,
            max_squared_distance: 1.0,
            local_frame0: Vec3::ZERO,
            local_frame1: Vec3::ZERO,
            stiffness: 10.0,
            damping: 10.0,
        }
    }
}

fn draw_ball(
    position: Vec3,
    radius: f32,
    color: Color,
    gizmos: &mut Gizmos,
){
    gizmos.circle(position, Direction3d::X, radius, color);
    gizmos.circle(position, Direction3d::Y, radius, color);
    gizmos.circle(position, Direction3d::Z, radius, color);
}

fn update_distance_joint(
    time: Res<Time>,
    mut gizmos: Gizmos,
    query_distance_joints: Query<&DistanceJoint>,
    mut query_actors: Query<(&Transform, Option<&mut Rigidbody>)>,
) {
    let delta_time = f32::min(time.delta_seconds(), 0.02);
    for distance_joint in query_distance_joints.iter() {
        // Calculate force.
        let mut translation0 = Vec3::ZERO;
        let mut translation1 = Vec3::ZERO;
        let mut velocity0 = Vec3::ZERO;
        let mut velocity1 = Vec3::ZERO;
        if let Some(entity) = distance_joint.actor0 {
            if let Ok((transform, rigidbody_opt)) = query_actors.get(entity) {
                translation0 =
                    transform.translation + transform.rotation * distance_joint.local_frame0;
                if let Some(rigidbody) = rigidbody_opt {
                    translation0 += rigidbody.velocity * delta_time;
                    velocity0 = rigidbody.velocity;
                }
            }
        }
        if let Some(entity) = distance_joint.actor1 {
            if let Ok((transform, rigidbody_opt)) = query_actors.get(entity) {
                translation1 =
                    transform.translation + transform.rotation * distance_joint.local_frame1;
                if let Some(rigidbody) = rigidbody_opt {
                    translation1 += rigidbody.velocity * delta_time;
                    velocity1 = rigidbody.velocity;
                }
            }
        }
        let (force0, force1) = spring(
            translation0,
            translation1,
            velocity0,
            velocity1,
            distance_joint.stiffness,
            distance_joint.damping,
            distance_joint.max_squared_distance,
            distance_joint.min_squared_distance,
            delta_time
        );

        // Visialize joint
        gizmos.line(translation0, translation1, Color::YELLOW);
        draw_ball(translation0, 0.1, Color::YELLOW, &mut gizmos);
        draw_ball(translation1, 0.1, Color::YELLOW, &mut gizmos);

        // Apply force.
        if let Some(entity) = distance_joint.actor0 {
            if let Ok((transform, rigidbody_opt)) = query_actors.get_mut(entity) {
                if let Some(mut rigidbody) = rigidbody_opt {
                    rigidbody.add_force(
                        *transform,
                        distance_joint.local_frame0,
                        force0,
                        Some(&mut gizmos),
                    )
                }
            }
        }
        if let Some(entity) = distance_joint.actor1 {
            if let Ok((transform, rigidbody_opt)) = query_actors.get_mut(entity) {
                if let Some(mut rigidbody) = rigidbody_opt {
                    rigidbody.add_force(
                        *transform,
                        distance_joint.local_frame1,
                        force1,
                        Some(&mut gizmos),
                    )
                }
            }
        }
    }
}
