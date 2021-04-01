ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

build:
	cargo build --release
	cp target/release/libvdf_ffi.so lib/
	go build -ldflags="-r $(ROOT_DIR)lib"

test: 
	go test -v -cover -ldflags="-r $(ROOT_DIR)lib" ./...