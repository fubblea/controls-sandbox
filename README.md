# Controls Sandbox

## About

Various experiments that are controlled through the [Gymnasium](https://gymnasium.farama.org/) environment.

The controllers for the experiments are written in Rust and then build using [Maturin](https://www.maturin.rs/) to be run in a Python script.

## Usage

1. Ensure you have Python >=3.12 installed
2. Install [`uv`](https://astral.sh/blog/uv): `pip install uv`
3. Change directory to any experiment (e.g. `cd cart-pend`)
4. Create a virtual environment: `uv venv` and activate it
5. Run the Makefile script: `make run`
