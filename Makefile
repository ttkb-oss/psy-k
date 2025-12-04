PRIVATE := target/.private

RUSTFLAGS := -C instrument-coverage
TEST_FLAGS := LLVM_PROFILE_FILE=$(PRIVATE)/profile/cargo-test-%p-%m.profraw
ENV_FLAGS := $(TEST_FLAGS)

ifeq ($(shell which -s llvm-profdata-20 && echo 1 || echo 0), 1)
    LLVM_PROFDATA := llvm-profdata-20
else
    LLVM_PROFDATA := llvm-profdata
endif

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
report: $(PRIVATE)/profile/json5format.profdata
	rm -rf target/coverage
	grcov $(PRIVATE)/profile --binary-path ./target/debug/deps/ -s src \
        -t html \
		--branch \
        --ignore-not-existing \
		--ignore '../*' \
		--ignore "/*" \
	    -o target/coverage/html

$(PRIVATE)/profile/json5format.profdata: test
	mkdir -p $(PRIVATE)/profile
	$(LLVM_PROFDATA) merge -sparse $(PRIVATE)/profile/*.profraw -o $@

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
	$(PRIVATE)/tests/data/psy-q-genesis/LIBSN68

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
