use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::{Postinstall, Recipe};

#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("cannot install tap '{name}': {reason}")]
    Tap { name: String, reason: String },

    #[error("cannot install formula '{name}': {reason}")]
    Formula { name: String, reason: String },

    #[error("cannot install cask '{name}': {reason}")]
    Cask { name: String, reason: String },

    #[error("command '{cmd}' failed: {reason}")]
    CommandFailed { cmd: String, reason: String },

    #[error("cannot create dir '{dir}': {reason}")]
    CreateDir { dir: String, reason: String },

    #[error("cannot copy '{src}' to '{dst}': {reason}")]
    Copy {
        src: String,
        dst: String,
        reason: String,
    },

    #[error("cannot download '{url}' to '{dst}': {reason}")]
    Download {
        url: String,
        dst: String,
        reason: String,
    },

    #[error("cannot read from or write to '{path}': {reason}")]
    ReadWrite { path: String, reason: String },
}

/// Installs the `Recipe` bundle.
///
/// ## Errors
/// Propagates errors encountered during the install process.
pub fn install(recipe_dir: &Path, recipe: Recipe) -> Result<(), InstallError> {
    let Recipe {
        taps,
        casks,
        formulas,
    } = recipe;

    for tap in taps {
        install_tap(tap)?;
    }

    for formula in formulas {
        install_formula(formula.name)?;

        for step in formula.postinstall_steps {
            run_postinstall_step(step, recipe_dir)?;
        }
    }

    for cask in casks {
        install_cask(cask)?;
    }

    Ok(())
}

/// Runs `brew tap`.
fn install_tap(tap: String) -> Result<(), InstallError> {
    let output =
        Command::new("brew")
            .arg("tap")
            .arg(&tap)
            .output()
            .map_err(|err| InstallError::CommandFailed {
                cmd: "brew tap".to_string(),
                reason: err.to_string(),
            })?;

    if !output.status.success() {
        return Err(InstallError::Tap {
            name: tap,
            reason: String::from_utf8(output.stderr).unwrap(),
        });
    }

    Ok(())
}

/// Runs `brew install`.
fn install_formula(formula: String) -> Result<(), InstallError> {
    let output = Command::new("brew")
        .arg("install")
        .arg(&formula)
        .output()
        .map_err(|err| InstallError::CommandFailed {
            cmd: "brew install".to_string(),
            reason: err.to_string(),
        })?;

    if !output.status.success() {
        return Err(InstallError::Formula {
            name: formula,
            reason: String::from_utf8(output.stderr).unwrap(),
        });
    }

    Ok(())
}

/// Carries out the specified postinstall step.
fn run_postinstall_step(
    step: Postinstall,
    postinstall_dir: &Path,
) -> Result<(), InstallError> {
    match step {
        // Copies `src` to `dst`, creating `dst` (and parent dirs) if it doesn't
        // exist yet. `src` is assumed to be inside `postinstall_dir`.
        Postinstall::Cp(src, dst) => {
            if !dst.is_dir() {
                fs::create_dir_all(&dst).map_err(|err| {
                    InstallError::CreateDir {
                        dir: dst.display().to_string(),
                        reason: err.to_string(),
                    }
                })?;
            }

            let src = postinstall_dir.join(src);
            fs::copy(&src, &dst).map_err(|err| InstallError::Copy {
                src: src.display().to_string(),
                dst: dst.display().to_string(),
                reason: err.to_string(),
            })?;
        }

        // Downloads some file from `url` and places it at `to`.
        Postinstall::Dl(url, to) => {
            let output = Command::new("curl")
                .arg("-fsSLo")
                .arg(&to)
                .arg("--create-dirs")
                .arg(&url)
                .output()
                .map_err(|err| InstallError::CommandFailed {
                    cmd: "curl".to_string(),
                    reason: err.to_string(),
                })?;

            if !output.status.success() {
                return Err(InstallError::Download {
                    url,
                    dst: to.display().to_string(),
                    reason: String::from_utf8(output.stderr).unwrap(),
                });
            }
        }

        // Echoes some string to file at `to`.
        Postinstall::Echo(string, to) => {
            let mut file =
                OpenOptions::new().append(true).open(&to).map_err(|err| {
                    InstallError::ReadWrite {
                        path: to.display().to_string(),
                        reason: err.to_string(),
                    }
                })?;
            writeln!(file, "{}", string).map_err(|err| {
                InstallError::ReadWrite {
                    path: to.display().to_string(),
                    reason: err.to_string(),
                }
            })?;
        }

        // Runs some arbitrary command with `fish`.
        Postinstall::Fish(cmd) => {
            let output =
                Command::new("fish").arg("-c").arg(&cmd).output().map_err(
                    |err| InstallError::CommandFailed {
                        cmd: "fish".to_string(),
                        reason: err.to_string(),
                    },
                )?;

            if !output.status.success() {
                return Err(InstallError::CommandFailed {
                    cmd,
                    reason: String::from_utf8(output.stderr).unwrap(),
                });
            }
        }
    };

    Ok(())
}

/// Runs `brew install --cask`.
fn install_cask(cask: String) -> Result<(), InstallError> {
    let output = Command::new("brew")
        .arg("install")
        .arg("--cask")
        .arg(&cask)
        .output()
        .map_err(|err| InstallError::CommandFailed {
            cmd: "brew install --cask".to_string(),
            reason: err.to_string(),
        })?;

    if !output.status.success() {
        return Err(InstallError::Cask {
            name: cask,
            reason: String::from_utf8(output.stderr).unwrap(),
        });
    }

    Ok(())
}
