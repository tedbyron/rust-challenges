//! given a positive integer `n`, return all the divisors from `2..n` or else
//! return a string saying the integer is prime

#[allow(dead_code)]
pub fn divisors(n: u32) -> Result<Vec<u32>, String> {
    let divs = (2..n).filter(|i| n % i == 0).collect::<Vec<u32>>();

    if divs.is_empty() {
        Ok(divs)
    } else {
        Err(format!("{} is prime", n))
    }
}
