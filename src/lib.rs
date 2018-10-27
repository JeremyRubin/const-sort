// Copyright 2018 Jeremy Rubin

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

fn prior_power_of_two(x: usize) -> usize {
    ((x as u128).next_power_of_two() >> 1) as usize
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

#[inline(always)]
pub fn const_sort<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    const_sort_asc(v, cmp);
}

pub fn const_sort_desc<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    match v.len() {
        0 | 1 => {
            return;
        }
        2 => {
            let (first, last) = v.split_at_mut(1);
            compare_and_swap(&mut first[0], &mut last[0], cmp)
        }
        n => {
            let m = n.wrapping_shr(1);
            const_sort_asc(&mut v[0..m], cmp);
            const_sort_desc(&mut v[m..n], cmp);
            const_merge_desc(v, cmp);
        }
    }
}

pub fn const_sort_asc<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    match v.len() {
        0 | 1 => {
            return;
        }
        2 => {
            let (first, last) = v.split_at_mut(1);
            compare_and_swap(&mut last[0], &mut first[0], cmp)
        }
        n => {
            let m = n.wrapping_shr(1);
            const_sort_desc(&mut v[0..m], cmp);
            const_sort_asc(&mut v[m..n], cmp);
            const_merge_asc(v, cmp);
        }
    }
}

fn const_merge_asc<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    let n = v.len();
    if n <= 1 {
        return;
    }
    let m = prior_power_of_two(n);

    // We want to compare 0 to n-m and m to n OR 0 to m and m to n. Either way
    // we are comparing (n-m) to m of th elements. It's easier to do 0 to m and
    // m to n expressed in rust.
    let (left, right) = v.split_at_mut(m);
    // N.B., technically the right side is guaranteed to be of size (n -m)
    // and the left size m.
    //
    // 2m <= n as a consequence of m being the prior power of two
    // 2m -m <= n-m
    // m <= n-m
    //
    // Therefore, every element on the right side is iterated over but we
    // may not compare elements in the middle.
    //
    left.iter_mut()
        .zip(right.iter_mut())
        .for_each(|(l, r)| compare_and_swap(r, l, cmp));
    const_merge_asc(&mut v[0..m], cmp);
    const_merge_asc(&mut v[m..n], cmp);
}
fn const_merge_desc<T, F>(v: &mut [T], cmp: &F)
where
    F: Fn(&T, &T) -> subtle::Choice,
    T: subtle::ConditionallySwappable,
{
    let n = v.len();
    if n <= 1 {
        return;
    }
    let m = prior_power_of_two(n);
    let (left, right) = v.split_at_mut(m);
    left.iter_mut()
        .zip(right.iter_mut())
        .for_each(|(l, r)| compare_and_swap(l, r, cmp));
    const_merge_desc(&mut v[0..m], cmp);
    const_merge_desc(&mut v[m..n], cmp);
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
