use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;
use std::path::PathBuf;
use vidyut_kosha::Kosha;

#[pyclass(name = "Kosha")]
pub struct PyKosha(Kosha);

#[pymethods]
impl PyKosha {
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Kosha::new(&path) {
            Ok(k) => Ok(Self(k)),
            Err(_) => Err(PyIOError::new_err("Unknown error -- our guess is that the input file is missing.")),
        }
    }
}
