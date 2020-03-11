mod _2020;

use _2020::molecule_to_atoms::parse_molecule;

fn main() {
    println!("{:?}", parse_molecule("K4[ON(SO3)2]2"));
    println!("{:?}", parse_molecule("Mg(OH"));
    println!("{:?}", parse_molecule("Mg)OH("));
    println!("{:?}", parse_molecule("Mg(OH}2"));
}
