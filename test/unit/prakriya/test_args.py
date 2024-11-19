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

    assert d.aupadeshika == kwargs["aupadeshika"]
    assert d.gana == kwargs["gana"]
    assert d.antargana == kwargs.get("antargana")
    assert d.prefixes == kwargs.get("prefixes", [])
    assert d.sanadi == kwargs.get("sanadi", [])


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
    _ = Dhatu.nama(pratipadika=Pratipadika(text="putra"), nama_sanadi=Sanadi.kAmyac)
    _ = Dhatu.nama(pratipadika=Pratipadika(text="putra"))


def test_dhatu__nama_must_have_pratipadika():
    try:
        d = Dhatu.nama(nama_sanadi=Sanadi.kAmyac)
        assert False, "Must have pratipadika"
    except TypeError:
        pass


def test_dhatu__dunders():
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    gam = Dhatu.mula(aupadeshika="ga\\mx~", gana=Gana.Bhvadi)
    kut = Dhatu.mula(aupadeshika="kuwa~", gana=Gana.Tudadi, antargana=Antargana.Kutadi)
    abhibubhusha = bhu.with_prefixes(["aBi"]).with_sanadi([Sanadi.san])

    # __eq__, __ne__
    bhu2 = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    assert bhu == bhu
    assert bhu == bhu2
    assert bhu != gam

    # __lt__, __gt__
    _ = sorted([bhu, gam])

    # __repr__
    assert repr(bhu) == "Dhatu(aupadeshika='BU', gana=Gana.Bhvadi)"
    assert repr(kut) == "Dhatu(aupadeshika='kuwa~', gana=Gana.Tudadi, antargana=Antargana.Kutadi)"
    assert repr(abhibubhusha) == "Dhatu(aupadeshika='BU', gana=Gana.Bhvadi, prefixes=['aBi'], sanadi=[Sanadi.san])"


@pytest.mark.skip("need to fix pratipadika internal representation")
def test_pratipadika_new():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        _p = Pratipadika()


def test_pratipadika__dunders():
    deva = Pratipadika(text="deva")
    eva = Pratipadika(text="eva")

    # __eq__, __ne__
    deva2 = Pratipadika(text="deva")
    assert deva == deva
    assert deva == deva2
    assert deva != eva

    # __lt__, __gt__
    _ = sorted([deva, eva])

    # __repr__
    assert repr(deva) == "Pratipadika(text='deva')"


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
    # Enum values can round-trip through strings.
    for val in enum.choices():
        assert enum.from_string(str(val)) == val

    # Enum values can be ordered.
    _ = sorted(enum.choices())

    # Enum values are already in sorted order.
    assert enum.choices()
    assert sorted(enum.choices()) == enum.choices()

    # Enum values set the `name` attribute.
    for val in enum.choices():
        assert getattr(enum, val.name) == val

    # `__repr__` can be used to create an enum.
    for val in enum.choices():
        assert eval(repr(val)) == val

    # Enum values are hashable.
    _ = {val: "foo" for val in enum.choices()}
