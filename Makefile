SHELL := /bin/bash
MAKEFLAGS := -s

REPO := https://github.com/rust-lang/rust
CHECKOUT := 31d74fb24bb16317e09f936fbf46590599b02940

TARGET := x86_64-unknown-daisogen

PATHS := $(shell find copy -type d)
PATHS := $(PATHS:copy/%=rust/%)
COPY := $(shell find copy -type f)
COPYDST := $(COPY:copy/%=rust/%)

.PHONY: all
all: | rust rust/.buildstamp
	@

.PHONY: enable
enable:
	rustup toolchain link dev-$(TARGET) `pwd`/build/x86_64-unknown-linux-gnu/stage2

rust:
	git clone $(REPO)
	cd rust && git checkout $(CHECKOUT)
	# Make sure files are copied
	touch $(COPY)

rust/.buildstamp: $(COPYDST)
	cd rust && ./x.py build library
	touch $@

$(COPYDST): rust/%: copy/% | $(PATHS)
	cp -av $< $@

$(PATHS): %:
	mkdir -p $@
