rust_format:
	cargo fmt

rust_run:
	cargo run

rust_lint:
	cargo clippy

rust_test:
	cargo test

rust_check:
	cargo check

release:
	cargo build --release

all: rust_format rust_lint rust_test rust_run






install:
	pip install --upgrade pip && pip install -r requirements.txt

format: 
	black *.py

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py

test: 
	python -m pytest -cov=main test_main.py

all: install format lint test
