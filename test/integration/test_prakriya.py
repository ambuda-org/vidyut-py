import os
import pytest
from pathlib import Path

from vidyut.prakriya import Data, Entry, Sutra, Dhatu, Gana


@pytest.fixture(scope="module")
def data() -> Data:
    return Data(Path(os.environ["VIDYUT_DATA_DIR"]) / "prakriya")


def test_dhatupatha(data):
    entries = data.load_dhatupatha()

    assert len(entries) == 2229
    assert entries[0] == Entry(code="01.0001", dhatu=Dhatu.mula("BU", Gana.Bhvadi), artha="sattAyAm")


def test_sutrapatha(data):
    sutras = data.load_sutrapatha()

    assert len(sutras) == 3983
    assert sutras[0] == Sutra(code="1.1.1", text="vfdDirAdEc")

    for s in sutras:
        assert eval(repr(s)) == s


def test_unadipatha(data):
    sutras = data.load_unadipatha()

    assert len(sutras) == 748
    assert sutras[0] == Sutra(code="1.1", text="kfvApAjimisvadisADyaSUBya uR")

    for s in sutras:
        assert eval(repr(s)) == s
