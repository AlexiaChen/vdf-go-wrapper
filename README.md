# vdf-go-wrapper
A Verifiable Delay Function(https://github.com/poanetwork/vdf) wrapper for go

The project is built for bounty of [harmony.one](https://github.com/harmony-one), which is my coding test of final round interview.

- Bounty issue: https://github.com/harmony-one/bounties/issues/4
- GitCoin: https://gitcoin.co/issue/harmony-one/bounties/4/100025313

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
# for test go and wrapper version
make gobench
```

Rust Wrapper Version:

```txt
BenchmarkVDFRustGenerateAndVerify-8            1        2540105400 ns/op            2424 B/op         11 allocs/op
BenchmarkVDFRustVerify-8                       2         918081350 ns/op             640 B/op          3 allocs/op
```

Go Version:

```txt
BenchmarkVDFGoGenerateAndVerify-8              1        21782273700 ns/op       22599506944 B/op        221367368 allocs/op
BenchmarkVDFGoVerify-8                         3          430348500 ns/op           95602450 B/op          946134 allocs/op
```

Run command as follows:

```bash
# for test pure rust vdf
make rustbench
```

Pure Rust Version:

```txt
Benchmarking vdf generate verify: Collecting 100 samples in estimated 254.63 s (100 iterations)
Benchmarking vdf generate verify: Analyzing
vdf generate verify     time:   [2.5783 s 2.5837 s 2.5892 s]
                        change: [-0.9282% -0.5185% -0.1339%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
mean   [2.5783 s 2.5892 s] std. dev.      [23.719 ms 32.048 ms]
median [2.5736 s 2.5867 s] med. abs. dev. [18.398 ms 33.491 ms]

Benchmarking vdf verify
Benchmarking vdf verify: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 95.4s, or reduce sample count to 10.
Benchmarking vdf verify: Collecting 100 samples in estimated 95.375 s (100 iterations)
Benchmarking vdf verify: Analyzing
vdf verify              time:   [960.19 ms 964.71 ms 969.97 ms]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
mean   [960.19 ms 969.97 ms] std. dev.      [15.679 ms 34.025 ms]
median [953.27 ms 964.37 ms] med. abs. dev. [10.966 ms 20.064 ms]
```

According to the above benchmark test results, there is no order of magnitude difference in CPU performance between VDF's rust and go version, which is basically the same level. However, the memory occupation and memory allocation times of rust are much lower than that of go version, that is to say, the rust version has only memory advantage over go version.

**Conclusion: so the Rust Version of VDF performance is not ideal compared with Go version from aspect of engineering. Maybe it is not a good repalcement for current VDF Go implementation**

## How to use 

Please see vdf_test.go, there are some examples and tests