//! given the name of a programmer, return 18 if the name is "Zach" because he
//! owns 18 lightsabers, or else 0

#[allow(dead_code)]
pub fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    match name {
        "Zach" => 18,
        _ => 0,
    }
}
