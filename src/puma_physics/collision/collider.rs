use bevy::math::Mat3;

pub(crate) trait Collider {
    fn get_inertia(&self, mass: f32) -> Mat3;
}