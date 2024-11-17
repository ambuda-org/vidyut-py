Introduction
============

This package defines Python bindings for `Vidyut`_, a high-performance Sanskrit
toolkit written in Rust. It contains the following modules:

- `vidyut.chandas` identifies the meter in some piece of Sanskrit text. This
  crate is experimental, and while it is useful for common and basic use cases,
  it is not a state-of-the-art solution.

- `vidyut.cheda` segments Sanskrit expressions into words then annotates those
  words with their morphological data. Our segmenter is optimized for real-time
  and interactive usage: it is fast, low-memory, and capably handles
  pathological input.

- `vidyut.kosha` defines a key-value store that can compactly map hundreds of
  millions of Sanskrit words to their inflectional data. Depending on the
  application, storage costs can be as low as 1 byte per word. This storage
  efficiency comes at the cost of increased lookup time, but in practice, we
  have found that this increase is negligible and well worth the efficiency
  gains elsewhere.

- `vidyut.lipi` is a transliteration library for Sanskrit and Pali that also
  supports many of the scripts used within the Indosphere. Our goal is to
  provide a standard transliterator that is easy to bind to other programming
  languages.

- `vidyut.prakriya` generates Sanskrit words with their *prakriyā*s
  (derivations) according to the rules of Paninian grammar. Our long-term goal
  is to provide a complete implementation of the Ashtadhyayi.

- `vidyut.sandhi` contains various utilities for working with sandhi changes
  between words. It is fast, simple, and appropriate for most use cases.

Our Python API is lightweight and mirrors the underlying Rust API, with minor
change to be more Pythonic.

.. _Vidyut: https://github.com/ambuda-org/vidyut


System requirements
-------------------

This package requires Python 3.7 or newer. You can check your Python
installation by running the following command:

.. code-block:: text

    $ python3 --version


Installation
------------

Our Python bindings are published under the `vidyut` package on PyPI and do not
require a Rust installation. You can install the `vidyut` package like so:

.. code-block:: text

    $ pip install vidyut

Vidyut is more interesting when used with our rich linguistic data, which you
can download here:

.. code-block:: text

    $ wget https://github.com/ambuda-org/vidyut-py/releases/download/0.3.0/data-0.3.0.zip
    $ unzip data-0.3.0.zip

You can use this data like so::

    from vidyut.cheda import Chedaka

    c = Chedaka("data-0.3.0")
    for token in c.run("tapaHsvADyAyaniratam"):
        print(token)


Getting help
------------

To ask for help and file bugs, we encourage you to `create an issue`_ on our
repo on GitHub. For more casual questions, you can also join the `#nlp` channel
on our `Discord`_ server.

.. _`create an issue`: https://github.com/ambuda-org/vidyut-py/issues
.. _Discord: https://discord.gg/7rGdTyWY7Z
