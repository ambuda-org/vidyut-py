use pyo3::exceptions::PyKeyError;
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_prakriya::args::{Dhatu, TinantaArgs};
use vidyut_prakriya::{Ashtadhyayi, Dhatupatha};
use vidyut_prakriya::{Prakriya, Rule, Step};

pub mod args;
use args::*;

/// A step in the derivation.
#[pyclass(name = "Step")]
#[derive(Clone)]
pub struct PyStep {
    #[pyo3(get)]
    pub rule: Rule,
    #[pyo3(get)]
    pub result: String,
}

/// A derivation.
#[pyclass(name = "Prakriya")]
pub struct PyPrakriya {
    /// The final output of the derivation.
    #[pyo3(get)]
    pub text: String,

    /// All of the steps that were applied during the derivation.
    #[pyo3(get)]
    pub history: Vec<PyStep>,
}

#[pyclass(name = "Dhatu")]
pub struct PyDhatu {
    /// The aupadeshika form of this dhatu.
    #[pyo3(get)]
    pub upadesha: String,
}

fn to_py_history(history: &[Step]) -> Vec<PyStep> {
    history
        .iter()
        .map(|x| PyStep {
            rule: x.rule(),
            result: x.result().clone(),
        })
        .collect()
}

fn to_py_prakriyas(prakriyas: Vec<Prakriya>) -> Vec<PyPrakriya> {
    prakriyas
        .into_iter()
        .map(|p| PyPrakriya {
            text: String::from(p.text()),
            history: to_py_history(p.history()),
        })
        .collect()
}

#[pyclass(name = "Dhatupatha")]
pub struct PyDhatupatha(Dhatupatha);

#[pymethods]
impl PyDhatupatha {
    #[new]
    pub fn new(path: PathBuf) -> Self {
        Self(Dhatupatha::from_path(path).unwrap())
    }

    pub fn __getitem__(&self, key: String) -> PyResult<PyDhatu> {
        match self.0.get(&key) {
            Some(d) => Ok(PyDhatu {
                upadesha: d.upadesha().to_string(),
            }),
            None => Err(PyKeyError::new_err("No value for key")),
        }
    }
}

#[pyclass(name = "Ashtadhyayi")]
#[derive(Default)]
pub struct PyAshtadhyayi(Ashtadhyayi);

#[pymethods]
impl PyAshtadhyayi {
    /// Creates a new API manager
    ///
    /// This constructor is not called `new` because `new` is a reserved word in JavaScript.
    #[new]
    pub fn new() -> Self {
        Self(Ashtadhyayi::new())
    }

    #[args(
        py_args = "*",
        dhatu,
        prayoga,
        purusha,
        vacana,
        lakara,
        sanadi = "None"
    )]
    pub fn derive_tinantas(
        &self,
        dhatu: String,
        prayoga: Prayoga,
        purusha: Purusha,
        vacana: Vacana,
        lakara: Lakara,
        sanadi: Option<Sanadi>,
    ) -> PyResult<Vec<PyPrakriya>> {
        let tin_args = TinantaArgs::builder()
            .prayoga(prayoga.into())
            .purusha(purusha.into())
            .vacana(vacana.into())
            .lakara(lakara.into())
            .build()
            .expect("should have all required fields");

        let mut dhatu = Dhatu::builder()
            .upadesha(&dhatu)
            // TODO: set gana
            .gana(1);

        if let Some(sanadi) = sanadi {
            dhatu = dhatu.sanadi(&[sanadi.into()]);
        }
        let dhatu = dhatu.build().expect("should have all required fields");

        let results = self.0.derive_tinantas(&dhatu, &tin_args);
        let py_results = to_py_prakriyas(results);
        Ok(py_results)
    }
}
