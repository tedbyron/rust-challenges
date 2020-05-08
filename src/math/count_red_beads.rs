//! given a number of blue beads, count the red beads if the arrangement of
//! beads is as follows
//! B RR B RR B RR B

#[allow(dead_code, clippy::missing_const_for_fn)]
pub fn count_red_beads(n: u32) -> u32 {
    (n * 2).saturating_sub(2)
}
