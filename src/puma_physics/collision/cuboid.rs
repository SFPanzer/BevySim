use super::collider::Collider;
use bevy::prelude::Mat3;
use bevy::prelude::Vec3;

pub struct CuboidCollider {
    width: f32,
    height: f32,
    depth: f32,
}

impl Collider for CuboidCollider {
    /// Get the inertia matrix of this cuboid collider.
    /// 
    /// ## Panics
    /// 
    /// Panics if `mass` is negtive.  
    fn get_inertia(&self, mass: f32) -> Mat3 {
        assert!(mass >= 0.0);
        let sqr_size = Vec3::new(
            self.width * self.width,
            self.height * self.height,
            self.depth * self.depth,
        );
        let inertia = Vec3::new(
            sqr_size.y + sqr_size.z,
            sqr_size.x + sqr_size.z,
            sqr_size.x + sqr_size.y,
        ) * (mass / 12.0);
        Mat3::from_cols_array(&[
            inertia.x, 0.0, 0.0,
            0.0, inertia.y, 0.0,
            0.0, 0.0, inertia.z
        ])
    }
}
