// for a given chemical formula represented by a string, return the number of
// of atoms of each element in the molecule

use regex::Regex;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

/// check if a string slice has balanced parentheses
fn is_balanced(s: &str) -> bool {
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

/// using a compiled regular expression, get its matches from a string slice
fn get_matches(r: &Regex, s: &str) -> Option<Vec<String>> {
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

/// get the components of a chemical formula in the form of a string slice
fn get_components(s: &str) -> Result<Vec<String>, ParseError> {
    if !is_balanced(s) {
        return Err(ParseError {});
    }

    match get_matches(
        &Regex::new(r"([A-Z][a-z]?\d*)+|[\(\[\{]|[\)\]\}]\d*").unwrap(),
        s,
    ) {
        Some(c) => Ok(c),
        None => Err(ParseError {}),
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let components = get_components(s)?;
    let subcomponent_pattern = Regex::new(r"[A-Z][a-z]?\d*").unwrap();

    println!("{:?}", get_matches(&subcomponent_pattern, &components[4]));

    Err(ParseError {})
}
