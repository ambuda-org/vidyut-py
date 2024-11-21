import os
import pytest
from pathlib import Path

from vidyut.prakriya import Data, Entry, Sutra, Dhatu, Gana, Source


@pytest.fixture(scope="module")
def data() -> Data:
    return Data(Path(os.environ["VIDYUT_DATA_DIR"]) / "prakriya")


@pytest.fixture(scope="module")
def all_sutras(data):
    return data.load_sutras()


def test_dhatupatha(data):
    entries = data.load_dhatu_entries()
    assert len(entries) == 2229
    assert entries[0] == Entry(
        code="01.0001", dhatu=Dhatu.mula("BU", Gana.Bhvadi), artha="sattAyAm"
    )


def test_dhatupatha_ganasutras(all_sutras):
    sutras = [s for s in all_sutras if s.source == Source.Dhatupatha]
    assert len(sutras) == 18
    assert sutras[0] == Sutra(
        source=Source.Dhatupatha, code="01.0933", text="GawAdayo mitaH"
    )


def test_sutrapatha(all_sutras):
    sutras = [s for s in all_sutras if s.source == Source.Ashtadhyayi]
    assert len(sutras) == 3983
    assert sutras[0] == Sutra(
        source=Source.Ashtadhyayi, code="1.1.1", text="vfdDirAdEc"
    )


def test_unadipatha(all_sutras):
    sutras = [s for s in all_sutras if s.source == Source.Unadipatha]
    assert len(sutras) == 748
    assert sutras[0] == Sutra(
        source=Source.Unadipatha, code="1.1", text="kfvApAjimisvadisADyaSUBya uR"
    )


def test_varttikas(all_sutras):
    sutras = [s for s in all_sutras if s.source == Source.Varttika]
    assert len(sutras) == 94
    assert sutras[0] == Sutra(
        source=Source.Varttika,
        code="1.1.33.1",
        text="viBAzAprakaraRe tIyasya NitsUpasaNKyAnam",
    )


def test_sutra_repr(all_sutras):
    for s in all_sutras:
        assert eval(repr(s)) == s
