//! for the function `count_kprimes`, return numbers between `start` and `end`
//! whose count of prime factors is `k`; for the function `puzzle`, find the
//! number of solutions for the equation `a + b + c = s`, where `a` is 1-prime,
//! `b` is 3-prime, and `c` is 7-prime

fn prime_factor_count(n: i32) -> i32 {
    let (mut n, mut candidate, mut count) = (n, 2, 0);
    while n > 1 {
        while n % candidate == 0 {
            count += 1;
            n /= candidate;
        }
        candidate += 1;
    }
    count
}

#[allow(dead_code)]
pub fn count_kprimes(k: i32, start: i32, end: i32) -> Vec<i32> {
    (start..=end)
        .filter(|&n| prime_factor_count(n) == k)
        .collect()
}

#[allow(
    dead_code,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation
)]
pub fn puzzle(s: i32) -> i32 {
    let v_1 = count_kprimes(1, 2, s);
    let v_3 = count_kprimes(3, 8, s);
    let v_7 = count_kprimes(7, 128, s);

    v_7.into_iter()
        .map(|c| {
            v_3.iter()
                .filter(|&b| v_1.iter().any(|&a| a + b + c == s))
                .count()
        })
        .sum::<usize>() as i32
}
