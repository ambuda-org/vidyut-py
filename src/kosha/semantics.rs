use pyo3::basic::CompareOp;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use vidyut_kosha::morph::*;

/// Generates the following boilerplate methods:
/// - `__hash__`
/// - `__str__`
/// - `from_string`, which is the inverse of `__str__`
/// - `From<RustEnum> for PyEnum`
/// - `From<PyEnum> for RustEnum`
///
/// Requirements:
/// - Enum must derive `Hash`
macro_rules! py_enum {
    ($Py:ident, $Rust:ident, [$( $variant:ident ),*]) => {
        impl From<$Rust> for $Py {
            fn from(val: $Rust) -> Self {
                match val {
                    $(
                        $Rust::$variant => $Py::$variant,
                    )*
                }
            }
        }

        impl From<$Py> for $Rust {
            fn from(val: $Py) -> Self {
                match val {
                    $(
                        $Py::$variant => $Rust::$variant,
                    )*
                }
            }
        }

        #[pymethods]
        impl $Py {
            fn __hash__(&self) -> u64 {
                let mut hasher = DefaultHasher::new();
                self.hash(&mut hasher);
                hasher.finish()
            }

            fn __str__(&self) -> String {
                let rust: $Rust = self.clone().into();
                rust.as_str().to_string()
            }

            /// Create an enum value from the given string.
            ///
            /// This is an inverse of the `__str__` method.
            #[staticmethod]
            fn from_string(val: &str) -> PyResult<Self> {
                match $Rust::from_str(val) {
                    Ok(x) => Ok(x.into()),
                    Err(_) => Err(PyNotImplementedError::new_err("Could not parse {val}")),
                }
            }
        }
    }
}

/// A short part-of-speech tag for some `Pada`.
///
/// These tags follow the normal distinction of *subanta* (nominal) and *tinanta* (verb). We also
/// model the *avyaya* (indeclinable) separately since it has different semantics from other
/// subantas.
#[pyclass(name = "PartOfSpeech", eq, eq_int)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

/// The person of a Sanskrit verb.
#[pyclass(name = "Purusha", eq, eq_int)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PyPurusha {
    /// The first *puruṣa* (third person).
    Prathama,
    /// The middle *puruṣa* (second person).
    Madhyama,
    /// The last *puruṣa* (first person).
    Uttama,
}

py_enum!(PyPurusha, Purusha, [Prathama, Madhyama, Uttama]);

/// The gender of a Sanskrit nominal.
#[pyclass(name = "Linga", eq, eq_int)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PyLinga {
    /// The masculine gender.
    Pum,
    /// The feminine gender.
    Stri,
    /// The neuter gender.
    Napumsaka,
}

py_enum!(PyLinga, Linga, [Pum, Stri, Napumsaka]);

/// The number of a Sanskrit word.
#[pyclass(name = "Vacana", eq, eq_int)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PyVacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

py_enum!(PyVacana, Vacana, [Eka, Dvi, Bahu]);

/// The case of a Sanskrit nominal.
#[pyclass(name = "Vibhakti", eq, eq_int)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PyVibhakti {
    /// The first *vibhakti* (nominative case).
    Prathama,
    /// The second *vibhakti* (accusative case).
    Dvitiya,
    /// The third *vibhakti* (instrumental case).
    Trtiya,
    /// The fourth *vibhakti* (dative case).
    Caturthi,
    /// The fifth *vibhakti* (ablative case).
    Panchami,
    /// The sixth *vibhakti* (genitive case).
    Sasthi,
    /// The seventh *vibhakti* (locative case).
    Saptami,
    /// The first *vibhakti* in the condition of *sambodhana* (vocative case).
    Sambodhana,
}

py_enum!(
    PyVibhakti,
    Vibhakti,
    [Prathama, Dvitiya, Trtiya, Caturthi, Panchami, Sasthi, Saptami, Sambodhana]
);

/*
/// The number of a Sanskrit word.
#[allow(non_camel_case_types)]
#[pyclass(name = "Sanadi", eq, eq_int)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PySanadi {
    Ric,
    san,
    yaN,
    yaNluk,
    kAmyac,
    kyaN,
    kyac,
}

py_enum!(
    PySanadi,
    Sanadi,
    [Ric, san, yaN, yaNluk, kAmyac, kyaN, kyac]
);
*/

/// The tense-mood of a Sanskrit verb.
#[pyclass(name = "Lakara", eq, eq_int)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PyLakara {
    /// *laṭ-lakāra* (present indicative).
    Lat,
    /// *liṭ-lakāra* (perfect).
    Lit,
    /// *luṭ-lakāra* (periphrastic future).
    Lut,
    /// *lṛṭ-lakāra* (simple future).
    Lrt,
    /// *leṭ-lakāra* (Vedic subjunctive).
    Let,
    /// *loṭ-lakāra* (imperative).
    Lot,
    /// *laṅ-lakāra* (imperfect).
    Lan,
    /// *liṅ-lakāra* in the sense of benediction (benedictive).
    AshirLin,
    /// *liṅ-lakāra* in the sense of a rule or injunction (optative).
    VidhiLin,
    /// *luṅ-lakāra* (aorist).
    Lun,
    /// *luṅ-lakāra* without its *a-* prefix (injunctive).
    LunNoAgama,
    /// *lṛṅ-lakāra* (conditional).
    Lrn,
}

py_enum!(
    PyLakara,
    Lakara,
    [Lat, Lit, Lut, Lrt, Let, Lot, Lan, AshirLin, VidhiLin, Lun, LunNoAgama, Lrn]
);

#[pyclass(name = "PadaPrayoga", eq, eq_int)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PyPadaPrayoga {
    Parasmaipada,
    AtmanepadaKartari,
    AtmanepadaNotKartari,
}

py_enum!(
    PyPadaPrayoga,
    PadaPrayoga,
    [Parasmaipada, AtmanepadaKartari, AtmanepadaNotKartari]
);

/// A verb root.
#[pyclass(name = "Dhatu", get_all)]
#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn __richcmp__(&self, other: PyRef<PyDhatu>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.text == other.text;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
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
#[pyclass(name = "Pratipadika", get_all)]
#[derive(Clone, Debug, Eq, PartialEq)]
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

    fn __richcmp__(&self, other: PyRef<PyPratipadika>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.text == other.text;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
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
#[pyclass(name = "Pada", get_all)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    fn __richcmp__(&self, other: PyRef<PyPada>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();

        // TODO: this is messy and can easily become buggy.
        let is_eq = self.pos == other.pos
            /*
            && self.dhatu == other.dhatu;
            */
            && self.vacana == other.vacana
            && self.lakara == other.lakara
            && self.purusha == other.purusha
            && self.pada_prayoga == other.pada_prayoga
            && self.vibhakti == other.vibhakti
            && self.linga == other.linga
            && self.is_purvapada == other.is_purvapada;

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
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
                    args.push(format!("purusha={}", x.__pyo3__repr__()));
                };
                if let Some(x) = &self.vacana {
                    args.push(format!("vacana={}", x.__pyo3__repr__()));
                };
                if let Some(x) = &self.lakara {
                    args.push(format!("lakara={}", x.__pyo3__repr__()));
                };
                if let Some(x) = &self.pada_prayoga {
                    args.push(format!("pada_prayoga={}", x.__pyo3__repr__()));
                };
            }
            Some(PyPartOfSpeech::Subanta) | Some(PyPartOfSpeech::Avyaya) => {
                if let Some(x) = &self.pratipadika {
                    args.push(format!("pratipadika={}", x.__repr__()));
                };
                if let Some(x) = &self.linga {
                    args.push(format!("linga={}", x.__pyo3__repr__()));
                };
                if let Some(x) = &self.vibhakti {
                    args.push(format!("vibhakti={}", x.__pyo3__repr__()));
                };
                if let Some(x) = &self.vacana {
                    args.push(format!("vacana={}", x.__pyo3__repr__()));
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
