//! given `t` max distance, `k` number of towns, and `ls` list of distances
//! between towns, calculate the distance closest to but not exceeding `t` using
//! `k` of the distances in `ls`, or -1 if there are no valid combinations of
//! distances

use itertools::Itertools;

#[allow(dead_code, clippy::cast_sign_loss)]
pub fn choose_best_sum(t: i32, k: i32, ls: &[i32]) -> i32 {
    Itertools::combinations(ls.iter(), k as usize)
        .map(|c| c.into_iter().sum())
        .filter(|&s| s <= t)
        .max()
        .unwrap_or(-1)
}
