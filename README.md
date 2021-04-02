# vdf-go-wrapper
A Verifiable Delay Function(https://github.com/poanetwork/vdf) wrapper for go


## Build & Test

### Build VDF Rust FFI dynamic library

if lib/libvdf_ffi.so does exist, you can skip this step and go next

you need to install the latest stable version of Rust, then run command as follows:

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

## BenchMarks

Run command as follows:

```bash
make bench
```

Rust Version:

```txt
BenchmarkVDFRustGenerateAndVerify-8            1        2540105400 ns/op            2424 B/op         11 allocs/op
BenchmarkVDFRustVerify-8                       2         918081350 ns/op             640 B/op          3 allocs/op
```

Go Version:

```txt
BenchmarkVDFGoGenerateAndVerify-8              1        21782273700 ns/op       22599506944 B/op        221367368 allocs/op
BenchmarkVDFGoVerify-8                         3          430348500 ns/op           95602450 B/op          946134 allocs/op
```

According to the above benchmark test results, there is no order of magnitude difference in CPU performance between VDF's rust and go version, which is basically the same level. However, the memory occupation and memory allocation times of rust are much lower than that of go version, that is to say, the rust version has only memory advantage over go version

## How to use 

Please see vdf_test.go, there are some examples and tests