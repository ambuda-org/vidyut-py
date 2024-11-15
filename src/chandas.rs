use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_chandas::{Akshara, Chandas, Weight};

#[pyclass(name = "Akshara", get_all, eq)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PyAkshara {
    pub text: String,
    pub weight: String,
}

impl From<&Akshara> for PyAkshara {
    fn from(a: &Akshara) -> Self {
        Self {
            text: a.text().to_string(),
            weight: match a.weight() {
                Weight::G => "G".to_string(),
                Weight::L => "L".to_string(),
            },
        }
    }
}

#[pyclass(name = "MatchResult", get_all, eq)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PyMatchResult {
    pub vrtta: Option<String>,
    pub aksharas: Vec<Vec<PyAkshara>>,
}

#[pymethods]
impl PyMatchResult {
    fn __repr__(&self) -> String {
        match &self.vrtta {
            Some(s) => format!("PyMatchResult(name=\'{}\')", s),
            None => "PyMatchResult(name=None)".to_string(),
        }
    }
}

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

    pub fn classify(&self, text: &str) -> PyResult<PyMatchResult> {
        let res = self.chandas.classify(text);

        let padya_name = res.padya().as_ref().map(|v| v.name().to_string());
        let mut aksharas = Vec::new();
        for row in res.aksharas() {
            let new_row: Vec<_> = row.iter().map(|a| a.into()).collect();
            aksharas.push(new_row);
        }
        Ok(PyMatchResult {
            vrtta: padya_name,
            aksharas,
        })
    }
}
