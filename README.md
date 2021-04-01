# vdf-go-wrapper
A Verifiable Delay Function(https://github.com/poanetwork/vdf) wrapper for go


## Build & Test

### Build VDF Rust FFI dynamic library

first, you need to install the latest stable version of Rust, then run command as follows:

```bash
make ffi
```

### Build VDF FFI wrapper for golang

```
make build
```

### Run VDF FFI wrapper test

```
make test
```

## How to use 

Please see vdf_test.go, there are some examples and tests