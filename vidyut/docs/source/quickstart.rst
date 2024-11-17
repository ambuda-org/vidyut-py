Quickstart
==========

This sections contains various examples of how to use Vidyut's core libraries.
In general, all of Vidyut's input text should use the `SLP1`_ encoding scheme.

The examples below assume that your linguistic data is saved at `/path/to/vidyut-data`.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1


Generating words
----------------

`vidyut.prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar. Our long-term goal is to provide a
complete implementation of the Ashtadhyayi.

The main class here is :class:`~vidyut.prakriya.Ashtadhyayi`, which defines an
interface to the Ashtadhyayi's rules. The main return type is
:class:`~vidyut.prakriya.Prakriya`, which contains both the final result and
the full derivation that produced it.


Main methods
~~~~~~~~~~~~

:class:`Ashtadhyayi` currently exposes three high-level functions. First,
`derive_tinantas` derives verbs::

    from vidyut.prakriya import (
        Vyakarana,
        Dhatupatha,
        Prayoga,
        Purusha,
        Vacana,
        Lakara
    )

    v = Vyakarana()
    dhatupatha = Dhatupatha("/path/to/vidyut-data/prakriya/dhatupatha.tsv")
    dhatus = {e.code: e for in dhatupatha.entries()}

    # Read "BU" from the dhatupatha.
    #
    # These dhatu codes are the same as the codes used on ashtadhyayi.com.
    dhatu = dhatus["01.0001"]

    prakriyas = v.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "Bavati"

Second, `derive_krdantas` derives verbal adjectives and verbal indeclinables::

    from vidyut.prakriya import Krt

    # Read "kf" from the dhatupatha.
    dhatu = d["08.0010"]

    prakriyas = a.derive_krdantas(
        dhatu=dhatu,
        krt=Krt.ktvA,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "kftvA"

Third, `derive_subantas` derives nominals::

    from vidyut.prakriya import (
        Pratipadika,
        Linga,
        Vibhakti,
    )

    prakriyas = a.derive_subantas(
        pratipadika=Pratipadika(text="deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "devaH"

Methods that accept a `dhatu` argument can also accept a
:class:`~vidyut.prakriya.Sanadi` argument::

    from vidyut.prakriya import Yan

    dhatu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Yan,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "boBUyate"


Using the `Prakriya` type
~~~~~~~~~~~~~~~~~~~~~~~~~

:class:`~vidyut.prakriya.Prakriya` exposes both the final word and the full
derivation that produced that word. For example, the following code::

    dhatu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )

    for p in prakriyas:
        print(p.text)
        print("=" * 20)
        for step in p.history:
            print("{:>10}: {}".format(step.rule, step.result))

produces the following output:

.. code-block:: text

    Bavati
    ====================
    1.3.1   : BU
    3.3.123 : BU + la~w
    1.3.2   : BU + la~w
    1.3.3   : BU + la~w
    1.3.9   : BU + l
    1.3.78  : BU + l
    3.4.78  : BU + tip
    1.3.3   : BU + tip
    1.3.9   : BU + ti
    3.4.113 : BU + ti
    3.1.68  : BU + Sap + ti
    1.3.3   : BU + Sap + ti
    1.3.8   : BU + Sap + ti
    1.3.9   : BU + a + ti
    3.4.113 : BU + a + ti
    7.3.84  : Bo + a + ti
    6.1.78  : Bav + a + ti


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
