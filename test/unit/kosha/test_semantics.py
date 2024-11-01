import pytest


from vidyut.kosha import (
    Purusha,
    Vacana,
    Pada,
    PadaPrayoga,
    Pratipadika,
    PartOfSpeech,
    Lakara,
    Linga,
    Vibhakti,
    Dhatu,
)


@pytest.mark.parametrize(
    "variant",
    [
        Lakara.Lat,
        Linga.Pum,
        PadaPrayoga.Parasmaipada,
        PartOfSpeech.Tinanta,
        Vibhakti.V1,
    ],
)
def test_enum__hashable(variant):
    """Verify that our enums are hashable."""
    _data = {variant: "foo"}


@pytest.mark.parametrize(
    "enum,variant,s",
    [
        (Lakara, Lakara.Lat, "lat"),
        (Linga, Linga.Pum, "m"),
        (PadaPrayoga, PadaPrayoga.Parasmaipada, "para"),
        (PartOfSpeech, PartOfSpeech.Tinanta, "t"),
        (Vibhakti, Vibhakti.V1, "1"),
    ],
)
def test_enum__str(enum, variant, s):
    """Verify that our enums can round-trip to strings."""
    assert str(variant) == s
    assert enum.from_string(s) == variant


def test_dhatu():
    gam = Dhatu(text="gam")
    assert gam.text == "gam"


def test_dhatu__eq():
    gam = Dhatu(text="gam")
    assert gam == gam
    assert gam != Dhatu(text="BU")


def test_dhatu__repr():
    dhatu = Dhatu(text="gam")
    assert repr(dhatu) == "Dhatu(text='gam')"
    assert str(dhatu) == "gam"


def test_pratipadika():
    deva = Pratipadika(text="deva")
    assert deva.text == "deva"


def test_pratipadika__eq():
    deva = Pratipadika(text="deva")
    assert deva == deva
    assert deva != Pratipadika(text="svarga")


def test_pratipadika__repr():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"
    assert str(p) == "deva"

    assert eval(repr(p)) == p


def test_pada__make_tinanta():
    p = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    assert p.pos == PartOfSpeech.Tinanta
    assert p.dhatu == Dhatu(text="gam")
    assert p.purusha == Purusha.Prathama
    assert p.lakara == Lakara.Lat
    assert p.vacana == Vacana.Eka
    assert p.pada_prayoga == PadaPrayoga.Parasmaipada
    assert p.linga is None
    assert p.vibhakti is None

    assert repr(p) == (
        "Pada(pos=PartOfSpeech.Tinanta, dhatu=Dhatu(text='gam'), "
        "purusha=Purusha.Prathama, vacana=Vacana.Eka, lakara=Lakara.Lat, "
        "pada_prayoga=PadaPrayoga.Parasmaipada)"
    )


def test_pada__make_tinanta__swapped_args():
    """Confirm that we can create a tinanta with args reordered."""
    p = Pada.make_tinanta(
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        dhatu=Dhatu(text="gam"),
    )

    assert p.pos == PartOfSpeech.Tinanta
    assert p.dhatu == Dhatu(text="gam")
    assert p.purusha == Purusha.Prathama
    assert p.lakara == Lakara.Lat
    assert p.vacana == Vacana.Eka
    assert p.pada_prayoga == PadaPrayoga.Parasmaipada
    assert p.linga is None
    assert p.vibhakti is None


def test_pada__make_tinanta__missing_args():
    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            # dhatu=Dhatu(text="gam"),
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            pada_prayoga=PadaPrayoga.Parasmaipada,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            dhatu=Dhatu(text="gam"),
            # purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            pada_prayoga=PadaPrayoga.Parasmaipada,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            dhatu=Dhatu(text="gam"),
            purusha=Purusha.Prathama,
            # vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            pada_prayoga=PadaPrayoga.Parasmaipada,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            dhatu=Dhatu(text="gam"),
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            # lakara=Lakara.Lat,
            pada_prayoga=PadaPrayoga.Parasmaipada,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            dhatu=Dhatu(text="gam"),
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
            # pada_prayoga=PadaPrayoga.Parasmaipada,
        )


def test_pada__make_tinanta__invalid_args():
    with pytest.raises(TypeError):
        _p = Pada.make_tinanta(
            dhatu=Dhatu(text="gam"),
            purusha=Purusha.Prathama,
            vacana="invalid vacana",
            lakara=Lakara.Lat,
            pada_prayoga=PadaPrayoga.Parasmaipada,
        )


def test_pada__make_subanta():
    p = Pada.make_subanta(
        pratipadika=Pratipadika(text="deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    assert p.pos == PartOfSpeech.Subanta
    assert p.pratipadika == Pratipadika(text="deva")
    assert p.linga == Linga.Pum
    assert p.vacana == Vacana.Eka
    assert p.vibhakti == Vibhakti.V7
    assert p.is_purvapada is False
    assert p.purusha is None
    assert p.lakara is None

    assert repr(p) == (
        "Pada(pos=PartOfSpeech.Subanta, pratipadika=Pratipadika(text='deva'), "
        "linga=Linga.Pum, vibhakti=Vibhakti.V7, vacana=Vacana.Eka, is_purvapada=False)"
    )


def test_pada__make_subanta__missing_args():
    with pytest.raises(TypeError):
        _p = Pada.make_subanta(
            # pratipadika=Pratipadika(text="deva"),
            linga=Linga.Pum,
            vibhakti=Vibhakti.V7,
            vacana=Vacana.Eka,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_subanta(
            pratipadika=Pratipadika(text="deva"),
            # linga=Linga.Pum,
            vibhakti=Vibhakti.V7,
            vacana=Vacana.Eka,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_subanta(
            pratipadika=Pratipadika(text="deva"),
            linga=Linga.Pum,
            # vibhakti=Vibhakti.V7,
            vacana=Vacana.Eka,
        )

    with pytest.raises(TypeError):
        _p = Pada.make_subanta(
            pratipadika=Pratipadika(text="deva"),
            linga=Linga.Pum,
            vibhakti=Vibhakti.V7,
            # vacana=Vacana.Eka,
        )


def test_pada__make_avyaya():
    p = Pada.make_avyaya(
        pratipadika=Pratipadika(text="ca"),
    )

    assert p.pos == PartOfSpeech.Avyaya
    assert p.pratipadika == Pratipadika(text="ca")
    assert p.linga is None
    assert p.vacana is None
    assert p.vibhakti is None
    assert p.purusha is None
    assert p.lakara is None
    assert p.is_purvapada is False

    assert repr(p) == (
        "Pada(pos=PartOfSpeech.Avyaya, pratipadika=Pratipadika(text='ca'), "
        "is_purvapada=False)"
    )


def test_pada__make_avyaya__missing_args():
    with pytest.raises(TypeError):
        Pada.make_avyaya(
            # pratipadika=Pratipadika(text="ca"),
        )
