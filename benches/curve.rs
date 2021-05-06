// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate criterion;
extern crate pprof;

mod perf;

use criterion::Criterion;
use libsm::sm2::ecc::EccCtx;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, One};

fn ecc_add(c: &mut Criterion) {
    let curve = EccCtx::new();
    let g = curve.generator();

    let n = curve.get_n() - BigUint::one();

    c.bench_function("Ecc old add ", move |b| {
        b.iter(|| curve.g_mul(&n));
    });
}

criterion_group!{
    name = curve;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = ecc_add
}
criterion_main!(curve);
