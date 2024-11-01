# import os
# import pytest
# from pathlib import Path

# from vidyut.prakriya import Dhatupatha


# @pytest.fixture(scope="module")
# def dhatupatha() -> Dhatupatha:
#     path = Path(os.environ["VIDYUT_DATA_DIR"])
#     assert path
#     return Dhatupatha(path / "prakriya" / "dhatupatha.tsv")
#
#
#     token = entries[0]
#     assert token.info.pos is not None
#
#
# def test_dhatupatha(dhatupatha):
#     # OK -- loaded successfully.
#     pass
