.. py:currentmodule:: vidyut.chandas

Analyzing meter with `vidyut.chandas`
=====================================

`vidyut.chandas` is an experimental classifier for Sanskrit meters.

All input text should use the `SLP1`_ encoding scheme.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1

Usage::

    from vidyut.chandas import Chandas

    c = Chandas("/path/to/meters.tsv")
    match = c.classify("mAtaH samastajagatAM maDukEwaBAreH")
