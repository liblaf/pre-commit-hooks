HOST != rustc --version --verbose | sed --quiet 's/host: //p'

default: check clippy fmt

check:
	cargo check

clippy:
	cargo clippy --fix --allow-dirty --allow-staged

dist: dist/pch-$(HOST)

fmt: cargo-fmt fmt-toml\:Cargo.toml

#####################
# Auxiliary Targets #
#####################

cargo-fmt:
	cargo fmt

dist/pch-$(HOST): target/release/pre-commit-hooks
	@ install -D --no-target-directory --verbose "$<" "$@"

target/release/pre-commit-hooks: force
	cargo build --release

fmt-toml\:%: %
	toml-sort --in-place --all "$<"
	taplo format "$<"

force:
