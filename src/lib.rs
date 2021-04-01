extern crate libc;
extern crate vdf;

use libc::*;
use vdf::{VDFParams, WesolowskiVDFParams, VDF};

/// WesolowskiVDF instance
#[repr(C)]
pub struct WesolowskiVDF {
    init_num_bits: u16,
}

/// Buffer to store bytes array
#[repr(C)]
pub struct Buffer {
    data: *mut u8,
    len: usize,
}

/// create new Wesolowski VDF instance
#[no_mangle]
pub extern "C" fn create_wesolowski_vdf(num_bits: u16) -> *mut WesolowskiVDF {
    assert!(num_bits == 2048_u16);

    let vdf = WesolowskiVDF {
        init_num_bits: num_bits,
    };
    Box::into_raw(Box::new(vdf))
}

/// free Wesolowski VDF instance
#[no_mangle]
pub extern "C" fn free_wesolowski_vdf(vdf: *mut WesolowskiVDF) {
    unsafe {
        Box::from_raw(vdf);
    }
}

/// free buffer generated from the library
#[no_mangle]
pub extern "C" fn free_buffer(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len) };
    unsafe {
        Box::from_raw(s as *mut [u8]);
    }
}

/// vdf_compute will output solution buffer that you need free when finished
/// chellenge buffer do not need free, you create chellenge bytes buffer
#[no_mangle]
pub extern "C" fn vdf_compute(
    vdf: *mut WesolowskiVDF,
    chellenge: Buffer,
    difficulty: u64,
    output_solution: *mut Buffer,
) -> c_int {
    unsafe {
        let wesolowski_vdf = WesolowskiVDFParams((*vdf).init_num_bits).new();
        let s = std::slice::from_raw_parts(chellenge.data, chellenge.len);
        let result = wesolowski_vdf.solve(s, difficulty);
        if result.is_ok() {
            let vec_result = result.unwrap();
            let mut buf = vec_result.into_boxed_slice();
            let data = buf.as_mut_ptr();
            let len = buf.len();
            std::mem::forget(buf);
            (*output_solution).data = data;
            (*output_solution).len = len;
            return 0;
        } else {
            return 1;
        }
    }
}

/// vdf_verify verify solution from chellenge and difficulty
#[no_mangle]
pub extern "C" fn vdf_verify(
    vdf: *mut WesolowskiVDF,
    chellenge: Buffer,
    difficulty: u64,
    solution: Buffer,
) -> c_int {
    unsafe {
        let wesolowski_vdf = WesolowskiVDFParams((*vdf).init_num_bits).new();
        let c = std::slice::from_raw_parts(chellenge.data, chellenge.len);
        let solu = std::slice::from_raw_parts(solution.data, solution.len);
        let result = wesolowski_vdf.verify(c, difficulty, solu);
        if result.is_ok() {
            return 0;
        } else {
            return 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use vdf::{VDFParams, WesolowskiVDFParams, VDF};

        let num_bits: u16 = 2048;

        // An instance of the VDF.  Instances can be used arbitrarily many times.
        let wesolowski_vdf = WesolowskiVDFParams(num_bits).new();

        // Solve for the correct answer.  This will take a minute or two.

        let result = &wesolowski_vdf.solve(b"\xaa", 10000).unwrap();

        // Verify the answer.  This should be far faster (less than a second).
        assert!(wesolowski_vdf.verify(b"\xaa", 10000, result).is_ok());
    }
}
