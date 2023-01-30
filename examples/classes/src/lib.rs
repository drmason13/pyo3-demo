use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// An example class implemented in Rust as a struct
#[pyclass]
struct ExampleClass {
    #[pyo3(get, set)]
    value: i32,
}

#[pymethods]
impl ExampleClass {
    #[new]
    fn new(value: i32) -> Self {
        ExampleClass { value }
    }

    fn double(&mut self) -> PyResult<()> {
        self.value = self.value.checked_mul(2).ok_or(PyValueError::new_err(
            "Value is too large and would overflow if doubled",
        ))?;

        Ok(())
    }
}

/// An example module implemented in Rust using PyO3.
#[pymodule]
fn classes(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<ExampleClass>()?;

    Ok(())
}
