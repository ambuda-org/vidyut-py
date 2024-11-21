use crate::utils::py_only_enum;
use pyo3::exceptions::{PyFileNotFoundError, PyValueError};
use pyo3::prelude::*;
use std::path::PathBuf;
use vidyut_prakriya::args::{Pratipadika, Subanta, Tinanta};
use vidyut_prakriya::{Dhatupatha, Vyakarana};
use vidyut_prakriya::{Prakriya, Step};

pub mod args;
use args::*;

/// Renders a string Pythonically.
fn py_repr_string(text: &str) -> String {
    if text.contains('\'') {
        format!("{:?}", text)
    } else {
        format!("'{}'", text)
    }
}

/// Defines the source of some rule.
#[pyclass(name = "Source", eq, ord, frozen, hash)]
#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PySource {
    Ashtadhyayi,
    Dhatupatha,
    Kashika,
    Kaumudi,
    Linganushasana,
    Phit,
    Unadipatha,
    Varttika,
}
py_only_enum!(
    PySource,
    Source,
    [
        Ashtadhyayi,
        Dhatupatha,
        Kashika,
        Kaumudi,
        Linganushasana,
        Phit,
        Unadipatha,
        Varttika
    ]
);

#[pyclass(name = "Sutra", get_all, eq, ord)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PySutra {
    source: PySource,
    /// The code of this sutra.
    code: String,
    /// The text of this sutra.
    text: String,
}

#[pymethods]
impl PySutra {
    #[new]
    fn new(source: PySource, code: String, text: String) -> Self {
        Self { source, code, text }
    }

    fn __repr__(&self) -> String {
        format!(
            "Sutra(source={}, code={}, text={})",
            self.source.__repr__(),
            py_repr_string(&self.code),
            py_repr_string(&self.text)
        )
    }
}

/// A step in the derivation.
#[pyclass(name = "Step", eq, get_all, ord)]
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyStep {
    /// The source of the rule that was applied.
    pub source: PySource,
    /// The rule that was applied.
    pub code: String,
    /// The result of applying `rule`.
    ///
    /// `result` is a list of SLP1 strings.
    pub result: Vec<String>,
}

#[pymethods]
impl PyStep {
    #[new]
    fn new(source: PySource, code: String, result: Vec<String>) -> Self {
        Self {
            source,
            code,
            result,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Step(source={}, code={}, result='{:?}')",
            self.source.__repr__(),
            self.code,
            self.result
        )
    }
}

/// A derivation.
#[pyclass(name = "Prakriya", get_all)]
pub struct PyPrakriya {
    /// The final output of the derivation.
    ///
    /// `text` is an SLP1 string.
    pub text: String,
    /// All of the steps that were applied during the derivation.
    pub history: Vec<PyStep>,
}

fn to_py_history(history: &[Step]) -> Vec<PyStep> {
    use vidyut_prakriya::Rule;

    history
        .iter()
        .map(|x| {
            let (source, code) = match x.rule() {
                Rule::Ashtadhyayi(s) => (PySource::Ashtadhyayi, s),
                Rule::Dhatupatha(s) => (PySource::Dhatupatha, s),
                Rule::Kashika(s) => (PySource::Kashika, s),
                Rule::Kaumudi(s) => (PySource::Kaumudi, s),
                Rule::Linganushasana(s) => (PySource::Linganushasana, s),
                Rule::Phit(s) => (PySource::Phit, s),
                Rule::Unadipatha(s) => (PySource::Unadipatha, s),
                Rule::Varttika(s) => (PySource::Varttika, s),
            };

            PyStep {
                source,
                code: code.to_string(),
                result: x.result().iter().map(|t| t.text().to_string()).collect(),
            }
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
#[pyclass(name = "Entry", eq, ord)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
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
    #[new]
    fn new(code: String, dhatu: PyDhatu, artha: String) -> Self {
        Self { code, dhatu, artha }
    }

    fn __repr__(&self) -> String {
        format!(
            "Entry(code={}, dhatu={}, artha={})",
            py_repr_string(&self.code),
            self.dhatu.__repr__(),
            py_repr_string(&self.artha)
        )
    }
}

/// Provides an interface to various data files.
#[pyclass(name = "Data")]
pub struct PyData(PathBuf);

#[pymethods]
impl PyData {
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
        Ok(Self(path))
    }

    /// Return all entries defined in the Dhatupatha.
    pub fn load_dhatu_entries(&self) -> PyResult<Vec<PyEntry>> {
        let path = self.0.join("dhatupatha.tsv");
        match Dhatupatha::from_path(&path) {
            Ok(dhatupatha) => {
                let mut ret = Vec::new();
                for entry in dhatupatha.iter() {
                    ret.push(PyEntry {
                        code: entry.code().clone(),
                        dhatu: entry.dhatu().clone().into(),
                        artha: entry.artha().clone(),
                    });
                }
                Ok(ret)
            }
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

    /// Return all sutras used as rules.
    ///
    /// Sutras come from various sources, including:
    /// - the gana-sutras in the Dhatupatha
    /// - the Ashtadhyayi
    /// - the Unadipatha
    /// - various varttikas
    pub fn load_sutras(&self) -> PyResult<Vec<PySutra>> {
        let mut sutras = Vec::new();

        for (filename, source) in &[
            ("dhatupatha-ganasutras.tsv", PySource::Dhatupatha),
            ("sutrapatha.tsv", PySource::Ashtadhyayi),
            ("varttikas.tsv", PySource::Varttika),
            ("unadipatha.tsv", PySource::Unadipatha),
        ] {
            let path = self.0.join(filename);
            let text = std::fs::read_to_string(path)?;

            for line in text.lines() {
                let fields: Vec<_> = line.split('\t').collect();
                if let &[code, text] = &fields[..] {
                    sutras.push(PySutra::new(*source, code.to_string(), text.to_string()))
                } else {
                    return Err(PyValueError::new_err(format!(
                        "Could not parse `{}`",
                        filename
                    )));
                }
            }
        }
        Ok(sutras)
    }
}

/// An interface to the rules of the Ashtadhyayi.
///
/// Options:
/// - `log_steps` (default: `True`) -- If set, log each step in the prakriya. Disable this if
///   you want extra speed.
/// - `is_chandasi` (default: `False`) -- If set, use rules tagged with *chandasi*, *mantre*,
///   etc.
/// - `use_svaras` (default: `False`) -- If set, use rules that add svaras to the output.
/// - `nlp_mode` (default: `False`) -- If set, preserve the final `s`/`r` of words. This
///   behavior is useful for certain applications in natural language processing.
#[pyclass(name = "Vyakarana")]
#[derive(Default)]
pub struct PyVyakarana(Vyakarana);

#[pymethods]
impl PyVyakarana {
    /// Create an interface to the system.
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

    #[pyo3(signature = (dhatu))]
    pub fn derive_dhatus(&self, dhatu: &PyDhatu) -> Vec<PyPrakriya> {
        let args = dhatu.as_ref();
        let results = self.0.derive_dhatus(args);
        to_py_prakriyas(results)
    }

    /// Return all pratipadikas that can be derived from the given arguments.
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

    /// Return all tinantas that can be derived from the given initial arguments.
    #[pyo3(signature = (*, dhatu, prayoga, lakara, purusha, vacana))]
    pub fn derive_tinantas(
        &self,
        dhatu: &PyDhatu,
        prayoga: PyPrayoga,
        lakara: PyLakara,
        purusha: PyPurusha,
        vacana: PyVacana,
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

    /// Return all subantas that can be derived from the given arguments.
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
}
