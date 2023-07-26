use pyo3::prelude::*;

extern crate alloc;

/// A Python module implemented in Rust.
#[pymodule]
fn _hy(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn rust_hello() -> PyResult<String> {
        Ok("hello".to_string())
    }

    Ok(())
}
