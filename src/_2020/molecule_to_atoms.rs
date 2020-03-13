//! for a given chemical formula represented by a string, return the number of
//! of atoms of each element in the molecule

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMPONENT_PATTERN: Regex =
        Regex::new(r"[A-Z][a-z]?\d*|[\(\[\{]|[\)\]\}]\d*").unwrap();
    static ref PAREN_PATTERN: Regex = Regex::new(r"[\(\)\[\]\{\}]").unwrap();
    static ref PAREN_L_PATTERN: Regex = Regex::new(r"[\(\[\{]").unwrap();
    static ref PAREN_R_PATTERN: Regex = Regex::new(r"[\)\]\}]").unwrap();
    static ref NUM_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

/// get the matches of a compiled regular expression from a string slice
fn get_matches(r: &Regex, s: &str) -> Vec<String> {
    r.find_iter(s).map(|m| m.as_str().to_string()).collect()
}

/// get the multiplier associated with an element or group of elements
fn get_multiplier(s: &str) -> (String, usize) {
    let m = get_matches(&NUM_PATTERN, s);
    let n = match m.len() {
        1 => m[0].parse::<usize>().unwrap(),
        _ => 1,
    };

    (s.replace(&n.to_string(), ""), n)
}

/// check if a string slice has parentheses that are balanced and in order
fn is_valid(s: &str) -> bool {
    let mut v = vec![];

    for s in PAREN_PATTERN.find_iter(s).map(|m| m.as_str()) {
        if PAREN_L_PATTERN.is_match(&s) {
            v.push(s);
        } else {
            if let Some(p) = v.pop() {
                match (&p[..], &s[..]) {
                    ("(", ")") | ("[", "]") | ("{", "}") => continue,
                    _ => return false,
                }
            } else {
                return false;
            }
        }
    }

    v.len() == 0
}

/// get the components of a chemical formula represented by a string slice
fn get_components(s: &str) -> Result<Vec<String>, ParseError> {
    if !is_valid(s) {
        return Err(ParseError {});
    }

    let c = get_matches(&COMPONENT_PATTERN, s);

    if c.is_empty() {
        Err(ParseError {})
    } else {
        Ok(c)
    }
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    let mut molecule_dups = vec![];
    let mut molecule = vec![];
    let mut multiplier = 1;
    let mut multipliers = vec![1];

    for (component, count) in get_components(s)?.iter().map(|s| get_multiplier(&s)).rev() {
        if PAREN_R_PATTERN.is_match(&component) {
            multipliers.push(count);
            multiplier *= count;
        } else if PAREN_L_PATTERN.is_match(&component) {
            multiplier /= multipliers.pop().unwrap();
        } else {
            molecule_dups.push((component, count * multiplier));
        }
    }

    for (element, count) in molecule_dups.into_iter() {
        if let Some((_, n)) = molecule.iter_mut().find(|(e, _)| e == &element) {
            *n += count;
        } else {
            molecule.push((element, count));
        }
    }

    Ok(molecule)
}
