/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Copyright 2017 - Dario Ostuni <dario.ostuni@gmail.com>
 *
 */

#![feature(step_by)]
#![feature(test)]

extern crate test;
#[allow(unused_imports)]
use test::Bencher;

#[allow(dead_code)]
fn sqrt(n: usize) -> usize
{
    let mut hi = n;
    let mut lo = 0usize;
    while lo + 1 < hi
    {
        let mid = (hi + lo) / 2;
        lo = mid;
        hi = n / lo;
        if lo > hi
        {
            std::mem::swap(&mut lo, &mut hi);
        }
    }
    hi
}

#[allow(dead_code)]
pub fn is_prime(n: usize) -> bool
{
    if let Some(preloaded) = match n
    {
           0 | 1 | 4 => Some(false),
           2 | 3 | 5 => Some(true),
           _ => None,
       }
    {
        return preloaded;
    }
    let mut primes: Vec<usize> = vec![2, 3];
    let mut primes_prod: usize = 6;
    let search_bound = sqrt(n);
    let window_bound = sqrt(search_bound);
    while primes_prod < window_bound
    {
        for i in (primes[primes.len() - 1] + 1)..
        {
            if primes.iter().all(|x| i % x != 0)
            {
                primes.push(i);
                primes_prod *= i;
                break;
            }
        }
    }
    if (2..primes_prod).any(|x| n % x == 0)
    {
        return false;
    }
    let mut mask = vec![true; primes_prod];
    for i in &primes
    {
        for j in (0..mask.len()).step_by(*i)
        {
            mask[j] = false;
        }
    }
    let num_mask: Vec<usize> = (0..mask.len()).filter(|&x| mask[x]).collect();
    !(1..)
         .take_while(|x| x * mask.len() <= search_bound)
         .any(|i| num_mask.iter().any(|x| n % (i * mask.len() + x) == 0))
}

#[allow(dead_code)]
fn is_prime_trivial(n: usize) -> bool
{
    n > 1 && (2..).take_while(|x| x * x <= n).all(|x| n % x != 0)
}

#[test]
fn check_is_prime()
{
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(41));
    assert!(!is_prime(81));
    assert!(!is_prime(12));
    assert!(!is_prime(25));
    assert!(is_prime(524287));
    assert!(is_prime(2147483647));
    assert!(is_prime(18014398509481951));
}

#[test]
fn check_is_prime_trivial()
{
    assert!(!is_prime_trivial(0));
    assert!(!is_prime_trivial(1));
    assert!(is_prime_trivial(2));
    assert!(is_prime_trivial(3));
    assert!(is_prime_trivial(41));
    assert!(!is_prime_trivial(81));
    assert!(!is_prime_trivial(12));
    assert!(!is_prime_trivial(25));
    assert!(is_prime_trivial(524287));
    assert!(is_prime_trivial(2147483647));
    assert!(is_prime_trivial(18014398509481951));
}

#[test]
fn check_consistency()
{
    for i in 0..100000
    {
        assert_eq!(is_prime(i), is_prime_trivial(i));
    }
}

#[test]
fn check_sqrt()
{
    assert_eq!(sqrt(42), 7);
    assert_eq!(sqrt(1000000007), 31623);
    assert_eq!(sqrt(17935225), 4235);
}

#[allow(non_snake_case)]
#[bench]
fn bench_is_prime_M19(b: &mut Bencher)
{
    b.iter(|| is_prime(524287));
}

#[allow(non_snake_case)]
#[bench]
fn bench_is_prime_trivial_M19(b: &mut Bencher)
{
    b.iter(|| is_prime_trivial(524287));
}

#[allow(non_snake_case)]
#[bench]
fn bench_is_prime_M31(b: &mut Bencher)
{
    b.iter(|| is_prime(2147483647));
}

#[allow(non_snake_case)]
#[bench]
fn bench_is_prime_trivial_M31(b: &mut Bencher)
{
    b.iter(|| is_prime_trivial(2147483647));
}

#[bench]
#[ignore]
fn bench_is_prime_18014398509481951(b: &mut Bencher)
{
    b.iter(|| is_prime(18014398509481951));
}

#[bench]
#[ignore]
fn bench_is_prime_trivial_18014398509481951(b: &mut Bencher)
{
    b.iter(|| is_prime_trivial(18014398509481951));
}
