use bevy::prelude::*;

#[derive(Component)]
pub struct Rigidbody {
    mass: f32,
    inertia_inverse: Mat3,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
    linear_damping: f32,
    angular_damping: f32,
    // collider: Option<Collision> // TODO!
}

impl Rigidbody {
    #[inline]
    pub fn new() -> Rigidbody{
        Self {
            mass: 0.0,
            inertia_inverse: Mat3::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            linear_damping: 0.0,
            angular_damping: 0.0
        }
    }

    /// The mass of this rigid body.
    #[inline]
    pub fn mass(&self) -> f32 {
        self.mass
    }

    /// The inversed inertia of this ridig body.
    #[inline]
    pub fn inertia_inverse(&self) -> Mat3 {
        self.inertia_inverse
    }

    #[inline]
    pub fn linear_velocity(&self) -> Vec3 {
        self.linear_velocity
    }

    #[inline]
    pub fn angular_velocity(&self) -> Vec3 {
        self.angular_velocity
    }

    #[inline]
    pub fn linear_damping(&self) -> f32 {
        self.linear_damping
    }

    #[inline]
    pub fn angular_damping(&self) -> f32 {
        self.angular_damping
    }

    /// Set the mass of rigid body.
    /// ```
    /// # use puma_physics::entity::Rigidbody;
    /// let mut rb = Rigidbody::new();
    /// rb.set_mass(1.0); 
    /// ```    
    /// ## Argument
    /// 
    /// - `mass` - The new mass of this rigid body. 
    ///  
    /// ## Panics 
    ///  
    /// Panics if `mass` is negtive.
    /// ```should_panic
    /// # use puma_physics::entity::Rigidbody;
    /// let mut rb = Rigidbody::new();
    /// rb.set_mass(-1.0); 
    /// ```    
    #[inline] 
    pub fn set_mass(&mut self, mass: f32) {
        assert!(mass >= 0.0);
        self.mass = mass;
    }

    /// Set the inertia of rigid body.
    ///  
    /// # Argument
    /// 
    /// - `inertia` - The new intertia of this rigid body.  
    #[inline]
    pub fn set_inertia(&mut self, inertia: Mat3) {
        self.inertia_inverse = inertia.inverse();
    } 

    /// Set the linear velocity of rigid body.
    ///  
    /// # Argument
    /// 
    /// - `linear_velocity` - The new linear velocity of this rigid body.  
    #[inline]
    pub fn set_linear_velocity(&mut self, linear_velocity: Vec3) {
        self.linear_velocity = linear_velocity;
    }

    /// Set the angular velocity of rigid body.
    ///  
    /// # Argument
    /// 
    /// - `angular_velocity` - The new angular velocity of this rigid body.   
    #[inline]
    pub fn set_angular_velocity(&mut self, angular_velocity: Vec3) {
        self.angular_velocity = angular_velocity;
    } 

    /// Set the linear damping of rigid body.
    /// ```
    /// # use puma_physics::entity::Rigidbody;
    /// let mut rb = Rigidbody::new();
    /// rb.set_linear_damping(1.0);  
    /// ```
    /// ## Argument
    /// 
    /// - `linear_velocity` - The new linear velocity of this rigid body.  
    ///   
    /// ## Panics
    ///  
    /// Panics if `linear_damping` is negtive.
    /// ```should_panic
    /// # use puma_physics::entity::Rigidbody;
    /// let mut rb = Rigidbody::new();
    /// rb.set_linear_damping(-1.0); 
    /// ```    
    #[inline]
    pub fn set_linear_damping(&mut self, linear_damping: f32) {
        assert!(linear_damping >= 0.0);
        self.linear_damping = linear_damping;
    }

    /// Set the angular damping of rigid body.
    /// 
    /// # Argument
    /// 
    /// - `angular_damping` - The new angular damping of this rigid body. 
    ///  
    /// # Panics
    /// 
    /// Panics if `angular_damping` is negtive.
    #[inline]
    pub fn set_angular_damping(&mut self, angular_damping: f32) {
        assert!(angular_damping >= 0.0);
        self.angular_damping = angular_damping;
    }  
}