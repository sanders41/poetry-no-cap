use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Result};

pub fn recreate_pyproject(
    pyproject_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> Result<()> {
    let pyproject_file = match pyproject_path {
        Some(f) => f,
        None => PathBuf::from("pyproject.toml"),
    };

    let contents = match fs::read_to_string(&pyproject_file) {
        Ok(c) => c,
        Err(err) => {
            bail!(
                "Could not read {}: {}",
                pyproject_file.to_string_lossy(),
                err
            );
        }
    };

    let processed = format_file(&contents);
    let output = match output_path {
        Some(f) => f,
        None => pyproject_file,
    };

    if let Err(err) = fs::write(output, processed) {
        bail!("Could not write pyproject.toml file: {}", err);
    };

    Ok(())
}

fn format_file(pyproject: &str) -> String {
    let mut processed = String::from("");
    let mut current_key: Option<&str> = None;

    for token in pyproject.split("\n") {
        if token.starts_with("[") && token.ends_with("]") {
            current_key = Some(token);
            processed = format!("{processed}{token}\n");
            continue;
        }
        if let Some(c) = current_key {
            // Poetry is unhappy if you set python to >= and other poetry packages haven't done
            // the same.
            if c.contains("poetry") && !token.starts_with("python =") {
                processed = format!("{processed}{}\n", token.replace("^", ">="));
            } else {
                processed = format!("{processed}{token}\n");
            }
        } else {
            processed = format!("{processed}{token}\n");
        }
    }

    processed
}
