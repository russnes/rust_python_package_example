#[macro_use]
extern crate log;
extern crate env_logger;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[pyfunction]
fn print_info(_py: Python, message: &str) -> PyResult<u32> {
    info!("{}", message);
    Ok(0)
}

#[pymodule]
fn module_example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    m.add_wrapped(wrap_pyfunction!(print_info))?;
    Ok(())
}
