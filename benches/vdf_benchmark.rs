//extern crate vdf_ffi;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vdf::{VDFParams, WesolowskiVDFParams, VDF};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vdf generate verify", |b| {
        let num_bits: u16 = 2048;
        let chellenge = vec![
            0xaa_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
            0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
            0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
            0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8, 0x00_u8,
        ];
        b.iter(|| {
            // An instance of the VDF.  Instances can be used arbitrarily many times.
            let wesolowski_vdf = WesolowskiVDFParams(num_bits).new();

            // Solve for the correct answer.  This will take a minute or two.

            let result = wesolowski_vdf.solve(&chellenge, 10000).unwrap();

            wesolowski_vdf.verify(&chellenge, 10000, &result)
        })
    });

    c.bench_function("vdf verify", |b| {
        let num_bits: u16 = 2048;
        let chellenge = vec![
            0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
            0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
            0xde, 0xad, 0xbe, 0xef,
        ];
        // An instance of the VDF.  Instances can be used arbitrarily many times.
        let wesolowski_vdf = WesolowskiVDFParams(num_bits).new();

        // Solve for the correct answer.  This will take a minute or two.

        let result = wesolowski_vdf.solve(&chellenge, 10000).unwrap();
        b.iter(|| wesolowski_vdf.verify(&chellenge, 10000, &result))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
