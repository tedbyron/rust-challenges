mod _2020;

use _2020::molecule_to_atoms::parse_molecule;

fn main() {
    println!("{:?}", parse_molecule("pie"));
    println!("{:?}", parse_molecule("Mg(OH("));
    println!("{:?}", parse_molecule("Mg(OH"));
    println!("{:?}", parse_molecule("Mg)OH("));
    println!("{:?}", parse_molecule("Mg(OH}2"));
    println!("{:?}", parse_molecule("MgOH"));
    println!("{:?}", parse_molecule("{[Co(NH3)4(OH)2]3Co}(SO4)3"));
    println!("{:?}", parse_molecule("OK4[ON(SOOx3)2]2"));
}
