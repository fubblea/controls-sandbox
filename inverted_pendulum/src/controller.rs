const TARGET_ANGLE: f32 = 180.0; // Target angle in deg

pub fn get_vel(theta: f32) -> f32 {
    const VEL: f32 = 50.0;

    // Left side of platform
    if -TARGET_ANGLE < theta && theta <= 0.0 {
        -VEL
    }
    // Right side of platform
    else if 0.0 < theta && theta < TARGET_ANGLE {
        VEL
    }
    // Correct position
    else {
        0.0
    }
}
