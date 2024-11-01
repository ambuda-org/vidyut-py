import os

import pytest

from vidyut import chandas
from vidyut.chandas import Chandas


@pytest.fixture(scope="module")
def chandas() -> Chandas:
    path = os.environ["VIDYUT_DATA_DIR"]
    chandas_path = path / "chandas" / "meters.tsv"
    assert chandas_path.exists()
    return Chandas(chandas_path)


@pytest.mark.parametrize("input,expected", [("mo mo go go vidyunmAlA", "vidyunmAlA")])
def test_vrttas(chandas, input, expected):
    assert chandas.classify(input) == expected
