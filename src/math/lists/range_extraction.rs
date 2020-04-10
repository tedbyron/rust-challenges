//! given a list of integers in increasing order, swap continuous runs of 3 or
//! more numbers with ranges and return the result as a string

#[allow(dead_code)]
pub fn range_extraction(a: &[i32]) -> String {
    a.iter()
        .enumerate()
        .fold((String::new(), 0, 0), |(mut r, curr, prev), (i, n)| {
            let next = *a.get(i + 1).unwrap_or(n) - n;
            match (i, n.to_string(), prev, curr, next) {
                (0, n, ..) => r += &n,
                (.., 1, 1) => (),
                (_, n, 1, 1, _) => r += &format!("-{}", n),
                (_, n, ..) => r += &format!(",{}", n),
            };
            (r, next, curr)
        })
        .0
}
