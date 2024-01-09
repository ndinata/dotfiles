use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use knuffel::{traits::ErrorSpan, Decode, DecodeScalar};
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
    Cp(
        #[knuffel(argument)] ExpandedPath,
        #[knuffel(argument)] ExpandedPath,
    ),

    /// Downloads `.0` to `.1`.
    Dl(
        #[knuffel(argument)] String,
        #[knuffel(argument)] ExpandedPath,
    ),

    /// Appends `.0` to file in `.1`.
    Echo(
        #[knuffel(argument)] String,
        #[knuffel(argument)] ExpandedPath,
    ),

    /// Runs some `fish` command `.0`.
    Fish(#[knuffel(argument)] String),
}

/// Newtype for a `PathBuf` with its `~` expanded into the full home dir path.
///
/// The expansion actually takes place when doing `raw_decode()`.
#[derive(Debug)]
pub struct ExpandedPath(PathBuf);

impl Deref for ExpandedPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for ExpandedPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<OsStr> for ExpandedPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

impl<S: ErrorSpan> DecodeScalar<S> for ExpandedPath {
    fn raw_decode(
        value: &knuffel::span::Spanned<knuffel::ast::Literal, S>,
        ctx: &mut knuffel::decode::Context<S>,
    ) -> Result<Self, knuffel::errors::DecodeError<S>> {
        let pathbuf = PathBuf::raw_decode(value, ctx)?;

        Ok(Self(simple_home_dir::expand_tilde(pathbuf).unwrap()))
    }

    fn type_check(
        type_name: &Option<knuffel::span::Spanned<knuffel::ast::TypeName, S>>,
        ctx: &mut knuffel::decode::Context<S>,
    ) {
        PathBuf::type_check(type_name, ctx)
    }
}
