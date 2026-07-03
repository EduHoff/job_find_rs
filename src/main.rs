use pyo3::ffi::c_str;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        py.run(c_str!(include_str!("hello.py")), None, None)?;        
        Ok(())
    })
}