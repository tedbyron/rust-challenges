#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::molecule_to_atoms::parse_molecule;

fn main() {
    println!("{:?}", parse_molecule("[C2(NO3)3]2"));
}
