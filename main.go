package main

/*
#cgo LDFLAGS: -L./lib -lvdf_ffi
#include "./lib/vdf.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

func main() {
	var vdf *C.struct_WesolowskiVDF = C.create_wesolowski_vdf(2048)
	data_ptr := []byte{0xaa}
	chellenge := C.struct_Buffer{
		data: (*C.uint8_t)(C.CBytes(data_ptr)),
		len:  1,
	}

	//data_slice := make([]byte, 1024)
	output := &C.struct_Buffer{
		//data: (*C.uchar)(C.CBytes(data_slice)),
		data: nil,
		len:  0,
	}

	var result C.int = C.vdf_compute(vdf, chellenge, 10000, output)
	if result == 0 {
		fmt.Println("compute success.")
		fmt.Printf("len: %v\n", output.len)

		bytes := C.GoBytes(unsafe.Pointer(output.data), (C.int)(output.len))
		for i := range bytes {
			fmt.Printf("%v ", i)
		}
	} else {
		fmt.Println("compute failed.")
	}

}
