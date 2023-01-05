<div align="center">
<h1><code>vidyut-py</code></h1>
<p><i>Python bindings for Vidyut</i></p>
</div>

`vidyut-py` defines Python bindings for [Vidyut][vidyut], a high-performance
Sanskrit toolkit. These are the same bindings we use for our work on
[Ambuda][ambuda], which is written primarily in Python.


- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Data](#data)

[vidyut]: https://github.com/ambuda-org/vidyut
[ambuda]: https://ambuda.org


Overview
--------

Vidyut, our high-performance Sanskrit toolkit, is implemented in Rust. Rust is
a wonderful language, but it is not right for all scenarios, and it is not a
language that many programmers know already. `vidyut-py` provides friendly
Python bindings on top of Rust so that you can use Vidyut more easily.

In general, our Python API is lightweight and mirrors
the underlying Rust API, with minor change to be more Pythonic.


Installation
------------

Our Python bindings are published under the `vidyut` package on PyPI and do not
require a Rust installation. You can install them like so:

```python
$ pip install vidyut
```

Usage
-----

Using `vidyut-cheda`:

```python
from vidyut import Chedaka

# For details on what this path should point to, see `Setup` below.
s = Segmenter('/path/to/vidyut-0.1.0')

# All input must be in SLP1.
print(s.segment('gacCati'))
```

Using `vidyut-kosha`:

```python
from vidyut import Kosha
```

Using `vidyut-prakriya`:

```python
from vidyut import Ashtadhyayi
```

For details, run `make docs`.


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
