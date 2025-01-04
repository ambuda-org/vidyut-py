use crate::utils::py_enum;
use pyo3::prelude::*;

use crate::prakriya::args::{PyLakara, PyLinga, PyPurusha, PyVacana, PyVibhakti};
use vidyut_kosha::entries::*;

/// A short part-of-speech tag for some `Pada`.
///
/// These tags follow the normal distinction of *subanta* (nominal) and *tinanta* (verb). We also
/// model the *avyaya* (indeclinable) separately since it has different semantics from other
/// subantas.
#[pyclass(name = "PartOfSpeech", eq, eq_int, ord)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyPartOfSpeech {
    /// An indeclinable.
    Avyaya,
    /// A nominal.
    Subanta,
    /// A verb.
    Tinanta,
    /// Unknown part of speech.
    Unknown,
}

py_enum!(PyPartOfSpeech, POSTag, [Avyaya, Subanta, Tinanta, Unknown]);

#[pyclass(name = "PadaPrayoga", eq, eq_int, ord)]
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyPadaPrayoga {
    Parasmaipada,
    AtmanepadaKartari,
    AtmanepadaNotKartari,
}

/// A verb root.
#[pyclass(name = "Dhatu", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyDhatu {
    /// The prefixes that this dhatu uses.
    prefixes: Vec<String>,
    /// The sanAdi-pratyayas that this dhatu uses.
    sanadi: Vec<String>,
    /// The base text of this dhatu.
    text: String,
}

#[pymethods]
impl PyDhatu {
    /// Create a new `Dhatu`.
    #[new]
    #[pyo3(signature = (*, text, prefixes, sanadi))]
    fn new(text: String, prefixes: Vec<String>, sanadi: Vec<String>) -> Self {
        Self {
            prefixes,
            sanadi,
            text,
        }
    }

    /// Create a new `Dhatu`.
    #[staticmethod]
    fn mula(text: &str) -> Self {
        Self {
            prefixes: Vec::new(),
            sanadi: Vec::new(),
            text: text.to_string(),
        }
    }

    fn __repr__(&self) -> String {
        format!("Dhatu(text='{}')", self.text)
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

impl From<PyDhatu> for Dhatu {
    fn from(val: PyDhatu) -> Self {
        Dhatu::mula(val.text)
    }
}

impl From<Dhatu> for PyDhatu {
    fn from(val: Dhatu) -> PyDhatu {
        PyDhatu {
            prefixes: val.prefixes().to_vec(),
            text: val.text().to_string(),
            sanadi: val
                .sanadi()
                .iter()
                .map(|s| s.as_str().to_string())
                .collect(),
        }
    }
}

/// A nominal stem.
#[pyclass(name = "Pratipadika", get_all, eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyPratipadika {
    /// The text of this pratipadika.
    text: String,
    // dhatu: Option<PyDhatu>,
    // lingas: Vec<PyLinga>
}

#[pymethods]
impl PyPratipadika {
    /// Create a new `Pratipadika`.
    #[new]
    #[pyo3(signature = (*, text))]
    fn new(text: String) -> Self {
        Self { text }
    }

    fn __repr__(&self) -> String {
        format!("Pratipadika(text='{}')", self.text)
    }

    fn __str__(&self) -> String {
        self.text.clone()
    }
}

impl From<PyPratipadika> for Pratipadika {
    fn from(val: PyPratipadika) -> Self {
        Pratipadika::Basic {
            text: val.text,
            lingas: vec![],
        }
    }
}

impl From<Dhatu> for PyPratipadika {
    fn from(val: Dhatu) -> Self {
        PyPratipadika {
            text: val.text().to_string(),
        }
    }
}

/// Morphological information for a Sanskrit word.
///
/// For ease of use in Python, all of these fields are defined on a single type.
#[pyclass(name = "Pada", get_all, eq, ord)]
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyPada {
    /// The part-of-speech.
    pos: Option<PyPartOfSpeech>,
    /// The verb root.
    dhatu: Option<PyDhatu>,
    pratipadika: Option<PyPratipadika>,
    purusha: Option<PyPurusha>,
    /// (For verbs) the tense-mood.
    lakara: Option<PyLakara>,
    pada_prayoga: Option<PyPadaPrayoga>,
    vacana: Option<PyVacana>,
    /// (For nominals) the gender.
    linga: Option<PyLinga>,
    /// (For nominals) the case.
    vibhakti: Option<PyVibhakti>,
    /// (For nominals) whether this word must not occur as the last member of a compound.
    is_purvapada: bool,
}

#[pymethods]
impl PyPada {
    /// Generic constructor. This is a useful test of our `repr` representation.
    /// TODO: handle creating a word with invalid state, e.g. a tinanta with no dhatu.
    /*
    #[new]
    #[pyo3(signature=(*, pos, dhatu, pratipadika, purusha, lakara, pada_prayoga, vacana, linga, vibhakti, is_purvapada))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        pos: Option<PyPartOfSpeech>,
        dhatu: Option<PyDhatu>,
        pratipadika: Option<PyPratipadika>,
        purusha: Option<PyPurusha>,
        lakara: Option<PyLakara>,
        pada_prayoga: Option<PyPadaPrayoga>,
        vacana: Option<PyVacana>,
        linga: Option<PyLinga>,
        vibhakti: Option<PyVibhakti>,
        is_purvapada: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            pos,
            dhatu,
            pratipadika,
            purusha,
            lakara,
            pada_prayoga,
            vacana,
            linga,
            vibhakti,
            is_purvapada,
        })
    }
    */

    /// Create a new *tinanta* (verb).
    #[staticmethod]
    #[pyo3(signature=(*, dhatu, purusha, vacana, lakara, pada_prayoga))]
    #[allow(clippy::too_many_arguments)]
    fn make_tinanta(
        dhatu: Option<PyDhatu>,
        purusha: Option<PyPurusha>,
        vacana: Option<PyVacana>,
        lakara: Option<PyLakara>,
        pada_prayoga: Option<PyPadaPrayoga>,
    ) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Tinanta),
            dhatu,
            purusha,
            lakara,
            vacana,
            pada_prayoga,
            ..Default::default()
        }
    }

    /// Create a new *subanta* (nominal).
    #[staticmethod]
    #[pyo3(signature=(*, pratipadika, linga, vibhakti, vacana))]
    #[allow(clippy::too_many_arguments)]
    fn make_subanta(
        pratipadika: Option<PyPratipadika>,
        linga: Option<PyLinga>,
        vibhakti: Option<PyVibhakti>,
        vacana: Option<PyVacana>,
    ) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Subanta),
            pratipadika,
            vacana,
            vibhakti,
            linga,
            ..Default::default()
        }
    }

    /// Create a new *avyaya* (indeclinable).
    #[staticmethod]
    #[pyo3(signature=(*, pratipadika))]
    #[allow(clippy::too_many_arguments)]
    fn make_avyaya(pratipadika: Option<PyPratipadika>) -> Self {
        Self {
            pos: Some(PyPartOfSpeech::Avyaya),
            pratipadika,
            ..Default::default()
        }
    }

    #[getter]
    pub fn lemma(&self) -> Option<String> {
        if let Some(d) = &self.dhatu {
            Some(d.text.to_string())
        } else {
            self.pratipadika.as_ref().map(|p| p.text.to_string())
        }
    }

    pub(crate) fn __repr__(&self) -> String {
        let mut args = Vec::new();

        // Common, part 1
        if let Some(x) = &self.pos {
            args.push(format!("pos={}", x.__pyo3__repr__()));
        }

        match self.pos {
            Some(PyPartOfSpeech::Tinanta) => {
                if let Some(x) = &self.dhatu {
                    args.push(format!("dhatu={}", x.__repr__()));
                };
                if let Some(x) = &self.purusha {
                    args.push(format!("purusha={}", x.__repr__()));
                };
                if let Some(x) = &self.vacana {
                    args.push(format!("vacana={}", x.__repr__()));
                };
                if let Some(x) = &self.lakara {
                    args.push(format!("lakara={}", x.__repr__()));
                };
            }
            Some(PyPartOfSpeech::Subanta) | Some(PyPartOfSpeech::Avyaya) => {
                if let Some(x) = &self.pratipadika {
                    args.push(format!("pratipadika={}", x.__repr__()));
                };
                if let Some(x) = &self.linga {
                    args.push(format!("linga={}", x.__repr__()));
                };
                if let Some(x) = &self.vibhakti {
                    args.push(format!("vibhakti={}", x.__repr__()));
                };
                if let Some(x) = &self.vacana {
                    args.push(format!("vacana={}", x.__repr__()));
                };
                let is_purvapada = match self.is_purvapada {
                    true => "True",
                    false => "False",
                };
                args.push(format!("is_purvapada={}", is_purvapada));
            }
            Some(PyPartOfSpeech::Unknown) | None => {
                args.push("None".to_string());
            }
        };

        let args = args.join(", ");
        format!("Pada({args})")
    }
}

impl From<Pada> for PyPada {
    fn from(val: Pada) -> Self {
        let mut res = PyPada::default();
        match val {
            Pada::Avyaya(a) => {
                res.pos = Some(PyPartOfSpeech::Avyaya);
                res.pratipadika = Some(PyPratipadika::new(a.pratipadika.lemma().to_string()));
            }
            Pada::Subanta(s) => {
                res.pos = Some(PyPartOfSpeech::Subanta);
                res.pratipadika = Some(PyPratipadika::new(s.pratipadika.lemma().to_string()));
                res.linga = s.linga.map(|x| x.into());
                res.vibhakti = s.vibhakti.map(|x| x.into());
                res.vacana = s.vacana.map(|x| x.into());
                res.is_purvapada = s.is_purvapada;
            }
            Pada::Tinanta(t) => {
                res.pos = Some(PyPartOfSpeech::Tinanta);
                res.dhatu = Some(PyDhatu::mula(t.dhatu.text()));
                res.lakara = Some(t.lakara.into());
                res.pada_prayoga = Some(t.pada.into());
                res.purusha = Some(t.purusha.into());
                res.vacana = Some(t.vacana.into());
            }
            Pada::Unknown => {
                res.pos = None;
            }
        };

        res
    }
}

impl From<PyPada> for Pada {
    fn from(val: PyPada) -> Self {
        use PyPartOfSpeech as PyPOS;

        match val.pos {
            Some(PyPOS::Tinanta) => Pada::Tinanta(Tinanta {
                dhatu: val.dhatu.map(|x| x.into()).expect("defined for tinanta"),
                purusha: val.purusha.map(|x| x.into()).expect("defined for tinanta"),
                vacana: val.vacana.map(|x| x.into()).expect("defined for tinanta"),
                lakara: val.lakara.map(|x| x.into()).expect("defined for tinanta"),
                pada: val
                    .pada_prayoga
                    .map(|x| x.into())
                    .expect("defined for tinanta"),
            }),

            Some(PyPOS::Subanta) => Pada::Subanta(Subanta {
                pratipadika: val
                    .pratipadika
                    .map(|x| x.into())
                    .expect("defined for subanta"),
                vacana: val.vacana.map(|x| x.into()),
                linga: val.linga.map(|x| x.into()),
                vibhakti: val.vibhakti.map(|x| x.into()),
                is_purvapada: val.is_purvapada,
            }),

            Some(PyPOS::Avyaya) => Pada::Avyaya(Avyaya {
                pratipadika: val
                    .pratipadika
                    .map(|x| x.into())
                    .expect("defined for avyaya"),
            }),
            Some(PyPOS::Unknown) | None => Pada::Unknown,
        }
    }
}
