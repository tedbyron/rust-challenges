// for a given chemical formula represented by a string, return the number of
// of atoms of each element in the molecule

use regex::Regex;

type Atom = (String, usize);
type Molecule = Vec<Atom>;

#[derive(Debug)]
pub struct ParseError {}

fn get_components(s: &str) -> Result<Vec<String>, ParseError> {
    let parts = Regex::new(r"[A-Z]{1}[a-z]?\d*|[\(\[\{]|[\)\]\}]\d*").unwrap();
    Err(ParseError {})
}

pub fn parse_molecule(s: &str) -> Result<Molecule, ParseError> {
    Err(ParseError {})
}
