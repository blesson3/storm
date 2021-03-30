# https://github.com/rust-cli/meta/issues/33#issuecomment-529796973
PKG_NAME = $(shell awk -F ' = ' '$$1 ~ /name/    { gsub(/[\"]/, "", $$2); printf("%s",$$2) }' Cargo.toml)

SRC = $(shell find src -type f -name "*.rs") Cargo.lock Cargo.toml
BIN_DEBUG = target/debug/$(PKG_NAME)
BIN_RELEASE = target/release/$(PKG_NAME)

.PHONY: build
build: $(BIN_DEBUG)

$(BIN_DEBUG): $(SRC)
	cargo build

.PHONY: release
release: $(BIN_RELEASE)

$(BIN_RELEASE): $(SRC)
	cargo build --release --locked

.PHONY: install
install: $(SRC)
	cargo install --path .
