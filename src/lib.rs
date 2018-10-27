// Copyright 2018 Jeremy Rubin

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

fn prior_power_of_two(x: usize) -> usize {
    ((x as u128).next_power_of_two() >> 1) as usize
}

// Algorithm From Knuth's Sorting and Searching 5.2.2M
pub fn const_sort<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    let t = prior_power_of_two(v.len());
    let mut p = t;
    while p > 0 {
        let mut q = t;
        let mut r = 0;
        let mut d = p;
        while d > 0 {
            for i in 0..(v.len()-d) {
                if (i & p) == r {
                    let (f,l) = v.split_at_mut(1+i);
                    compare_and_swap(&mut l[d-1], &mut f[i], cmp);
                }
            }
            d = q -p;
            q >>=1;
            r = p;
        }
        p >>=1;
    }
}
#[inline(always)]
fn compare_and_swap<T, F>(v: &mut T, s: &mut T, cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    let choice: subtle::Choice = cmp(&v, &s);
    v.conditional_swap(s, choice);
}


#[cfg(test)]
mod tests {
    use rand::prelude::*;
    #[test]
    fn test_correct() {
        for x in 0..=255usize {
            let mut a = vec![0; x];
            for val in a.iter_mut() {
                *val = thread_rng().gen_range(0, 100);
            }
            crate::const_sort(&mut a, &|l, r| ((l < r) as u8).into());
            let mut ans = a.clone();
            ans.sort();
            assert_eq!(ans, a);
        }
    }
    #[test]
    fn test_bitonic_u8_exhaustive() {
        for x in 0..=7u8 {
            let mut a = [1u8 & (x >> 0), 1u8 & (x >> 1), 1u8 & (x >> 2)];
            crate::const_sort(&mut a, &|l, r| ((l < r) as u8).into());
            let mut ans = a.clone();
            ans.sort();
            assert_eq!(ans, a);
        }
    }

    // If this succeeds, we can probably sort correctly -- reason being,
    // we are maximally testing that the network can handle weird sizes.
    //
    #[test]
    fn test_bitonic_random_large_prime() {
        for _ in 0..=255u8 {
            let mut a = [0u8; 7919];
            for i in 0..7919 {
                a[i] = thread_rng().gen_range(0, 1);
            }
            crate::const_sort(&mut a, &|l, r| ((l < r) as u8).into());
            let mut ans = a.clone();
            ans.sort();
            assert_eq!(ans[..], a[..]);
        }
    }
}
