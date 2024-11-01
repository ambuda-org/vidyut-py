import tempfile

import pytest


from vidyut.kosha import (
    Builder,
    Kosha,
    Purusha,
    Vacana,
    Pada,
    PadaPrayoga,
    Pratipadika,
    Lakara,
    Linga,
    Vibhakti,
    Dhatu,
)


@pytest.fixture(scope="session")
def kosha():
    """Create a sample Kosha."""
    gacchati_tin = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    gacchati_sup = Pada.make_subanta(
        pratipadika=Pratipadika(text="gam"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    with tempfile.TemporaryDirectory() as tempdir:
        b = Builder(tempdir)
        b.insert("gacCati", gacchati_tin)
        b.insert("gacCati", gacchati_sup)
        b.finish()

        return Kosha(tempdir)


def test_init(kosha):
    assert kosha


def test_init_fails_if_file_does_not_exist():
    with pytest.raises(OSError):
        with tempfile.TemporaryDirectory() as tempdir:
            _kosha = Kosha(tempdir)


def test_contains(kosha):
    assert "Bavati" not in kosha
    assert "gacCati" in kosha


def test_contains_prefix(kosha):
    for prefix in ["", "g", "ga", "gac", "gacC", "gacCa", "gacCat", "gacCati"]:
        assert kosha.contains_prefix(prefix)

    assert not kosha.contains_prefix("gacCati2")


def test_get_all(kosha):
    gacchati_tin = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    gacchati_sup = Pada.make_subanta(
        pratipadika=Pratipadika(text="gam"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    assert len(kosha.get_all("Bavati")) == 0
    assert len(kosha.get_all("gacCati")) == 2

    [tin, sup] = kosha.get_all("gacCati")
    assert tin == gacchati_tin
    assert sup == gacchati_sup
