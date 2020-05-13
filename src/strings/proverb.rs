//! given a list of inputs, return the proverb

#[allow(dead_code, clippy::module_name_repetitions)]
pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .chain(
            list.iter()
                .next()
                .map(|&i| format!("And all for the want of a {}.", i)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
