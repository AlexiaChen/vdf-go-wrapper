package vdf_rs_wrapper

/*
#cgo LDFLAGS: -L./lib -lvdf_ffi
#include "./lib/vdf.h"
*/
import "C"
import (
	"unsafe"
)

// VDF is the struct holding necessary state for a hash chain delay function.
type VDF struct {
	difficulty int
	input      [32]byte
	output     [516]byte
	outputChan chan [516]byte
	finished   bool
}

//size of long integers in quadratic function group
const sizeInBits = 2048

// New create a new instance of VDF.
func New(difficulty int, input [32]byte) *VDF {
	return &VDF{
		difficulty: difficulty,
		input:      input,
		outputChan: make(chan [516]byte),
	}
}

// GetOutputChannel returns the vdf output channel.
// VDF output consists of 258 bytes of serialized Y and  258 bytes of serialized Proof
func (vdf *VDF) GetOutputChannel() chan [516]byte {
	return vdf.outputChan
}

// Execute runs the VDF until it's finished and put the result into output channel.
func (vdf *VDF) Execute() {
	vdf.finished = false

	var vdf_wesolowski *C.struct_WesolowskiVDF = C.create_wesolowski_vdf(sizeInBits)

	chellenge_data_slice := vdf.input[:]
	chellenge := C.struct_Buffer{
		data: (*C.uint8_t)(C.CBytes(chellenge_data_slice)),
		len:  32,
	}

	output := &C.struct_Buffer{
		data: nil,
		len:  0,
	}

	var result C.int = C.vdf_compute(vdf_wesolowski, chellenge, C.ulong(vdf.difficulty), output)
	if result == 0 {
		bytes := C.GoBytes(unsafe.Pointer(output.data), (C.int)(output.len))
		copy(vdf.output[:], bytes)
	}

	go func() {
		vdf.outputChan <- vdf.output
	}()

	C.free_buffer(*output)
	C.free_wesolowski_vdf(vdf_wesolowski)

	vdf.finished = true
}

// Verify runs the verification of generated proof
func (vdf *VDF) Verify(proof [516]byte) bool {
	var vdf_wesolowski *C.struct_WesolowskiVDF = C.create_wesolowski_vdf(sizeInBits)

	chellenge_data_slice := vdf.input[:]
	chellenge := C.struct_Buffer{
		data: (*C.uint8_t)(C.CBytes(chellenge_data_slice)),
		len:  32,
	}

	output_proof := &C.struct_Buffer{
		data: (*C.uint8_t)(C.CBytes(proof[:])),
		len:  516,
	}

	var result_verify C.int = C.vdf_verify(vdf_wesolowski, chellenge, C.ulong(vdf.difficulty), *output_proof)

	C.free_wesolowski_vdf(vdf_wesolowski)

	return result_verify == 0
}

// IsFinished returns whether the vdf execution is finished or not.
func (vdf *VDF) IsFinished() bool {
	return vdf.finished
}

// GetOutput returns the vdf output, which can be bytes of 0s is the vdf is not finished.
func (vdf *VDF) GetOutput() [516]byte {
	return vdf.output
}
