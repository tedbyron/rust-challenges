// for a given chemical formula represented by a string, return the number of
// of atoms of each element in the molecule

use regex::Regex;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

fn has_balanced_parens(s: &str) -> bool {
    let paren_pattern = Regex::new(r"[\(\)\[\]\{\}]").unwrap();
    let parens = paren_pattern
        .find_iter(s)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let (parens_l, parens_r) = parens.split_at(parens.len() / 2);

    if parens_r
        .iter()
        .rev()
        .enumerate()
        .any(|(i, slice)| match slice {
            &")" => parens_l[i] != "(",
            &"]" => parens_l[i] != "[",
            &"}" => parens_l[i] != "{",
            _ => true,
        })
    {
        return false;
    }

    true
}

fn get_subcomponents(r: &Regex, s: &str) -> Option<Vec<String>> {
    let subcomponents = r
        .find_iter(s)
        .map(|m| m.as_str().to_string())
        .collect::<Vec<String>>();

    if subcomponents.len() > 0 {
        Some(subcomponents)
    } else {
        None
    }
}

fn get_components(s: &str) -> Result<Vec<String>, ParseError> {
    if !has_balanced_parens(s) {
        return Err(ParseError {});
    }

    let component_pattern = Regex::new(r"([A-Z][a-z]?\d*)+|[\(\[\{]|[\)\]\}]\d*").unwrap();
    match get_subcomponents(&component_pattern, s) {
        Some(c) => Ok(c),
        None => Err(ParseError {}),
    }
}

// fn get_multiplier(r: &Regex, s: &str) -> i32 {
//     0
// }

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let components = get_components(s)?;
    let subcomponent_pattern = Regex::new(r"[A-Z][a-z]?\d*").unwrap();

    println!(
        "{:?}",
        get_subcomponents(&subcomponent_pattern, &components[4]) // components
    );

    Err(ParseError {})
}
