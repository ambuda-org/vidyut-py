import pytest

from vidyut.lipi import transliterate, detect, Scheme


def test_scheme():
    # Verify that round-tripping through strings works.
    for val in Scheme.choices():
        assert Scheme.from_string(str(val)) == val

    # Verify that enums are hashable.
    for val in Scheme.choices():
        _data = {val: "foo"}


@pytest.mark.parametrize(
    "input,source,dest,expected",
    [
        ("saMskRtam", Scheme.HarvardKyoto, Scheme.Devanagari, "संस्कृतम्"),
    ],
)
def test_transliterate(input, source, dest, expected):
    assert transliterate(input, source, dest) == expected


@pytest.mark.parametrize(
    "text,expected",
    [
        ("", Scheme.HarvardKyoto),
        ("संस्कृतम्", Scheme.Devanagari),
    ],
)
def test_detect(text, expected):
    assert detect(text) == expected
