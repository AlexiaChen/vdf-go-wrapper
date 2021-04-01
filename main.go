package main

/*
#cgo LDFLAGS: -L./lib -lvdf_go_wrapper
#include "./lib/vdf.h"
*/
import "C"

func main() {
	C.create_wesolowski_vdf(2048)
}
