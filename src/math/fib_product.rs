//! given a positive integer `prod`, return two consecutive fibonacci numbers
//! whose product is equal to `prod`; if the product equals `prod` then return
//! the fibonacci numbers and true, if the product is greater than `prod`,
//! return the fibonacci numbers and false

#[allow(dead_code)]
pub const fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut n = 0;
    let mut n_1 = 1;

    while n * n_1 < prod {
        let c = n + n_1;
        n = n_1;
        n_1 = c;
    }

    (n, n_1, n * n_1 == prod)
}
