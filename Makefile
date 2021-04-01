ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cargo build --release
	cp target/release/libvdf_go_wrapper.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib" main.go

run: build
	./main