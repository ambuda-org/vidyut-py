use pyo3::prelude::*;
use vidyut_prakriya::args as rust;

#[pyclass]
#[derive(Clone)]
pub enum Prayoga {
    Kartari,
    Karmani,
    Bhave,
}
impl From<Prayoga> for rust::Prayoga {
    fn from(val: Prayoga) -> Self {
        use Prayoga::*;
        match val {
            Kartari => Self::Kartari,
            Karmani => Self::Karmani,
            Bhave => Self::Bhave,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Purusha {
    Prathama,
    Madhyama,
    Uttama,
}
impl From<Purusha> for rust::Purusha {
    fn from(val: Purusha) -> Self {
        use Purusha::*;
        match val {
            Prathama => Self::Prathama,
            Madhyama => Self::Madhyama,
            Uttama => Self::Uttama,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Vacana {
    Eka,
    Dvi,
    Bahu,
}
impl From<Vacana> for rust::Vacana {
    fn from(val: Vacana) -> Self {
        use Vacana::*;
        match val {
            Eka => Self::Eka,
            Dvi => Self::Dvi,
            Bahu => Self::Bahu,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Lakara {
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
impl From<Lakara> for rust::Lakara {
    fn from(val: Lakara) -> Self {
        use Lakara::*;
        match val {
            Lat => Self::Lat,
            Lit => Self::Lit,
            Lut => Self::Lut,
            Lrt => Self::Lrt,
            Let => Self::Let,
            Lot => Self::Lot,
            Lan => Self::Lan,
            VidhiLin => Self::VidhiLin,
            AshirLin => Self::AshirLin,
            Lun => Self::Lun,
            Lrn => Self::Lrn,
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub enum Sanadi {
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
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Nic,
}
impl From<Sanadi> for rust::Sanadi {
    fn from(val: Sanadi) -> Self {
        use Sanadi::*;
        match val {
            San => Self::San,
            Yan => Self::Yan,
            Nic => Self::Nic,
        }
    }
}
