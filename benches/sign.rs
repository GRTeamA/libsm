// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate criterion;
extern crate pprof;

mod perf;

use criterion::Criterion;
use libsm::sm2::signature::SigCtx;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, One};


fn signature(c: &mut Criterion) {
    let string = String::from("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
    let msg = string.as_bytes();

    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();

    c.bench_function("sign", move |b| {
        b.iter(|| ctx.sign(msg, &sk, &pk));
    });
}

criterion_group!{
    name = sign;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = signature
}
criterion_main!(sign);
