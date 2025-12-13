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
