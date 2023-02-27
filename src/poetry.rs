use std::process::{exit, Command};

use anyhow::Result;

use crate::pyproject::{is_poetry_project, recreate_pyproject};

/// Add a new dependency and remove the upper bound cap.
pub fn add(args: String, pin: bool) {
    match is_poetry_project() {
        Ok(v) => {
            if !v {
                eprintln!("\nThis project does not appear to be using Poetry");
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error checking the pyproject.toml file: {}", err);
            exit(1);
        }
    }
    let mut full_args = vec!["add"];
    let split_args = args.split(' ');
    for split in split_args {
        full_args.push(split);
    }

    println!("\nRunning poetry add\n");
    match run_poetry_add(full_args) {
        Ok(_) => println!("poetry add complete\n"),
        Err(err) => {
            eprint!("Error with adding dependency: {err}");
            exit(1);
        }
    };

    println!("Removing caps from pyproject.toml file");
    if let Err(err) = recreate_pyproject(false, pin) {
        eprintln!("{err}");
        exit(1);
    };

    println!("\nUpdating poetry.lock file\n");
    match run_poetry_lock() {
        Ok(_) => println!("\npoetry lock complete\n"),
        Err(err) => {
            eprint!("Error updating lock file: {err}");
            exit(1);
        }
    };
}

/// Update the pyproject.toml file, removing upper bound caps, without adding any new dependencies
pub fn fix(dry_run: bool, pin: bool) {
    match is_poetry_project() {
        Ok(v) => {
            if !v {
                eprintln!("\nThis project does not appear to be using Poetry");
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error checking the pyproject.toml file: {}", err);
            exit(1);
        }
    }

    println!("Removing caps from pyproject.toml file");
    if let Err(err) = recreate_pyproject(dry_run, pin) {
        eprintln!("{err}");
        exit(1);
    };

    if !dry_run {
        println!("\nUpdating poetry.lock file\n");
        match run_poetry_lock() {
            Ok(_) => println!("\npoetry lock complete\n"),
            Err(err) => {
                eprint!("Error updating lock file: {err}");
                exit(1);
            }
        };
    }
}

/// Update depdendencies and remove the upper bound cap.
pub fn update(pin: bool) {
    match is_poetry_project() {
        Ok(v) => {
            if !v {
                eprintln!("\nThis project does not appear to be using Poetry");
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error checking the pyproject.toml file: {}", err);
            exit(1);
        }
    }

    println!(
        "\nRemoving caps from pyproject.toml file to ensure nothing is capped durring update\n"
    );
    if let Err(err) = recreate_pyproject(false, pin) {
        eprintln!("{err}");
        exit(1);
    };

    println!("Running poetry update\n");
    match run_poetry_update() {
        Ok(_) => println!("poetry update complete\n"),
        Err(err) => {
            eprint!("Error with updating dependencies: {err}");
            exit(1);
        }
    };

    println!("Removing caps from pyproject.toml file");
    if let Err(err) = recreate_pyproject(false, pin) {
        eprintln!("{err}");
        exit(1);
    };

    println!("\nUpdating poetry.lock file\n");
    match run_poetry_lock() {
        Ok(_) => println!("\npoetry lock complete\n"),
        Err(err) => {
            eprint!("Error updating lock file: {err}");
            exit(1);
        }
    };
}

fn run_poetry_add(full_args: Vec<&str>) -> Result<()> {
    let mut poetry_add = Command::new("poetry").args(full_args).spawn()?;
    poetry_add.wait()?;

    Ok(())
}

fn run_poetry_lock() -> Result<()> {
    let mut poetry_lock = Command::new("poetry")
        .args(["lock", "--no-update"])
        .spawn()?;
    poetry_lock.wait()?;

    Ok(())
}

fn run_poetry_update() -> Result<()> {
    let mut poetry_update = Command::new("poetry").args(["update"]).spawn()?;
    poetry_update.wait()?;

    Ok(())
}
