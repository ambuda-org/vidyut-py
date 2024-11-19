use pyo3::exceptions::{PyIOError, PyOSError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_chandas::{Akshara, Chandas, Jati, Pada, Vrtta, Weight};

/// A Sanskrit syllable.
///
/// An akshara follows the following rules:
///
/// - It must contain exactly one vowel.
/// - It must end with a vowel, an anusvara, or a visarga.
/// - It must not start with an anusvara or visarga.
///
/// Together, these three rurles mean that an input string has exactly one division into aksharas.
#[pyclass(name = "Akshara", get_all, eq)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PyAkshara {
    text: String,
    weight: String,
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

#[pymethods]
impl PyAkshara {
    #[new]
    #[pyo3(signature = (*, text, weight))]
    fn new(text: String, weight: String) -> Self {
        PyAkshara { text, weight }
    }

    #[getter]
    fn text(&self) -> &str {
        &self.text
    }

    #[getter]
    fn weight(&self) -> &str {
        &self.weight
    }

    fn __repr__(&self) -> String {
        format!("Akshara(text='{}', weight='{}')", self.text, self.weight)
    }
}

#[pyclass(name = "PatternWeight", eq, eq_int)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PyPatternWeight {
    G,
    L,
    Any,
}

#[pyclass(name = "Pada")]
#[derive(Clone)]
pub struct PyPada {}

#[pymethods]
impl PyPada {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    #[staticmethod]
    #[pyo3(signature = (text))]
    pub fn from_string(text: String) -> Self {
        Self {}
    }
}

/// Models a *vṛtta*, which defines a specific pattern of light and heavy syllables.
#[pyclass(name = "Vrtta")]
pub struct PyVrtta {
    #[pyo3(get)]
    name: String,
    padas: Vec<PyPada>,
}

#[pymethods]
impl PyVrtta {
    #[new]
    #[pyo3(signature = (name, padas))]
    pub fn new(name: String, padas: Vec<PyPada>) -> Self {
        Self { name, padas }
    }

    pub fn padas(&self) -> Vec<PyPada> {
        self.padas.clone()
    }
}

impl From<Vrtta> for PyVrtta {
    fn from(v: Vrtta) -> Self {
        Self {
            name: v.name().to_string(),
            // TODO: fill this in once Pada is exported.
            padas: vec![],
        }
    }
}

/// Models a *jāti*, which defines a specific pattern of *mātrā*s (morae).
#[pyclass(name = "Jati", eq)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyJati {
    #[pyo3(get)]
    name: String,
    matras: Vec<i32>,
}

#[pymethods]
impl PyJati {
    #[new]
    #[pyo3(signature = (name, matras))]
    pub fn new(name: String, matras: Vec<i32>) -> Self {
        Self { name, matras }
    }

    pub fn __repr__(&self) -> String {
        format!("Jati(name=\'{}\', matras={:?})", self.name, self.matras)
    }

    pub fn matras(&self) -> Vec<i32> {
        self.matras.clone()
    }
}

impl From<Jati> for PyJati {
    fn from(val: Jati) -> Self {
        Self {
            name: val.name().clone(),
            matras: val.matras().to_vec(),
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
    #[getter]
    fn vrtta(&self) -> Option<String> {
        self.vrtta.clone()
    }

    #[getter]
    fn aksharas(&self) -> Vec<Vec<PyAkshara>> {
        self.aksharas.clone()
    }

    fn __repr__(&self) -> String {
        match &self.vrtta {
            Some(s) => format!("MatchResult(name=\'{}\')", s),
            None => "MatchResult(name=None)".to_string(),
        }
    }
}

/// A metrical classifier.
#[pyclass(name = "Chandas")]
pub struct PyChandas {
    chandas: Chandas,
}

#[pymethods]
impl PyChandas {
    /// Initialize `Chedaka` by reading the necessary data from the directory at `path`.
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        match Chandas::from_file(path) {
            Ok(chandas) => Ok(PyChandas { chandas }),
            Err(_) => Err(PyOSError::new_err("Could not read data from path.")),
        }
    }

    #[staticmethod]
    fn from_text(text: String) -> PyResult<Self> {
        match Chandas::from_text(text) {
            Ok(chandas) => Ok(PyChandas { chandas }),
            Err(_) => Err(PyIOError::new_err("Could not parse text.")),
        }
    }

    /// Classify the input string against an internal list of meters.
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

    /// Return all vrttas defined on this classifier.
    #[getter]
    pub fn vrttas(&self) -> Vec<PyVrtta> {
        self.chandas
            .vrttas()
            .iter()
            .map(|x| x.clone().into())
            .collect()
    }

    /// Return all jatis defined on this classifier.
    #[getter]
    pub fn jatis(&self) -> Vec<PyJati> {
        self.chandas
            .jatis()
            .iter()
            .map(|x| x.clone().into())
            .collect()
    }
}
