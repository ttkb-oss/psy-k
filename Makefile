PRIVATE := target/.private

RUSTFLAGS := -C instrument-coverage
TEST_FLAGS := LLVM_PROFILE_FILE=$(PRIVATE)/profile/cargo-test-%p-%m.profraw
ENV_FLAGS := $(TEST_FLAGS)

# Detect LLVM version
LLVM_VERSION := $(shell \
	if command -v llvm-profdata-20 >/dev/null 2>&1; then echo "20"; \
	elif command -v llvm-profdata-19 >/dev/null 2>&1; then echo "19"; \
	elif command -v llvm-profdata-18 >/dev/null 2>&1; then echo "18"; \
	elif command -v llvm-profdata >/dev/null 2>&1; then echo ""; \
	else echo "none"; fi)

ifeq ($(LLVM_VERSION),none)
    $(error LLVM tools not found. Install with: rustup component add llvm-tools-preview)
endif

ifneq ($(LLVM_VERSION),)
    LLVM_PROFDATA := llvm-profdata-$(LLVM_VERSION)
    LLVM_COV := llvm-cov-$(LLVM_VERSION)
else
    LLVM_PROFDATA := llvm-profdata
    LLVM_COV := llvm-cov
endif

# Coverage flags
RUSTFLAGS_COV := -C instrument-coverage \
                 -C link-dead-code \
                 -C opt-level=0 \
                 -C debuginfo=2

# Profile output location
PROFRAW_DIR := $(PRIVATE)/coverage/profraw
PROFDATA_FILE := $(PRIVATE)/coverage/merged.profdata

# Source root (for accurate path mapping)
SOURCE_ROOT := $(shell pwd)

.PHONY: all
all:
	$(ENV_FLAGS) RUSTFLAGS="$(RUSTFLAGS)" cargo build

.PHONY: check
check: test spellcheck doc clippy fmt

.PHONY: test
test: test-data
	rm -rf $(PRIVATE)/profile
	$(ENV_FLAGS) RUST_BACKTRACE=1 RUSTFLAGS="$(RUSTFLAGS)" cargo test --verbose

.PHONY: test-docs
test-docs:
	cargo test --doc

.PHONY: doc
doc:
	cargo doc

.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: clean
clean:
	cargo clean
	rm -rf $(PRIVATE)

.PHONY: report
report: coverage-html

# coverage
# Install llvm-cov if not present
.PHONY: coverage-install
coverage-install:
	@echo "Installing coverage tools..."
	rustup component add llvm-tools-preview
	cargo install cargo-llvm-cov || true

# Run tests with coverage
.PHONY: coverage
coverage: coverage-clean test-data
	@echo "Running tests with coverage instrumentation..."
	@mkdir -p $(PRIVATE)/coverage
	cargo llvm-cov --all-features --workspace --lcov --output-path $(PRIVATE)/coverage/lcov.info

# Generate HTML report (human-readable)
.PHONY: coverage-html
coverage-html: coverage
	@echo "Generating HTML coverage report..."
	cargo llvm-cov --all-features --workspace --html
	@echo "Coverage report generated at: target/llvm-cov/html/index.html"
	@echo "Open with: open target/llvm-cov/html/index.html (macOS) or xdg-open target/llvm-cov/html/index.html (Linux)"

# Generate JSON report (machine-readable)
.PHONY: coverage-json
coverage-json: coverage
	@echo "Generating JSON coverage report..."
	@mkdir -p $(PRIVATE)/coverage
	cargo llvm-cov --all-features --workspace --json --output-path $(PRIVATE)/coverage/coverage.json

# Generate text report (terminal output)
.PHONY: coverage-text
coverage-text: coverage
	@echo "Generating text coverage report..."
	cargo llvm-cov --all-features --workspace

# Generate all formats
.PHONY: coverage-all
coverage-all: coverage-html coverage-json coverage-text
	@echo "Coverage reports generated:"
	@echo "  - LCOV:  $(PRIVATE)/coverage/lcov.info"
	@echo "  - JSON:  $(PRIVATE)/coverage/coverage.json"
	@echo "  - HTML:  target/llvm-cov/html/index.html"

# Upload to codecov.io
.PHONY: coverage-upload
coverage-upload: coverage
	@echo "Uploading coverage to codecov.io..."
	@if [ -z "$$CODECOV_TOKEN" ]; then \
		echo "Error: CODECOV_TOKEN environment variable not set"; \
		echo "Get token from: https://codecov.io/gh/ttkb-oss/psy-k/settings"; \
		exit 1; \
	fi
	curl -Os https://uploader.codecov.io/latest/linux/codecov
	chmod +x codecov
	./codecov -t $$CODECOV_TOKEN -f $(PRIVATE)/coverage/lcov.info
	rm -f codecov

# Clean coverage artifacts
.PHONY: coverage-clean
coverage-clean:
	@echo "Cleaning coverage artifacts..."
	rm -rf $(PRIVATE)/coverage
	rm -rf target/llvm-cov
	cargo llvm-cov clean --workspace || true

# spell check
.PHONY: spellcheck
spellcheck: target/.private/dict.dic
	cargo spellcheck
target/.private/dict.txt: .config/dict.txt
	mkdir -p $(dir $@)
	cat $< | sort | uniq > $@
target/.private/dict.dic: target/.private/dict.txt
	wc -l $< | awk '{print $$1}' > $@.tmp
	cat $< >> $@.tmp
	mv $@.tmp $@

.PHONY: test-data
test-data: \
	$(PRIVATE)/tests/data/psy-q-saturn/dos/GNUSHLIB \
	$(PRIVATE)/tests/data/psy-q-genesis/LIBSN68 \
	$(PRIVATE)/tests/data/Psy-Q_46 \
	$(PRIVATE)/tests/data/Psy-Q_47

additional-sdks: \
	$(PRIVATE)/tests/data/SN_Systems_N64_SDK_ProDG_v1.0.0.2 \
	$(PRIVATE)/tests/data/psyq_snes_sdk

$(PRIVATE)/tests/data/psy-q-saturn/dos/GNUSHLIB: $(PRIVATE)/tests/data/psy-q-saturn
	unzip $</dos/GNUSHLIB.ZIP -d $</dos/GNUSHLIB
	chmod -R +w $@

$(PRIVATE)/tests/data/psy-q-saturn: $(PRIVATE)/tests/data/psy-q-saturn.zip
	unzip $< -d $@

$(PRIVATE)/tests/data/psy-q-saturn.zip:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ https://archive.org/download/psy-q/psy-q.zip
	shasum -c tests/data/psy-q-saturn.zip.sha1 || ( mv $@ $@.bad && false )

$(PRIVATE)/tests/data/psy-q-genesis/LIBSN68: $(PRIVATE)/tests/data/psy-q-genesis
	unzip $</LIBSN68.ZIP -d $</LIBSN68

$(PRIVATE)/tests/data/psy-q-genesis: $(PRIVATE)/tests/data/psy-q-genesis.zip
	unzip $< -d $@

$(PRIVATE)/tests/data/psy-q-genesis.zip:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ "https://drive.google.com/uc?id=1btNRRCcqY-tgVLMcXlVEkgTx0uCi1vET"
	shasum -c tests/data/psy-q-genesis.zip.sha1 || ( mv $@ $@.bad && false )

$(PRIVATE)/tests/data/Psy-Q_46: $(PRIVATE)/tests/data/Psy-Q_46.zip
	unzip $< -d $@

$(PRIVATE)/tests/data/Psy-Q_46.zip:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ 'https://download.gamingdoc.org/Software/Consoles/Sony%20Playstation/SDK/Psy-Q/4.6/Psy-Q_46.zip'
	shasum -c tests/data/Psy-Q_46.zip.sha1 || ( mv $@ $@.bad && false )

$(PRIVATE)/tests/data/Psy-Q_47: $(PRIVATE)/tests/data/Psy-Q_47.zip
	unzip $< -d $@

$(PRIVATE)/tests/data/Psy-Q_47.zip:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ 'https://download.gamingdoc.org/Software/Consoles/Sony%20Playstation/SDK/Psy-Q/4.7/Psy-Q_47.zip'
	shasum -c tests/data/Psy-Q_47.zip.sha1 || ( mv $@ $@.bad && false )

$(PRIVATE)/tests/data/SN_Systems_N64_SDK_ProDG_v1.0.0.2: $(PRIVATE)/tests/data/SN_Systems_N64_SDK_ProDG_v1.0.0.2.zip
	unzip $< -d $@

$(PRIVATE)/tests/data/SN_Systems_N64_SDK_ProDG_v1.0.0.2.zip:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ 'http://retrogameplayer.com/downloads/sdk/n64/SN_Systems_N64_SDK_ProDG_v1.0.0.2.zip'
	shasum -c tests/data/SN_Systems_N64_SDK_ProDG_v1.0.0.2.zip.sha1 || ( mv $@ $@.bad && false )

$(PRIVATE)/tests/data/psyq_snes_sdk: $(PRIVATE)/tests/data/psyq_snes_sdk.7z
	7z x -o$@ $<

$(PRIVATE)/tests/data/psyq_snes_sdk.7z:
	mkdir -p $(shell dirname $@)
	wget -q -O $@ 'https://web.archive.org/web/20170731214848if_/http://www.romhacking.net/download/utilities/1022/'
	shasum -c tests/data/psyq_snes_sdk.7z.sha1 || ( mv $@ $@.bad && false )
