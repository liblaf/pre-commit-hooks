HOST != rustc --version --verbose | sed --quiet 's/host: //p'

default: check clippy fmt

check:
	cargo check

clippy:
	cargo clippy --fix --allow-dirty --allow-staged

dist: dist/pch-$(HOST)

fmt: cargo-fmt fmt/Cargo.toml

#####################
# Auxiliary Targets #
#####################

cargo-fmt:
	cargo fmt

dist/pch-$(HOST): target/release/pre-commit-hooks
	@ install -D --no-target-directory --verbose "$<" "$@"

.PHONY: target/release/pre-commit-hooks
target/release/pre-commit-hooks:
	cargo build --release

fmt/Cargo.toml: Cargo.toml
	toml-sort --in-place --all "$<"
	taplo format "$<"
