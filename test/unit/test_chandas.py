import tempfile
from pathlib import Path

from vidyut import chandas
from vidyut.chandas import Chandas



def test_init():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"mandAkrAntA	vrtta	GGGG|LLLLLG|GLGGLGG\n")
        assert Path(f.name).exists()

        _ = Chandas(f.name)


def test_init__invalid_file():
    with tempfile.NamedTemporaryFile() as f:
        f.write(b"junk data")
        assert Path(f.name).exists()

        _ = Chandas(f.name)


def test_init__missing_file():
    try:
        _ = Chandas("/tmp/path/to/invalid/file.tsv")
        assert False
    except OSError:
        pass
