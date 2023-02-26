mod pyproject;

use std::path::PathBuf;
use std::process::{exit, Command};

use pyo3::prelude::*;

use pyproject::recreate_pyproject;

#[pyfunction]
fn add(
    args: String,
    pyproject_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> PyResult<()> {
    let mut full_args = vec!["add"];
    let split_args = args.split(" ");
    for split in split_args {
        full_args.push(split);
    }
    let mut poetry_add = Command::new(format!("poetry"))
        .args(full_args)
        .spawn()
        .expect("Error running poetry add");
    match poetry_add.wait() {
        Ok(_) => println!("\npoetry add complete\n"),
        Err(err) => {
            eprint!("Error with adding dependency: {err}");
            exit(1);
        }
    };

    println!("Removing caps from pyproject.toml file");
    if let Err(err) = recreate_pyproject(pyproject_path, output_path) {
        eprintln!("{err}");
        exit(1);
    };

    println!("\nUpdating poetry.lock file\n");
    let mut poetry_lock = Command::new("poetry")
        .args(["lock", "--no-update"])
        .spawn()
        .expect("Error updating lock file");
    match poetry_lock.wait() {
        Ok(_) => println!("\npoetry lock complete\n"),
        Err(err) => {
            eprint!("Error updating lock file: {err}");
            exit(1);
        }
    };

    Ok(())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn poetry_no_cap(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
