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
	. env/bin/activate && pytest
	@echo
	@echo "vidyut-py built successfully."

# Builds the package.
#
# Use this command directly instead of `cargo build`, which will likely fail
# due to missing linker arguments.
build: py-venv-check
	maturin build

develop: py-venv-check
	maturin develop

test: develop
	pytest
