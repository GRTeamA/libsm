// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate criterion;
extern crate pprof;

mod perf;

use criterion::Criterion;
use libsm::sm2::field::*;

fn pmul(c: &mut Criterion) {
    let ctx = FieldCtx::new();
    let a= FieldElem::new([0xffff_ffff;8]);
    let d= FieldElem::new([0xffff_ffff;8]);

    c.bench_function("field mul", move |b| {
        b.iter(|| raw_add(&a,&d));
    });
}

criterion_group!{
    name = primefield;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = pmul
}
criterion_main!(primefield);
