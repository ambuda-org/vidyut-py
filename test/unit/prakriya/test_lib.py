import pytest

from vidyut.prakriya import (
    Vyakarana,
    Dhatupatha,
    Krt,
    Lakara,
    Prayoga,
    Pratipadika,
    Linga,
    Vibhakti,
    Purusha,
    Sanadi,
    Step,
    Vacana,
)


v = Vyakarana()
# Path is relative to the project root.
d = Dhatupatha("test/data/test-dhatupatha.tsv")


def test_dhatupatha():
    bhu = d["01.0001"]
    assert bhu.upadesha == "BU"

    kr = d["08.0010"]
    assert kr.upadesha == "qukf\\Y"


def test_prakriya():
    dhatu = d["01.0001"]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    assert len(prakriyas) == 1
    p = prakriyas[0]
    assert p.text == "Bavati"

    # Filter out debugging rules, which have `rule` = "    "
    filtered_steps = [step for step in p.history if step.rule.strip()]

    assert filtered_steps == [
        Step(rule='1.3.1', result=["BU"]),
        Step(rule='3.2.123', result=["BU", "la~w"]),
        Step(rule='1.3.2', result=["BU", "la~w"]),
        Step(rule='1.3.3', result=["BU", "la~w"]),
        Step(rule='1.3.9', result=["BU", "l"]),
        Step(rule='1.3.78', result=["BU", "l"]),
        Step(rule='3.4.78', result=["BU", "tip"]),
        Step(rule='1.3.3', result=["BU", "tip"]),
        Step(rule='1.3.9', result=["BU", "ti"]),
        Step(rule='3.4.113', result=["BU", "ti"]),
        Step(rule='3.1.68', result=["BU", "Sap", "ti"]),
        Step(rule='1.3.3', result=["BU", "Sap", "ti"]),
        Step(rule='1.3.8', result=["BU", "Sap", "ti"]),
        Step(rule='1.3.9', result=["BU", "a", "ti"]),
        Step(rule='3.4.113', result=["BU", "a", "ti"]),
        Step(rule='7.3.84', result=["Bo", "a", "ti"]),
        Step(rule='1.4.14', result=["Bo", "a", "ti"]),
        Step(rule='6.1.78', result=["Bav", "a", "ti"]),
        Step(rule='8.4.68', result=["Bav", "a", "ti"]),
    ]


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "Bavati"),
        ("08.0010", "karoti|kurute"),
    ],
)
def test_basic_kartari_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUyate"),
        ("08.0010", "kriyate"),
    ],
)
def test_basic_karmani_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Karmani,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "buBUzati"),
        # ("08.0010", "cikIrzati"),
    ],
)
def test_sannanta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.san,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BAvayati|BAvayate"),
        ("08.0010", "kArayati|kArayate"),
    ],
)
def test_nijanta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Ric,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "boBUyate"),
        # ("08.0010", "cekrIyate"),
    ],
)
def test_yananta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.yaN,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUtvA"),
        ("08.0010", "kftvA"),
    ],
)
def test_krdantas(code, expected):
    dhatu = d[code]
    prakriyas = v.derive_krdantas(
        dhatu=dhatu,
        krt=Krt.ktvA,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_subantas():
    prakriyas = v.derive_subantas(
        pratipadika=Pratipadika(text="deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )
    expected = {"devaH"}
    actual = {x.text for x in prakriyas}
    assert expected == actual
