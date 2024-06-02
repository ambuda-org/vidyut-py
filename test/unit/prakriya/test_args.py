import pytest

from vidyut.prakriya import (
    Pratipadika,
    Lakara,
    Linga,
    Prayoga,
    Vibhakti,
)


@pytest.mark.parametrize(
    "variant",
    [
        Lakara.Lat,
        Linga.Pum,
        Prayoga.Kartari,
        Vibhakti.Prathama,
    ],
)
def test_enum__hashable(variant):
    """Verify that our enums are hashable."""
    data = {variant: "foo"}


@pytest.mark.parametrize(
    "enum,variant,s",
    [
        (Lakara, Lakara.Lat, "lat"),
        (Linga, Linga.Pum, "pum"),
        (Prayoga, Prayoga.Kartari, "kartari"),
        (Vibhakti, Vibhakti.Prathama, "1"),
    ],
)
def test_enum__str(enum, variant, s):
    """Verify that our enums can round-trip to strings."""
    assert str(variant) == s
    assert enum.from_string(s) == variant


@pytest.mark.skip("need to fix pratipadika internal representation")
def test_pratipadika_new():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        p = Pratipadika()


def test_pratipadika_new__fails_if_unnamed_args():
    with pytest.raises(TypeError):
        p = Pratipadika("deva")


def test_pratipadika_eq():
    p1 = Pratipadika(text="deva")
    assert p1 == p1

    p2 = Pratipadika(text="deva")
    assert p1 == p2

    p2 = Pratipadika(text="eva")
    assert p1 != p2
