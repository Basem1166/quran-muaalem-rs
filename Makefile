# Developer workflow for the workspace and the BoltFFI bindings.
# CI (.github/workflows) calls these same targets. See README.md, "BoltFFI bindings".
#
# Toolchain logic lives in crates/engine/tests/bindings/toolchains.sh and the
# smoke tests in crates/engine/tests/bindings/run.sh; this file only wires them up.

SHELL := bash
TOOLCHAINS := bash crates/engine/tests/bindings/toolchains.sh
RUN_BINDINGS := $(TOOLCHAINS) run bash crates/engine/tests/bindings/run.sh

LANGS := python java kotlin dart swift
# the Swift smoke test packs an Apple xcframework, so it only runs on macOS
HOST_LANGS := $(if $(filter Darwin,$(shell uname -s)),$(LANGS),$(filter-out swift,$(LANGS)))

.PHONY: help test lint fmt install-boltffi doctor bind-all test-bind-all \
	$(addprefix bind-,$(LANGS)) $(addprefix test-bind-,$(LANGS)) $(addprefix doctor-,$(LANGS))

help:
	@echo "Rust:"
	@echo "  make test                 cargo test --workspace"
	@echo "  make lint                 rustfmt check + clippy -D warnings (same as CI)"
	@echo "  make fmt                  format the workspace"
	@echo "Bindings:"
	@echo "  make install-boltffi      install the boltffi CLI version pinned in Cargo.toml"
	@echo "  make doctor               check every toolchain, print fix commands"
	@echo "  make bind-<lang>          install a toolchain into .toolchains/ (no sudo)"
	@echo "  make bind-all             ... for every language this host can test"
	@echo "  make test-bind-<lang>     doctor + pack + smoke test one binding"
	@echo "  make test-bind-all        ... for every language this host can test ($(HOST_LANGS))"
	@echo "  <lang> = $(LANGS)"

test:
	cargo test --workspace

lint:
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

fmt:
	cargo fmt --all

install-boltffi:
	@$(TOOLCHAINS) install-boltffi

doctor:
	@$(TOOLCHAINS) doctor

# static pattern rules: plain `doctor-%:` rules are skipped for .PHONY targets
$(addprefix doctor-,$(LANGS)): doctor-%:
	@$(TOOLCHAINS) doctor cli $*

$(addprefix bind-,$(LANGS)): bind-%:
	@$(TOOLCHAINS) install $*

bind-all:
	@for lang in $(HOST_LANGS); do $(TOOLCHAINS) install $$lang; done

$(addprefix test-bind-,$(LANGS)): test-bind-%: doctor-%
	@$(RUN_BINDINGS) $*

test-bind-all:
	@$(TOOLCHAINS) doctor cli $(HOST_LANGS)
	@$(RUN_BINDINGS) $(HOST_LANGS)
