rust_format:
	cargo fmt --manifest-path ./rustsql/Cargo.toml

rust_run:
	cargo run --manifest-path ./rustsql/Cargo.toml

rust_lint:
	cargo clippy --manifest-path ./rustsql/Cargo.toml

rust_test:
	cargo test --manifest-path ./rustsql/Cargo.toml

rust_check:
	cargo check --manifest-path ./rustsql/Cargo.toml

release:
	cargo build --release --manifest-path ./rustsql/Cargo.toml

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
