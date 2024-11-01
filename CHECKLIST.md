Deployment checklist
====================

Version number:

- Increase version number in various files:
    - `pyproject.toml`.
    - `vidyut/docs/source/conf.py`
- Create changelog entry in CHANGES.rst. Use a temporary release date if
  necessary.
- Grep for old version and confirm it exists only in comments and CHANGES.rst.

Data (if a major release):

- `make create_data` passes
- Data directory exists with release version.
  (`zip -r data-VERSION.zip data-VERSION/`)

Quality:

- `make test` passes
- `make integration_tests` passes
- `make lint-check` passes
- `make docs` passes

Finalize:

- Update release date in CHANGES.rst
- Tag commit with new VERSION.
- Run `maturin publish`.
