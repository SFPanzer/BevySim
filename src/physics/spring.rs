use bevy::math::Vec3;

pub fn spring(
    translation0: Vec3,
    translation1: Vec3,
    velocity0: Vec3,
    velocity1: Vec3,
    stiffness: f32,
    damping: f32,
    max_squared_distance: f32,
    min_squared_distance: f32,
    delta_time: f32
) -> (Vec3, Vec3) {
    let translation0 = translation0 + velocity0 * delta_time;
    let translation1 = translation1 + velocity1 * delta_time;
    eprintln!("\n translation0: {}, translation1: {}, velocity0: {}, velocity1: {}", translation0, translation1, velocity0, velocity1);

    let spring_vector = translation1 - translation0;
    let squared_distance = spring_vector.length_squared();
    let direction = if squared_distance != 0.0 {spring_vector.normalize()} else {Vec3::new(0.0, 1.0, 0.0)};
    let close_rate = Vec3::dot(velocity0, direction) - Vec3::dot(velocity1, direction);
    eprintln!("spring_vector: {}, direction: {}, close_rate: {}", spring_vector, direction, close_rate);

    let mut deformation: f32 = 0.0;
    if squared_distance > max_squared_distance {
        deformation = squared_distance - max_squared_distance;
    } else if squared_distance < min_squared_distance {
        deformation = squared_distance - min_squared_distance;
    }

    let spring_force = deformation * stiffness;
    let mut damping_force = 0.0;
    if deformation != 0.0 {
        damping_force = close_rate * damping;
    }
    eprintln!("spring_force: {}, damping_force: {}", spring_force, damping_force);
    return ((spring_force - damping_force) * direction, (damping_force - spring_force) * direction);
}
