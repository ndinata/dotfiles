use std::path::PathBuf;

use knuffel::Decode;
use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("failed to parse Recipe")]
#[diagnostic(transparent)]
pub struct ParseError(#[from] knuffel::Error);

pub fn parse_recipe(
    file_name: &str,
    input: &str,
) -> Result<Recipe, ParseError> {
    Ok(knuffel::parse(file_name, input)?)
}

#[derive(Debug, Decode)]
pub struct Recipe {
    #[knuffel(children(name = "tap"), unwrap(argument))]
    pub taps: Vec<String>,
    #[knuffel(children(name = "cask"), unwrap(argument))]
    pub casks: Vec<String>,
    #[knuffel(children(name = "brew"))]
    pub formulas: Vec<Formula>,
}

#[derive(Debug, Decode)]
pub struct Formula {
    #[knuffel(argument)]
    pub name: String,
    #[knuffel(children)]
    pub postinstall_steps: Vec<Postinstall>,
}

#[derive(Debug, Decode)]
#[non_exhaustive]
pub enum Postinstall {
    Cp(#[knuffel(argument)] PathBuf, #[knuffel(argument)] PathBuf),
    Dl(#[knuffel(argument)] String, #[knuffel(argument)] PathBuf),
    Echo(#[knuffel(argument)] String, #[knuffel(argument)] PathBuf),
    Fish(#[knuffel(argument)] String),
}
