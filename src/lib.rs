use pyo3::prelude::*;

mod cheda;
mod kosha;
mod prakriya;

/// Vidyut: a lightning-fast Sanskrit toolkit.
///
/// Vidyut supports only SLP1 transliteration.
#[pymodule]
fn vidyut(_py: Python, m: &PyModule) -> PyResult<()> {
    // vidyut-cheda
    m.add_class::<cheda::PyChedaka>()?;

    // vidyut-kosha
    m.add_class::<kosha::PyKosha>()?;

    // vidyut-prakriya
    m.add_class::<prakriya::PyAshtadhyayi>()?;
    m.add_class::<prakriya::PyDhatupatha>()?;
    m.add_class::<prakriya::args::Prayoga>()?;
    m.add_class::<prakriya::args::Purusha>()?;
    m.add_class::<prakriya::args::Vacana>()?;
    m.add_class::<prakriya::args::Lakara>()?;
    m.add_class::<prakriya::args::Sanadi>()?;

    Ok(())
}
