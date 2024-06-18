run: build
	python run_sim.py

test: build
	python -m pytest

# This assumes you have a virtual environment with the correct Python version and uv installed
build:
	cd cp_controller && maturin build
	uv pip install -r requirements.txt
