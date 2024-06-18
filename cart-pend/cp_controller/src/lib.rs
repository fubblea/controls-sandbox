use pyo3::{exceptions::PyValueError, prelude::*};

#[allow(dead_code)]
struct CartPoleState {
    cart_pos: f64,   // Cart position [m]
    cart_vel: f64,   // Cart velocity [m/s]
    pole_angle: f64, // Pole angle [rad]
    pole_vel: f64,   // Pole angular velocity [rad/s]
}

impl CartPoleState {
    fn from_observation(obs: Vec<f64>) -> Self {
        Self {
            cart_pos: obs[0],
            cart_vel: obs[1],
            pole_angle: obs[2],
            pole_vel: obs[3],
        }
    }
}

/// Calculate the action for the next step of the CartPole environment.
#[pyfunction]
fn get_action(obs_raw: Vec<f64>) -> PyResult<f64> {
    match obs_raw.len() {
        4 => (),
        _ => return Err(PyValueError::new_err("Invalid observation length")),
    }

    // Unwrap the observation
    let obs = CartPoleState::from_observation(obs_raw);

    if obs.pole_vel > 0.0 {
        // If the pole is falling to the right, the action is to push the cart to the left
        Ok(1.0)
    } else {
        // If the pole is falling to the left, the action is to push the cart to the right
        Ok(0.0)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn cp_controller(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_action, m)?)?;
    Ok(())
}
