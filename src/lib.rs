use ::vidyut::io::DataPaths;
use ::vidyut::parsing::Parser;
use ::vidyut::semantics::Semantics;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

impl ToPyDict for Semantics {
    fn as_pydict(&self) -> HashMap<String, String> {
        let mut ret = HashMap::new();
        let pos = match self {
            Semantics::PrefixGroup => "prefix-group",
            Semantics::Avyaya => "avyaya",
            Semantics::Subanta(_) => "subanta",
            Semantics::Tinanta(_) => "tinanta",
            Semantics::None => "none",
            Semantics::Ktva(_) => "ktva",
            Semantics::Tumun(_) => "tumun",
        };

        match self {
            // FIXME: add more semantics here.
            Semantics::Tinanta(s) => {
                ret.insert("purusha", s.purusha.to_str());
                ret.insert("vacana", s.vacana.to_str());
                // FIXME: add more fields here.
            }
            Semantics::Subanta(s) => {
                ret.insert("linga", s.linga.to_str());
                ret.insert("vacana", s.vacana.to_str());
                ret.insert("vibhakti", s.vibhakti.to_str());
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
struct PyParsedWord {
    pub text: String,
    pub lemma: String,
    pub semantics: HashMap<String, String>,
}

#[pymethods]
impl PyParsedWord {
    fn __repr__(&self) -> String {
        format!(
            "ParsedWord<(\"{}\", \"{}\", {:?})>",
            self.text, self.lemma, self.semantics
        )
    }
}

/// A Sanskrit parser.
#[pyclass(name = "Parser")]
struct PyParser {
    parser: Parser,
}

trait ToPyDict {
    fn as_pydict(&self) -> HashMap<String, String>;
}

#[pymethods]
impl PyParser {
    /// Initializes a Sanskrit parser by reading the necessary data from the filesystem. This
    /// method raises a ValueError if the initialiation fails.
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let path = PathBuf::from(path);
        let parser = Parser::from_paths(&DataPaths::from_dir(&path));
        match parser {
            Ok(parser) => Ok(PyParser { parser }),
            Err(e) => Err(PyValueError::new_err(format!(
                "Could not initialize parser. Error was: `{:?}`",
                e
            ))),
        }
    }

    /// Parses the given SLP1 input and returns a list of `ParsedWord` objects.
    pub fn parse(&self, slp1_text: &str) -> Vec<PyParsedWord> {
        let words = self.parser.parse(slp1_text);
        let mut ret = Vec::new();

        for word in words {
            ret.push(PyParsedWord {
                text: word.text.clone(),
                lemma: word.lemma(),
                semantics: word.semantics.as_pydict(),
            });
        }

        ret
    }
}

/// Vidyut: a lightning-fast Sanskrit toolkit.
///
/// Vidyut supports only SLP1 transliteration.
#[pymodule]
fn vidyut(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyParser>()?;
    Ok(())
}
