#[macro_use]
extern crate log;
extern crate env_logger;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn init_lib(_py: Python) -> PyResult<u32> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    Ok(0)
}

#[pyfunction]
fn print_info(_py: Python, message: &str) -> PyResult<u32> {
    info!("{}", message);
    Ok(0)
}

#[pymodule]
fn rustpythonmoduleexample(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(init_lib))?;
    m.add_wrapped(wrap_pyfunction!(print_info))?;
    Ok(())
}
