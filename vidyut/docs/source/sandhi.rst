.. py:currentmodule:: vidyut.sandhi

Handling sandhi with `vidyut.sandhi`
====================================

`vidyut.sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for most use cases.

The main class here is :class:`~vidyut.sandhi.Splitter`, which manages a list
of sandhi rules. The main return type is :class:`~vidyut.splitter.Split`, which
contains information about the split.

`vidyut.sandhi` tends to overgenerate, so we suggest using `vidyut.sandhi` only
as part of a larger system. This is the exact approach we take with
`vidyut.cheda`, which combines these splits with a dictionary and a scoring
model.

Example usage::

    from vidyut.sandhi import Splitter

    splitter = Splitter("/path/to/vidyut-data/sandhi-rules.csv")

    for split in splitter.split("ityapi", 2):
        print(split.first, split.second, split.is_valid)

