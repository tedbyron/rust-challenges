// calculate the product of consecutive fibonacci numbers

#[allow(dead_code)]
pub fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut n = 0;
    let mut n_1 = 1;

    while n * n_1 < prod {
        let c = n + n_1;
        n = n_1;
        n_1 = c;
    }

    (n, n_1, n * n_1 == prod)
}
