// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate criterion;
extern crate pprof;

mod perf;

use criterion::Criterion;
use libsm::sm2::field::{FieldCtx, FieldElem};

fn pmul(c: &mut Criterion) {
    let ctx = FieldCtx::new();
    let modulus = FieldElem::new([
        0xffff_fffe,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0xffff_ffff,
        0x0000_0000,
        0xffff_ffff,
        0xffff_ffff,
    ]);

    c.bench_function("field mul", move |b| {
        b.iter(|| ctx.mul(&modulus,&modulus));
    });
}

criterion_group!{
    name = primefield;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = pmul
}
criterion_main!(primefield);
