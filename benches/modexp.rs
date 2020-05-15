extern crate criterion;
extern crate num_bigint;
extern crate rust_modexp_testing;

use criterion::{criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use core::mem::MaybeUninit;
use gmp_mpfr_sys::gmp;
use num_bigint::BigUint;
use num_traits::Num;
use std::ffi::CString;
use std::vec::Vec;

fn gmp_benchmark(c: &mut Criterion) {
    unsafe {
        let mut b_vec = Vec::new();
        let mut e_vec = Vec::new();
        let mut m_vec = Vec::new();

        for test in rust_modexp_testing::MODEXPTESTS.iter() {
            let mut base_bn = MaybeUninit::uninit();
            gmp::mpz_init(base_bn.as_mut_ptr());
            let mut base_bn = base_bn.assume_init();
            let b = CString::new(test.base).expect("b failed");
            gmp::mpz_set_str(&mut base_bn, b.as_ptr(), 16);

            let mut exp_bn = MaybeUninit::uninit();
            gmp::mpz_init(exp_bn.as_mut_ptr());
            let mut exp_bn = exp_bn.assume_init();
            let e = CString::new(test.exponent).expect("e failed");
            gmp::mpz_set_str(&mut exp_bn, e.as_ptr(), 16);

            let mut mod_bn = MaybeUninit::uninit();
            gmp::mpz_init(mod_bn.as_mut_ptr());
            let mut mod_bn = mod_bn.assume_init();
            let m = CString::new(test.modulus).expect("m failed");
            gmp::mpz_set_str(&mut mod_bn, m.as_ptr(), 16);

            b_vec.push(base_bn);
            e_vec.push(exp_bn);
            m_vec.push(mod_bn);
        }

        let mut result_bn = MaybeUninit::uninit();
        gmp::mpz_init(result_bn.as_mut_ptr());
        let mut result_bn = result_bn.assume_init();
        c.bench(
            "gmp",
            ParameterizedBenchmark::new(
                "modexp",
                move |b, idx| {
                    b.iter(|| {
                        gmp::mpz_powm(&mut result_bn, &b_vec[*idx], &e_vec[*idx], &m_vec[*idx]);
                    })
                },
                0..rust_modexp_testing::MODEXPTESTS.len()
            ),
        );
    }
}

fn biguint_benchmark(c: &mut Criterion) {
    let mut b_vec = Vec::new();
    let mut e_vec = Vec::new();
    let mut m_vec = Vec::new();

    for test in rust_modexp_testing::MODEXPTESTS.iter() {
        b_vec.push(BigUint::from_str_radix(test.base, 16).unwrap());
        e_vec.push(BigUint::from_str_radix(test.exponent, 16).unwrap());
        m_vec.push(BigUint::from_str_radix(test.modulus, 16).unwrap());
    }

    c.bench(
        "biguint",
        ParameterizedBenchmark::new(
            "modexp",
            move |b, idx| {
                b.iter(|| {
                    b_vec[*idx].modpow(&(e_vec[*idx]), &(m_vec[*idx]));
                })
            },
            0..rust_modexp_testing::MODEXPTESTS.len()
        ),
    );
}

criterion_group!(benches, gmp_benchmark, biguint_benchmark);
criterion_main!(benches);
