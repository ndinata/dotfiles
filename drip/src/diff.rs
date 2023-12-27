use std::collections::HashSet;
use std::process::Command;

use crate::Recipe;

#[derive(Debug, thiserror::Error)]
#[error("command '{cmd}' failed: {reason}")]
pub struct CommandError {
    pub cmd: &'static str,
    pub reason: String,
}

/// Prints the difference between the recipe and locally-installed formulas.
///
/// ## Errors
/// Returns an error if running `brew leaves` fails.
pub fn print_diff(recipe: Recipe) -> Result<(), CommandError> {
    let recipe_formulas: HashSet<String> = HashSet::from_iter(
        recipe.formulas.into_iter().map(|formula| formula.name),
    );

    let local_formulas = local_formulas()?;

    let recipe_not_local = recipe_formulas
        .difference(&local_formulas)
        .collect::<Vec<_>>();

    let local_not_recipe = local_formulas
        .difference(&recipe_formulas)
        .collect::<Vec<_>>();

    if !recipe_not_local.is_empty() {
        println!(
            "{} formula(s) present in Recipe but not installed:",
            recipe_not_local.len()
        );
        println!("{:?}\n", recipe_not_local);
    }

    if !local_not_recipe.is_empty() {
        println!(
            "{} formula(s) installed locally but not recorded in Recipe:",
            local_not_recipe.len()
        );
        println!("{:?}\n", local_not_recipe);
    }

    Ok(())
}

/// Gets the set of locally-installed formula names.
fn local_formulas() -> Result<HashSet<String>, CommandError> {
    let output = Command::new("brew")
        .arg("leaves")
        .arg("-r")
        .output()
        .map_err(|err| CommandError {
            cmd: "brew leaves",
            reason: err.to_string(),
        })?;

    Ok(String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect())
}
