use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_prakriya::args::{BaseKrt, Krdanta, Krt, Pratipadika, Subanta, Taddhitanta, Tinanta};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
use vidyut_prakriya::{Prakriya, Step};

pub mod args;
use args::*;

/// A step in the derivation.
#[pyclass(name = "Step", get_all)]
#[derive(Clone)]
pub struct PyStep {
    /// The rule that was applied.
    pub rule: String,
    /// The result of applying `rule`.
    pub result: Vec<String>,
}

#[pymethods]
impl PyStep {
    #[new]
    fn new(rule: String, result: Vec<String>) -> Self {
        Self { rule, result }
    }

    fn __repr__(&self) -> String {
        format!("Step(rule='{}', result='{:?}')", self.rule, self.result)
    }

    fn __richcmp__(&self, other: PyRef<PyStep>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.rule == other.rule && self.result == other.result;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

/// A derivation.
#[pyclass(name = "Prakriya", get_all)]
pub struct PyPrakriya {
    /// The final output of the derivation.
    pub text: String,
    /// All of the steps that were applied during the derivation.
    pub history: Vec<PyStep>,
}

fn to_py_history(history: &[Step]) -> Vec<PyStep> {
    history
        .iter()
        .map(|x| PyStep {
            rule: x.rule().code().to_string(),
            result: x.result().iter().map(|t| t.text().to_string()).collect(),
        })
        .collect()
}

fn to_py_prakriyas(prakriyas: Vec<Prakriya>) -> Vec<PyPrakriya> {
    prakriyas
        .into_iter()
        .map(|p| PyPrakriya {
            text: p.text(),
            history: to_py_history(p.history()),
        })
        .collect()
}

/// An entry in the Dhatupatha.
#[pyclass(name = "Entry")]
pub struct PyEntry {
    /// The numeric code associtaed witht this dhatu.
    ///
    /// If using `dhatupatha.tsv`, this code is consistent with the codes used by ashtadhyayi.com.
    #[pyo3(get)]
    code: String,
    /// The
    #[pyo3(get)]
    dhatu: PyDhatu,
    #[pyo3(get)]
    artha: String,
}

#[pymethods]
impl PyEntry {
    fn __repr__(&self) -> String {
        format!(
            "Entry(code='{}', dhatu={:?}, artha=\"{}\")",
            self.code,
            self.dhatu.__repr__(),
            self.artha
        )
    }
}

/// Provides an interface to the Dhatupatha.
#[pyclass(name = "Dhatupatha", mapping)]
pub struct PyDhatupatha(Dhatupatha);

#[pymethods]
impl PyDhatupatha {
    /// Create a new dhatupatha instance from the given `path`.
    ///
    /// `path` should point to a 3-column TSV with columns `code`, `dhatu`, and `artha`.
    ///
    /// - `code` should be a number in the format "X.Y", such as "01.0001".
    ///
    /// - `dhatu` should be the dhatu`s aupadeshika form transliterated as SLP1.
    ///   Svaras and nasal vowels must be indicated explictly.
    ///
    /// - `artha` is an arbitrary string.
    ///
    /// Exceptions:
    /// - `FlieNotFoundError` if the file does not exist.
    /// - `ValueError` if the function cannot parse the input file.
    #[new]
    pub fn new(path: PathBuf) -> PyResult<Self> {
        if !path.exists() {
            let message = format!("No such file: '{}'", path.display());
            return Err(PyFileNotFoundError::new_err(message));
        }

        match Dhatupatha::from_path(&path) {
            Ok(d) => Ok(Self(d)),
            Err(e) => {
                let message = format!(
                    "Could not parse file '{}'. Error was: `{}`",
                    path.display(),
                    e
                );
                Err(PyValueError::new_err(message))
            }
        }
    }

    /// Return all entries defined in the Dhatupatha.
    pub fn entries(&self) -> Vec<PyEntry> {
        let mut ret = Vec::new();
        for e in self.0.iter() {
            ret.push(PyEntry {
                code: e.code().clone(),
                dhatu: e.dhatu().clone().into(),
                artha: e.artha().clone(),
            });
        }
        ret
    }
}

/// An interface to the rules of the Ashtadhyayi.
#[pyclass(name = "Vyakarana")]
#[derive(Default)]
pub struct PyVyakarana(Vyakarana);

#[pymethods]
impl PyVyakarana {
    /// Creates an interface to the system.
    ///
    /// Options:
    /// - `log_steps` (default: `True`) -- If set, log each step in the prakriya. Disable this if
    ///   you want extra speed.
    /// - `is_chandasi` (default: `False`) -- If set, use rules tagged with *chandasi*, *mantre*,
    ///   etc.
    /// - `use_svaras` (default: `False`) -- If set, use rules that add svaras to the output.
    /// - `nlp_mode` (default: `False`) -- If set, preserve the final `s`/`r` of words. This
    ///   behavior is useful for certain applications in natural language processing.
    #[new]
    #[pyo3(signature = (*, log_steps=true, is_chandasi=false, use_svaras=false, nlp_mode=false))]
    pub fn new(log_steps: bool, is_chandasi: bool, use_svaras: bool, nlp_mode: bool) -> Self {
        Self(
            Vyakarana::builder()
                .log_steps(log_steps)
                .is_chandasi(is_chandasi)
                .use_svaras(use_svaras)
                .nlp_mode(nlp_mode)
                .build(),
        )
    }

    /// Return all possible tinanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, dhatu, prayoga, purusha, vacana, lakara))]
    pub fn derive_tinantas(
        &self,
        dhatu: &PyDhatu,
        prayoga: PyPrayoga,
        purusha: PyPurusha,
        vacana: PyVacana,
        lakara: PyLakara,
    ) -> Vec<PyPrakriya> {
        let tin_args = Tinanta::builder()
            .dhatu(dhatu.as_rust().clone())
            .prayoga(prayoga.into())
            .purusha(purusha.into())
            .vacana(vacana.into())
            .lakara(lakara.into())
            .build()
            .expect("should have all required fields");

        let results = self.0.derive_tinantas(&tin_args);
        to_py_prakriyas(results)
    }

    /// Return all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, dhatu, krt))]
    pub fn derive_krdantas(&self, dhatu: &PyDhatu, krt: PyKrt) -> Vec<PyPrakriya> {
        let base_krt: BaseKrt = krt.into();
        let args = Krdanta::new(dhatu.as_ref().clone(), Krt::from(base_krt));

        let results = self.0.derive_krdantas(&args);
        to_py_prakriyas(results)
    }

    /// Return all possible taddhitanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, pratipadika, taddhita))]
    pub fn derive_taddhitantas(
        &self,
        pratipadika: &PyPratipadika,
        taddhita: PyTaddhita,
    ) -> Vec<PyPrakriya> {
        let args = Taddhitanta::builder()
            .pratipadika(pratipadika.as_ref().clone())
            .taddhita(taddhita.into())
            .build()
            .expect("has both pratipadika and taddhita");

        let results = self.0.derive_taddhitantas(&args);
        to_py_prakriyas(results)
    }

    /// Return all possible krdanta prakriyas that can be derived with the given initial
    /// conditions.
    #[pyo3(signature = (*, pratipadika, linga, vibhakti, vacana))]
    pub fn derive_subantas(
        &self,
        pratipadika: &PyPratipadika,
        linga: PyLinga,
        vibhakti: PyVibhakti,
        vacana: PyVacana,
    ) -> Vec<PyPrakriya> {
        let args = Subanta::new(
            pratipadika.as_ref(),
            linga.into(),
            vibhakti.into(),
            vacana.into(),
        );
        let results = self.0.derive_subantas(&args);
        to_py_prakriyas(results)
    }

    #[pyo3(signature = (dhatu))]
    pub fn derive_dhatus(&self, dhatu: &PyDhatu) -> Vec<PyPrakriya> {
        let args = dhatu.as_ref();
        let results = self.0.derive_dhatus(args);
        to_py_prakriyas(results)
    }

    /// Creates a pratipadika from the given input.
    ///
    /// This function is meant mainly for krdantas and taddhitantas. If a pratipadika cannot be
    /// derived, the function returns an empty list.
    #[pyo3(signature = (pratipadika))]
    pub fn derive_pratipadikas(&self, pratipadika: &PyPratipadika) -> Vec<PyPrakriya> {
        let args = pratipadika.as_ref();
        match args {
            Pratipadika::Krdanta(k) => {
                let results = self.0.derive_krdantas(k);
                to_py_prakriyas(results)
            }
            Pratipadika::Taddhitanta(t) => {
                let results = self.0.derive_taddhitantas(t);
                to_py_prakriyas(results)
            }
            _ => Vec::new(),
        }
    }
}
