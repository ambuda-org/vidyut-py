.. py:currentmodule:: vidyut.prakriya

Generating words with `vidyut.prakriya`
=======================================

`vidyut.prakriya` generates Sanskrit words with their prakriyās (derivations)
according to the rules of Paninian grammar. This powerful word generator is
useful both for creating Sanskrit words and for understanding how those words
are derived. 

All input text should use the `SLP1`_ encoding scheme.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1


Quickstart
----------

All usage of `vidyut.prakriya` follows a basic flow:

1. We create a new :class:`Vyakarana` object, which is our main interface to
   the grammar. When we create :class:`Vyakarana`, we can also fine-tune the
   rules and settings the system will use.

2. We pass various arguments to :class:`Vyakarana` to define our derivation. 

3. We receive a list of :class:`Prakriya` objects, which contain our final
   results and the rules that produced them.

To illustrate, let's derive the simple verb ``Bavati``::

    from vidyut.prakriya import (
        Vyakarana,
        Dhatu,
        Gana,
        Prayoga,
        Purusha,
        Vacana,
        Lakara
    )

    v = Vyakarana()

    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    prakriyas = v.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )

    for prakriya in prakriyas:
        print(prakriya.text)
        print('===================')
        for step in prakriya.history:
            result = ' + '.join(step.result)
            print("{:<10}: {}".format(step.rule, result))

This produces the following output:

.. code-block:: text

    Bavati
    ===================
    1.3.1     : BU
    3.2.123   : BU + la~w
    1.3.2     : BU + la~w
    1.3.3     : BU + la~w
    1.3.9     : BU + l
    1.3.78    : BU + l
    3.4.78    : BU + tip
    1.3.3     : BU + tip
    1.3.9     : BU + ti
    3.4.113   : BU + ti
    3.1.68    : BU + Sap + ti
    1.3.3     : BU + Sap + ti
    1.3.8     : BU + Sap + ti
    1.3.9     : BU + a + ti
    3.4.113   : BU + a + ti
    7.3.84    : Bo + a + ti
    1.4.14    : Bo + a + ti
    6.1.78    : Bav + a + ti
    8.4.68    : Bav + a + ti


Basic methods
-------------

`vidyut.prakriya` currently exposes five basic methods.

The first is `derive_tinantas`, which creates verbs::

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    prakriyas = v.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "Bavati"

The second is `derive_subantas`, which derives nominals::

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

The third is `derive_krdantas`, which adds suffixes to verb roots::

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    prakriyas = v.derive_krdantas(dhatu=bhu, krt=Krt.ktvA)

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "BUtvA"

The fourth is `derive_taddhitantas`, which adds suffixes to nominal stems::

    from vidyut.prakriya import *

    v = Vyakarana()
    guru = Pratipadika(text="guru")
    prakriyas = v.derive_taddhitantas(pratipadika=guru, taddhita=Taddhita.aR)

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "gOrava"

The fifth is `derive_dhatus`, which processes raw dhatus from the Dhatupatha
along with any prefixes and suffixes we might wish to add::

    from vidyut.prakriya import *

    v = Vyakarana()
    upa_r = Dhatu.mula(aupadeshika="f\\", gana=Gana.Bhvadi).with_prefixes(["upa"])
    prakriyas = v.derive_dhatus(upa_r)

    assert len(prakriyas) == 1
    assert prakriyas[0].text == "upAr"


Specifying arguments
--------------------

The real expressive power in this package comes from expressing different kinds
of arguments.

For example, we can modify a `dhatu` argument to use one or more prefixes::

    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    abhibhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi, prefixes=["abhi"])

    # Or, use the shorthand method `with_prefixes`:
    paribhu = bhu.with_prefixes(["abhi"])

Or one or more :class:`Sanadi` suffixes::

    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    bhavaya = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi, sanadi=[Sanadi.Ric])

    # Or, use the shorthand method `with_sanadi`.
    bubhusha = bhu.with_sanadi([Sanadi.san])

Likewise, we can declare a `pratipadika` as coming from a krdanta::

    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)
    bhavaka = Pratipadika.from_krdanta(bhu, Krt.Rvul)

Or from a taddhitanta::

    guru = Pratipadika(text="guru")
    gaurava = Pratipadika.from_taddhitanta(guru, Taddhita.aR)



Using the Dhatupatha
--------------------

Rather than define dhatus manually, it is simpler and easier to use an existing
Dhatupatha::

    from vidyut.prakriya import *

    dhatupatha = Dhatupatha("/path/to/dhatupatha.tsv")
    dhatus = [e.dhatu for e in dhatupatha.entries()]

    v = Vyakarana(log_steps=False)
    for dhatu in dhatus:
        prakriyas = v.derive_tinantas(
            dhatu=dhatu,
            prayoga=Prayoga.Kartari,
            purusha=Purusha.Prathama,
            vacana=Vacana.Eka,
            lakara=Lakara.Lat,
        )
        for prakriya in prakriyas:
            print(prakriya.text)


Recipes
-------

This section contains various recipes that we have found useful.


Generate tinantas for a dhatu and prayoga
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    v = Vyakarana(log_steps=False)
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)

    for lakara in Lakara.choices():
        for purusha in Purusha.choices():
            for vacana in Vacana.choices():
                prakriyas = v.derive_tinantas(
                    dhatu=bhu,
                    prayoga=Prayoga.kartari,
                    purusha=purusha,
                    vacana=vacana,
                    lakara=lakara,
                )
                for p in prakriyas:
                    print(p.text)


Generate all tinantas for some prayoga
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    dhatupatha = Dhatupatha("/path/to/dhatupatha.tsv")
    dhatus = [e.dhatu for e in dhatupatha.entries()]

    v = Vyakarana(log_steps=False)

    for dhatu in dhatus:
        for prayoga in Prayoga.choices():
            for lakara in Lakara.choices():
                for purusha in Purusha.choices():
                    for vacana in Vacana.choices():
                        prakriyas = v.derive_tinantas(
                            dhatu=dhatu,
                            prayoga=prayoga,
                            purusha=purusha,
                            vacana=vacana,
                            lakara=lakara,
                        )
                        for p in prakriyas:
                            print(p.text)


Generate all subantas for some pratipadika
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    v = Vyakarana(log_steps=False)
    nara = Pratipadika(text="nara")

    for vibhakti in Vibhakti.choices():
        for vacana in Vacana.choices():
            prakriyas = v.derive_subantas(
                pratipadika=nara,
                linga=Linga.Pum,
                vibhakti=vibhakti,
                vacana=vacana,
            )
            for p in prakriyas:
                print(vibhakti, vacana, p.text)


Generate all krdantas for a dhatu
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)

    for krt in Krt.choices():
        prakriyas = v.derive_krdantas(dhatu=bhu, krt=krt)
        for p in prakriyas:
            print(krt, p.text)
        else:
            print(f"- (no results for BU + {krt})")


Generate all taddhitantas for a pratipadika
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    v = Vyakarana()
    guru = Pratipadika(text="guru")

    for taddhita in Taddhita.choices():
        prakriyas = v.derive_taddhitantas(pratipadika=guru, taddhita=taddhita)
        for p in prakriyas:
            print(taddhita, p.text)
        else:
            print(f"- (no results for guru + {taddhita})")


Find all rules used by a dhatu's tinantas
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: python

    from vidyut.prakriya import *

    v = Vyakarana()
    bhu = Dhatu.mula(aupadeshika="BU", gana=Gana.Bhvadi)

    rules = set()
    for prayoga in Prayoga.choices():
        for lakara in Lakara.choices():
            for purusha in Purusha.choices():
                for vacana in Vacana.choices():
                    prakriyas = v.derive_tinantas(
                        dhatu=bhu,
                        prayoga=prayoga,
                        purusha=purusha,
                        vacana=vacana,
                        lakara=lakara,
                    )
                    for p in prakriyas:
                        for step in p.history:
                            rules.add(step.rule)

    print(sorted(rules))
