pub trait Controller {
    fn get_force(&self, x_pos: f32, theta: f32) -> f32;
}

pub struct DumbController {
    x_target: f32,
    theta_target: f32,
    actuator_gain: f32,
}

impl DumbController {
    pub fn from_target(x_target: f32, theta_target: f32, actuator_gain: f32) -> Self {
        Self {
            x_target,
            theta_target,
            actuator_gain,
        }
    }
}

impl Controller for DumbController {
    fn get_force(&self, x_pos: f32, theta: f32) -> f32 {
        let x_diff = x_pos - self.x_target;

        if x_diff.abs() < 1.0 {
            let theta_diff = theta - self.theta_target;

            if theta_diff < 0.0 {
                return self.actuator_gain * 100.0;
            } else if theta_diff > 0.0 {
                return self.actuator_gain * -100.0;
            } else {
                return 0.0;
            }
        } else if x_diff < 0.0 {
            return self.actuator_gain * 1000.0;
        } else {
            return self.actuator_gain * -1000.0;
        }
    }
}
