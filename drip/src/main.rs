use std::path::{Path, PathBuf};
use std::{env, fs};

use clap::{Parser, Subcommand, ValueHint};
use miette::{miette, IntoDiagnostic};

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Bundle { recipe_dir } => {
            // We get `recipe_dir` either from the user via cli, or from the env
            // var if not specified by the user.
            let recipe_dir = match recipe_dir {
                Some(dir) => dir,
                None => env::var("DRIP_RECIPE_DIR")
                    .map(|dir| Path::new(&dir).to_path_buf())
                    .map_err(|_| miette!("$DRIP_RECIPE_DIR is not set"))?,
            };

            let recipe = fs::read_to_string(recipe_dir.join("recipe.kdl"))
                .into_diagnostic()?;

            let recipe = drip::parse_recipe("recipe.kdl", &recipe)?;
            drip::install(&recipe_dir, recipe).into_diagnostic()?;
        }

        Command::Diff { recipe_dir } => {
            // We get `recipe_dir` either from the user via cli, or from the env
            // var if not specified by the user.
            let recipe_dir = match recipe_dir {
                Some(dir) => dir,
                None => env::var("DRIP_RECIPE_DIR")
                    .map(|dir| Path::new(&dir).to_path_buf())
                    .map_err(|_| miette!("$DRIP_RECIPE_DIR is not set"))?,
            };

            let recipe = fs::read_to_string(recipe_dir.join("recipe.kdl"))
                .into_diagnostic()?;

            let recipe = drip::parse_recipe("recipe.kdl", &recipe)?;
            drip::print_diff(recipe).into_diagnostic()?;
        }
    };

    Ok(())
}

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install the Recipe bundle.
    #[command(visible_alias = "b")]
    Bundle {
        /// The source Recipe dir.
        #[arg(short = 'd', long, value_hint = ValueHint::DirPath)]
        recipe_dir: Option<PathBuf>,
    },

    /// Print difference between the source Recipe and local formulas.
    #[command(visible_alias = "d")]
    Diff {
        /// The source Recipe dir.
        #[arg(short = 'd', long, value_hint = ValueHint::DirPath)]
        recipe_dir: Option<PathBuf>,
    },
}
