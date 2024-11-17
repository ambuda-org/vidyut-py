Quickstart
==========

This sections contains various examples of how to use Vidyut's core libraries.
In general, all of Vidyut's input text should use the `SLP1`_ encoding scheme.

The examples below assume that your linguistic data is saved at `/path/to/vidyut-data`.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1


Storing words 
-------------

`vidyut.kosha` defines a key-value store that can compactly map tens of
millions of Sanskrit words to their inflectional data. Depending on the
application, storage costs can be as low as 1 byte per word. This storage
efficiency comes at the cost of increased lookup time, but in practice, we have
found that this increase is negligible and well worth the efficiency gains
elsewhere.

The main class here is :class:`~vidyut.kosha.Kosha`, which defines an interface
to the underlying dictionary data. The main return type is
:class:`~vidyut.kosha.Pada`, which defines rich morphological data about the
given word.

Example usage::

    from vidyut.kosha import Kosha

    kosha = Kosha("/path/to/vidyut-data/kosha")

    for entry in kosha.get_all("gacCati"):
        print(entry)

    # `Kosha` also provides fast existence checks ...
    assert "gacCati" in kosha

    # ... and fast prefix checks.
    assert kosha.contains_prefix("gacCat")


Segmenting and tagging
----------------------

`vidyut.cheda` segments Sanskrit expressions into words then annotates those
words with their morphological data. Our segmenter is optimized for real-time
and interactive usage: it is fast, low-memory, and capably handles pathological
input.

The main class here is :class:`~vidyut.cheda.Chedaka`, which defines a
segmenter. The main return type is :class:`~vidyut.cheda.Token`, which contains
the segmented text with its associated :class:`~vidyut.kosha.Pada` data.

Example usage::

    from vidyut.cheda import Chedaka

    chedaka = Chedaka("/path/to/vidyut-data")

    for token in chedaka.run('gacCati'):
        print(token.text, token.info)


Working with sandhi
-------------------

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
