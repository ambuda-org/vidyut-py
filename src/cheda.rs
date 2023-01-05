use vidyut_cheda::{Chedaka, Config};
use vidyut_kosha::semantics::Pada;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

trait ToPyDict {
    fn to_pydict(&self) -> HashMap<String, String>;
}

impl ToPyDict for Pada {
    fn to_pydict(&self) -> HashMap<String, String> {
        let mut ret = HashMap::new();
        let pos = match self {
            Pada::Avyaya(_) => "avyaya",
            Pada::Subanta(_) => "subanta",
            Pada::Tinanta(_) => "tinanta",
            Pada::None => "none",
        };

        match self {
            // FIXME: add more semantics here.
            Pada::Tinanta(s) => {
                ret.insert("purusha", s.purusha.as_str());
                ret.insert("vacana", s.vacana.as_str());
                // FIXME: add more fields here.
            }
            Pada::Subanta(s) => {
                ret.insert("linga", s.linga.as_str());
                ret.insert("vacana", s.vacana.as_str());
                ret.insert("vibhakti", s.vibhakti.as_str());
                // FIXME: how to set Python boolean here?
                ret.insert(
                    "is_purvapada",
                    if s.is_purvapada { "true" } else { "false" },
                );
            }
            &_ => (),
        }

        ret.insert("pos", pos);
        ret.iter()
            .map(|(x, y)| (x.to_string(), y.to_string()))
            .collect()
    }
}

/// A parsed word.
#[allow(dead_code)]
#[pyclass(name = "ParsedWord")]
pub struct PyParsedWord {
    pub text: String,
    pub lemma: String,
    pub info: HashMap<String, String>,
}

#[pymethods]
impl PyParsedWord {
    fn __repr__(&self) -> String {
        format!(
            "ParsedWord<(\"{}\", \"{}\", {:?})>",
            self.text, self.lemma, self.info
        )
    }
}

/// A Sanskrit parser.
#[pyclass(name = "Chedaka")]
pub struct PyChedaka {
    chedaka: Chedaka,
}

#[pymethods]
impl PyChedaka {
    /// Initializes a Sanskrit parser by reading the necessary data from the filesystem. This
    /// method raises a ValueError if the initialiation fails.
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let config = Config::new(&PathBuf::from(path));
        let chedaka = Chedaka::new(config);
        match chedaka {
            Ok(chedaka) => Ok(PyChedaka { chedaka }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Could not initialize segmenter. Error was: `{:?}`",
                e
            ))),
        }
    }

    /// Parses the given SLP1 input and returns a list of `ParsedWord` objects.
    pub fn tokenize(&self, slp1_text: &str) -> Vec<PyParsedWord> {
        let words = self.chedaka.tokenize(slp1_text);
        let mut ret = Vec::new();

        for word in words {
            ret.push(PyParsedWord {
                text: word.text.clone(),
                lemma: word.lemma(),
                info: word.info.to_pydict(),
            });
        }

        ret
    }
}
