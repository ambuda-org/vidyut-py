vidyut-py
=========

Experimental Python bindings for [Vidyut][vidyut], a lightning-fast Sanskrit toolkit.


Status
------

These bindings are **experimental**. Future changes will likely cause breaking
changes to the API and the data format.


Usage
-----

    from vidyut import Segmenter

    # For details on what this path should point to, see `Setup` below.
    s = Segmenter('/path/to/vidyut-0.1.0')

    # All input must be in SLP1.
    print(s.segment('gacCati'))


Setup
-----

*(Requires Rust's `cargo` command)*

Currently, `vidyut-py` does not include any linguistic data. To use Vidyut, you
must build this linguistic data manually.

To build this data, please use the main [Vidyut][vidyut] repo as follows.

    # Build our linguistic data by using the main `vidyut` repo.
    git clone git@github.com:ambuda-org/vidyut.git
    cd vidyut
    make install

    # The output data will be in `data/vidyut-x.y.z`, where `x.y.z` is the Vidyut version.
    # Once the `data` folder has been # created, you can move it wherever you like.
    ls data/vidyut-0.1.0/

Then, pass the path to this data into the `Segmenter`:

    s = Segmenter('path/to/vidyut-0.1.0')


[vidyut]: https://github.com/ambuda-org/vidyut
