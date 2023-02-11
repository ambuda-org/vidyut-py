/*!
Wrappers for vidyut-prakriya arguments.

Pyo3 doesn't allow us to annotate existing enums, and using a wrapping struct has poor ergonomics
for callers. So instead, redefine our enums of interest.
*/
use pyo3::basic::CompareOp;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use vidyut_prakriya::args::*;

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

/// A verb root.
#[pyclass(name = "Dhatu", module = "prakriya")]
pub struct PyDhatu(Dhatu);

impl PyDhatu {
    pub fn new(d: Dhatu) -> Self {
        Self(d)
    }
}

#[pymethods]
impl PyDhatu {
    /// The aupadeshika form of this dhatu.
    #[getter]
    pub fn upadesha(&self) -> String {
        self.0.upadesha().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Dhatu(upadesha='{}')", self.0.upadesha())
    }
}

impl PyDhatu {
    pub fn as_ref(&self) -> &Dhatu {
        &self.0
    }
}

/// A nominal stem.
#[pyclass(name = "Pratipadika", module = "prakriya")]
pub struct PyPratipadika(Pratipadika);

impl PyPratipadika {
    pub fn as_ref(&self) -> &Pratipadika {
        &self.0
    }
}

#[pymethods]
impl PyPratipadika {
    /// Create a new pratipadika with the given text.
    #[new]
    #[pyo3(signature = (*, text))]
    pub fn new(text: String) -> Self {
        Self(Pratipadika::new(text))
    }

    /// The text of this stem.
    #[getter]
    pub fn text(&self) -> String {
        self.0.text().to_string()
    }

    fn __repr__(&self) -> String {
        format!("Pratipadika(text='{}')", self.0.text())
    }

    fn __richcmp__(&self, other: PyRef<PyPratipadika>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        let is_eq = self.0.text() == other.0.text();

        match op {
            CompareOp::Eq => (is_eq).into_py(py),
            CompareOp::Ne => (!is_eq).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pyclass(name = "Krt", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
#[allow(non_camel_case_types)]
pub enum PyKrt {
    /// -Alu
    Aluc,
    /// -Aru
    Aru,
    /// -a
    Ga,
    /// -a
    GaY,
    /// -in
    GinuR,
    /// -ura
    Gurac,
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
    /// -van
    Nvanip,
    /// -a
    Ra,
    /// -in
    Rini,
    /// -aka
    Rvuc,
    /// -aka
    Rvul,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
    /// -Ana (laBamAna, sevamAna, ...)
    SAnac,
    /// -Ana
    SAnan,
    /// -a
    Sa,
    /// -at (gacCat, Bavat, ...)
    Satf,
    /// -Taka (gATaka)
    Takan,
    /// -Tu (vepaTu). Allowed only for dhatus that are `qvit`.
    Tuc,
    /// -Uka
    Uka,
    /// -a
    aR,
    /// -a
    ac,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
    /// -ani
    ani,
    /// -at (jarat)
    atfn,
    /// -Ana
    cAnaS,
    /// -in
    ini,
    /// -itra
    itra,
    /// -izRu (alaMkarizRu, prajanizRu, ...)
    izRuc,
    /// -a
    ka,
    /// -i (udaDi, ...)
    kAnac,
    /// -Ana
    ki,
    /// -i
    kin,
    /// -luka (BIluka)
    klukan,
    /// -mara
    kmarac,
    /// -nu
    knu,
    /// -ru (BIru)
    kru,
    /// -ruka (BIruka)
    kruka,
    /// -snu (glAsnu, jizRu, ...)
    ksnu,
    /// -ta (gata, bhUta, ...)
    kta,
    /// -tavat (gatavat, bhUtavat, ...)
    ktavatu,
    /// -ti
    ktic,
    /// -ti
    ktin,
    /// -tri
    ktri,
    /// -tvA (gatvA, bhUtva, ...)
    ktvA,
    /// -ura (BaNgura, ...)
    kurac,
    /// -vara
    kvarap,
    /// -vas
    kvasu,
    /// (empty suffix)
    kvip,
    /// -ya
    kyap,
    /// -ana
    lyu,
    /// -ana
    lyuw,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
    /// -u
    qu,
    /// -na (namra, kampra, ...)
    ra,
    /// -ru
    ru,
    /// -tavya (gantavya, bhavitavya, ...)
    tavya,
    /// -tavya
    tavyat,
    /// -tf (gantA, bhavitA, ...)
    tfc,
    /// -tf
    tfn,
    /// -tum (gantum, bhavitum, ...)
    tumun,
    /// -u (yuyutsu, Bikzu, ...)
    u,
    /// -uka
    ukaY,
    /// -vaca
    varac,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -a
    wak,
    /// -ya
    yat,
    /// -ana
    yuc,
    /// -Aka
    zAkan,
    /// -aka
    zvun,
    /// -tra
    zwran,
}

py_enum!(
    PyKrt,
    Krt,
    [
        Aluc, Aru, Ga, GaY, GinuR, Gurac, Kal, Nvanip, Ra, Rini, Rvuc, Rvul, Ryat, Ryuw, SAnac,
        SAnan, Sa, Satf, Takan, Tuc, Uka, aR, ac, anIyar, ani, atfn, cAnaS, ini, itra, izRuc,
        kAnac, ka, ki, kin, klukan, kmarac, knu, kru, kruka, ksnu, kta, ktavatu, ktic, ktin, ktri,
        ktvA, kurac, kvarap, kvasu, kvip, kyap, lyu, lyuw, naN, najiN, nan, qu, ra, ru, tavya,
        tavyat, tfc, tfn, tumun, u, ukaY, varac, vuY, vun, wak, yat, yuc, zAkan, zvun, zwran
    ]
);

/// The lakara to use in the derivation.
#[pyclass(name = "Lakara", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyLakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    /// The Vedic subjunctive. `vidyut-prakriya` currently has poor support for this lakara.
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes potential or hypothetical actions. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

py_enum!(
    PyLakara,
    Lakara,
    [Lat, Lit, Lut, Lrt, Let, Lot, Lan, VidhiLin, AshirLin, Lun, Lrn]
);

/// The linga to use in the derivation.
#[pyclass(name = "Linga", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyLinga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}

py_enum!(PyLinga, Linga, [Pum, Stri, Napumsaka]);

/// The prayoga of some tinanta.
#[pyclass(name = "Prayoga", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyPrayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}

py_enum!(PyPrayoga, Prayoga, [Kartari, Karmani, Bhave]);

/// The person of some tinanta.
#[pyclass(name = "Purusha", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyPurusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}

py_enum!(PyPurusha, Purusha, [Prathama, Madhyama, Uttama]);

#[pyclass(name = "Sanadi", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PySanadi {
    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    San,
    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    Yan,
    /// `yaN` with `luk` (unused)
    YanLuk,
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Nic,
}

py_enum!(PySanadi, Sanadi, [San, Yan, YanLuk, Nic]);

/// The number of some tinanta or subanta.
#[pyclass(name = "Vacana", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyVacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

py_enum!(PyVacana, Vacana, [Eka, Dvi, Bahu]);

/// The case ending of some subanta.
#[pyclass(name = "Vibhakti", module = "prakriya")]
#[derive(Copy, Clone, Hash)]
pub enum PyVibhakti {
    /// The first vibhakti. Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
    /// The first vibhakti used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti but rather an additional semantic condition
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}

py_enum!(
    PyVibhakti,
    Vibhakti,
    [Prathama, Dvitiya, Trtiya, Caturthi, Panchami, Sasthi, Saptami, Sambodhana]
);
