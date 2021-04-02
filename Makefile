ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

ffi:
	cargo build --release
	cp target/release/libvdf_ffi.so lib/

build:
	go build -ldflags="-r $(ROOT_DIR)lib"

test: 
	go test -v -cover -ldflags="-r $(ROOT_DIR)lib" ./...

bench: 
	go test -bench=. -benchmem  -run=none -ldflags="-r $(ROOT_DIR)lib"