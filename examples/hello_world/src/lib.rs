use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn greet(name: String) -> PyResult<String> {
    Ok(format!("Hello {name}"))
}

/// A Python module implemented in Rust.
#[pymodule]
fn hello_world(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
