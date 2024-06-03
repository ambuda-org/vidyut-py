Introduction
============

This package defines Python bindings for `Vidyut`_, a high-performance Sanskrit
toolkit written in Rust. It includes modules for segmentation, word generation,
word lookup, and sandhi.

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

    $ wget https://github.com/ambuda-org/vidyut-py/releases/download/0.2.0/data-0.2.0.zip
    $ unzip data-0.2.0.zip

You can use this data like so::

    from vidyut.cheda import Chedaka

    c = Chedaka("data-0.2.0")
    for token in c.run("tapaHsvADyAyaniratam"):
        print(token)


Getting help
------------

To ask for help and file bugs, we encourage you to `create an issue`_ on our
repo on GitHub. For more casual questions, you can also join the `#nlp` channel
on our `Discord`_ server.

.. _`create an issue`: https://github.com/ambuda-org/vidyut-py/issues
.. _Discord: https://discord.gg/7rGdTyWY7Z
