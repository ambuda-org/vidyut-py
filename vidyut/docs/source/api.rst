API
===

This page defines an API reference for the `vidyut` package.

The tools we used to generate this page are optimized for pure Python modules.
But since this package is not pure Python, these tools occasionally have issues
when processing our code. Known issues:

- Enum members don't have docstrings.
- Special methods (`__new__`, ...) don't have docstrings.
- Function docs don't have argument types or return types.
- Properties and attributes don't have types.

We will fix these issues as soon as we can. In the meantime, please refer to
the extensive examples in :doc:`quickstart`.


`vidyut.cheda`
--------------

.. automodule:: vidyut.cheda
    :members: Chedaka, Token


`vidyut.kosha`
--------------

Consumer types
~~~~~~~~~~~~~~

.. autoclass:: vidyut.kosha.Kosha
   :members:
   :special-members: __new__, __contains__
   :undoc-members:

.. autoclass:: vidyut.kosha.Pada
   :members:
   :undoc-members:


Builder API
~~~~~~~~~~~

.. autoclass:: vidyut.kosha.Builder
   :members:
   :undoc-members:


`vidyut.lipi`
--------------

.. automodule:: vidyut.lipi
    :members: Scheme, detect, transliterate


`vidyut.prakriya`
-----------------

Main types
~~~~~~~~~~

*Main types* define the high-level API.

.. autoclass:: vidyut.prakriya.Vyakarana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Dhatupatha
   :members:
   :undoc-members:


Input types
~~~~~~~~~~~

*Input types* define the input arguments for our API.

Names of suffixes, including the members of `Krt` and `Sanadi`, are written in
the `SLP1`_ encoding scheme.

.. _SLP1: https://en.wikipedia.org/wiki/SLP1

.. autoclass:: vidyut.prakriya.Dhatu
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Pratipadika
   :members:
   :undoc-members:

.. autoclass:: vidyut.prakriya.Gana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Krt
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Lakara
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Linga
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Prayoga
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Purusha
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Sanadi
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Vacana
   :members:
   :undoc-members:
.. autoclass:: vidyut.prakriya.Vibhakti
   :members:
   :undoc-members:


Output types
~~~~~~~~~~~~

*Output types* define the output format of our API.

.. autoclass:: vidyut.prakriya.Prakriya
   :members:
   :undoc-members:


`vidyut.sandhi`
---------------

.. automodule:: vidyut.sandhi
    :members: Split, Splitter
