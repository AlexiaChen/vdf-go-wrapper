package vdf_go

import (
	"testing"
)

func TestVDF(t *testing.T) {

	bytes := [32]byte{0xaa}

	vdf := New(10000, bytes)
	vdf.Execute()
	isSuccess := vdf.Verify(vdf.output)
	if isSuccess == false {
		t.Errorf("VDF Verify failed")
	}
}
