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


fn verify_key(c: &mut Criterion) {
    let string = String::from("abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd");
    let msg = string.as_bytes();

    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    let signature = ctx.sign(msg, &sk, &pk);

    c.bench_function("Ecc old add ", move |b| {
        b.iter(|| ctx.verify(msg, &pk, &signature));
    });
}

criterion_group!{
    name = verify;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = verify_key
}
criterion_main!(verify);
