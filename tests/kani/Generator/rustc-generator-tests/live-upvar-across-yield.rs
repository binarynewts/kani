// Copyright rustc Contributors
// Adapted from rustc: https://github.com/rust-lang/rust/tree/5f98537eb7b5f42c246a52c550813c3cff336069/src/test/ui/generator/live-upvar-across-yield.rs
//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Modifications Copyright Kani Contributors
// See GitHub history for details.

// run-pass

#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

#[kani::proof]
fn main() {
    let b = |_| 3;
    let mut a = || {
        b(yield);
    };
    Pin::new(&mut a).resume(());
}
