use std::path::PathBuf;

use knuffel::Decode;
use miette::Diagnostic;

/// Error type for when parsing a `Recipe` fails.
#[derive(Debug, Diagnostic, thiserror::Error)]
#[error("failed to parse Recipe")]
#[diagnostic(transparent)]
pub struct ParseError(#[from] knuffel::Error);

/// Tries to parse a `Recipe` from the given input string.
///
/// ## Errors
/// Returns an error if the input isn't in valid `Recipe` format.
pub fn parse_recipe(
    file_name: &str,
    input: &str,
) -> Result<Recipe, ParseError> {
    Ok(knuffel::parse(file_name, input)?)
}

/// The root `Recipe` container.
#[derive(Debug, Decode)]
pub struct Recipe {
    /// List of taps in the recipe.
    #[knuffel(children(name = "tap"), unwrap(argument))]
    pub taps: Vec<String>,

    /// List of cask names in the recipe.
    #[knuffel(children(name = "cask"), unwrap(argument))]
    pub casks: Vec<String>,

    /// List of formulas in the recipe.
    #[knuffel(children(name = "brew"))]
    pub formulas: Vec<Formula>,
}

/// A formula consisting of a name and (optionally) some postinstall steps.
#[derive(Debug, Decode)]
pub struct Formula {
    /// Name of the formula.
    #[knuffel(argument)]
    pub name: String,

    /// List of postinstall steps.
    #[knuffel(children)]
    pub postinstall_steps: Vec<Postinstall>,
}

/// Postinstall steps of a formula.
#[derive(Debug, Decode)]
#[non_exhaustive]
pub enum Postinstall {
    /// Copies `.0` to `.1`.
    Cp(#[knuffel(argument)] PathBuf, #[knuffel(argument)] PathBuf),

    /// Downloads `.0` to `.1`.
    Dl(#[knuffel(argument)] String, #[knuffel(argument)] PathBuf),

    /// Appends `.0` to file in `.1`.
    Echo(#[knuffel(argument)] String, #[knuffel(argument)] PathBuf),

    /// Runs some `fish` command `.0`.
    Fish(#[knuffel(argument)] String),
}
