# Checks that the caller is in a virtual environment.
py-venv-check: 
ifeq ("$(VIRTUAL_ENV)","")
	@echo "Error: You are not running within a virtualenv. Please run the following command:"
	@echo
	@echo "> source env/bin/activate"
	@echo
	exit 1
endif

# Initializes the repository for local development.
install:
	python3 -m venv env
	. env/bin/activate && pip install -r requirements.txt
	. env/bin/activate && make develop
	. env/bin/activate && make test
	@echo
	@echo "vidyut-py built successfully."

# Builds the package.
#
# Use this command directly instead of `cargo build`, which will likely fail
# due to missing linker arguments.
build: py-venv-check
	maturin build

# Builds the package and installs it in the local virtualenv.
develop: py-venv-check
	maturin develop

# Runs all Python-specific unit tests.
test: develop
	pytest test/unit/

# NOTE: this command is very slow.
create_data:
	git clone https://github.com/ambuda-org/vidyut.git vidyut-rust
	cd vidyut-rust/vidyut-cheda
	make create_all_data

# Runs integration tests against a data directory.
#
# Usage:
#
# VIDYUT_DATA_DIR="/path/to/your/dir" make integration_tests
integration_tests: develop
	pytest test/integration/

# Lints all Rust and Python code.
lint:
	cargo fmt
	black .

# Generates Sphinx docs.
#
# To view the docs, open `vidyut/docs/build/html/index.html`.
docs: develop
	cd vidyut/docs && make html
