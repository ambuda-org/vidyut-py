.. py:currentmodule:: vidyut.kosha

Storing words with `vidyut.kosha`
=================================

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



