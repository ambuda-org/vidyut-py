vidyut-py
=========

Experimental Python bindings for [Vidyut][vidyut], a lightning-fast Sanskrit toolkit.


Status
------

These bindings are **experimental**. Future changes will likely cause breaking
changes to the API and the data format.


Usage
-----

    from vidyut import Parser

    # For details on what this path should point to, see `Setup` below.
    p = Parser('path/to/my/data')

    # All input must be in SLP1.
    print(p.parse('gacCati'))


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

    # The output data will be in `data`. Once the `data` folder has been
    # created, you can move it wherever you like.
    ls data/

Then, pass the path to this data into the `Parser`:

    p = Parser('path/to/my/data')


[vidyut]: https://github.com/ambuda-org/vidyut
