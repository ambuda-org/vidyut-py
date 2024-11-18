import pytest

from vidyut.prakriya import (
    Dhatu,
    Pratipadika,
    Antargana,
    Gana,
    Krt,
    Lakara,
    Linga,
    Prayoga,
    Purusha,
    Sanadi,
    Taddhita,
    Vacana,
    Vibhakti,
)


@pytest.mark.parametrize(
    "kwargs",
    [
        {"aupadeshika": "BU", "gana": Gana.Bhvadi},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "antargana": Antargana.Akusmiya},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "prefixes": ["aBi"]},
        {"aupadeshika": "BU", "gana": Gana.Bhvadi, "sanadi": [Sanadi.san]},
        {
            "aupadeshika": "BU",
            "gana": Gana.Bhvadi,
            "prefixes": ["aBi"],
            "sanadi": [Sanadi.san],
        },
    ],
)
def test_dhatu__mula(kwargs):
    d = Dhatu.mula(**kwargs)
    for key, value in kwargs.items():
        assert getattr(d, key) == value


def test_dhatu__mula_must_have_aupadeshika_and_gana():
    try:
        d = Dhatu.mula(aupadeshika="BU")
        assert False, "Must have gana"
    except TypeError:
        pass

    try:
        d = Dhatu.mula(gana=Gana.Bhvadi)
        assert False, "Must have aupadeshika"
    except TypeError:
        pass


def test_dhatu__nama():
    d = Dhatu.nama(pratipadika=Pratipadika(text="putra"), nama_sanadi=Sanadi.kAmyac)


@pytest.mark.skip("need to fix pratipadika internal representation")
def test_pratipadika_new():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        _p = Pratipadika()


def test_pratipadika_new__fails_if_unnamed_args():
    with pytest.raises(TypeError):
        _p = Pratipadika("deva")


def test_pratipadika_eq():
    p1 = Pratipadika(text="deva")
    assert p1 == p1

    p2 = Pratipadika(text="deva")
    assert p1 == p2

    p3 = Pratipadika(text="eva")
    assert p1 != p3


@pytest.mark.parametrize(
    "enum",
    [
        Antargana,
        Gana,
        Krt,
        Lakara,
        Linga,
        Prayoga,
        Purusha,
        Sanadi,
        Taddhita,
        Vacana,
        Vibhakti,
    ],
)
def test_enums(enum):
    # Enum values can round-trip through strings
    for val in enum.choices():
        assert enum.from_string(str(val)) == val

    # Enum values are hashable.
    for val in enum.choices():
        _data = {val: "foo"}
