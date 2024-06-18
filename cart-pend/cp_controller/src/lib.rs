use pyo3::{exceptions::PyValueError, prelude::*};

/// Calculate the action for the next step of the CartPole environment.
#[pyfunction]
fn get_action(obs: Vec<f64>) -> PyResult<f64> {
    match obs.len() {
        4 => {
            println!("Everything is fine");
            Ok(1.0)
        }
        _ => Err(PyValueError::new_err("Invalid observation length")),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn cp_controller(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_action, m)?)?;
    Ok(())
}
