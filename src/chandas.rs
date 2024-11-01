use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_chandas::Chandas;

#[pyclass(name = "Chandas")]
pub struct PyChandas {
    chandas: Chandas,
}

#[pymethods]
impl PyChandas {
    /// Initialize `Chedaka` by reading the necessary data from the directory at `path`.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Chandas::from_file(&path) {
            Ok(chandas) => Ok(PyChandas { chandas }),
            Err(_) => Err(PyOSError::new_err(format!(
                "Could not read data from path."
            ))),
        }
    }
}
