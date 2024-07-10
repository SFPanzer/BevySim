use bevy::math::Mat3;

pub trait Collider: Send + Sync {
    fn get_inertia(&self, mass: f32) -> Mat3;
}