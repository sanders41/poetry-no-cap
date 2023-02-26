use std::fs;
use std::path::PathBuf;
use std::process::exit;

use anyhow::{bail, Result};

struct PyprojectFile {
    file_path: PathBuf,
    contents: String,
}

impl PyprojectFile {
    fn load_pyproject_file() -> Result<PyprojectFile> {
        let file_path = PathBuf::from("pyproject.toml");
        let contents = fs::read_to_string(&file_path)?;

        Ok(PyprojectFile {
            file_path,
            contents,
        })
    }
}

fn format_file(pyproject: &str, pin: bool) -> String {
    let mut processed = String::from("");
    let mut current_key: Option<&str> = None;

    for token in pyproject.split('\n') {
        if token.starts_with('[') && token.ends_with(']') {
            current_key = Some(token);
            processed = format!("{processed}{token}\n");
            continue;
        }
        if let Some(c) = current_key {
            // Poetry is unhappy if you set python to >= and other poetry packages haven't done
            // the same.
            if c.contains("poetry") && !token.starts_with("python =") {
                if pin {
                    processed = format!(
                        "{processed}{}\n",
                        token.replace('^', "").replace(">=", "").replace('~', "")
                    );
                } else {
                    processed = format!("{processed}{}\n", token.replace('^', ">="));
                }
            } else {
                processed = format!("{processed}{token}\n");
            }
        } else {
            processed = format!("{processed}{token}\n");
        }
    }

    processed.pop(); // Remove the extra \n
    processed
}

/// Check the pyproject.toml file for the presents on a `tool.poetry*` key
pub fn is_poetry_project() -> Result<bool> {
    let pyproject_file = PyprojectFile::load_pyproject_file()?;

    Ok(pyproject_file.contents.contains("tool.poetry"))
}

/// Recreates the pyproject.toml file removing capped upper bounds.
pub fn recreate_pyproject(dry_run: bool, pin: bool) -> Result<()> {
    match PyprojectFile::load_pyproject_file() {
        Ok(v) => {
            let contents = v.contents;
            let processed = format_file(&contents, pin);

            if dry_run {
                println!("\nPerforming dry run, skipping save\n");
                println!("{processed}");
            } else {
                match fs::write(v.file_path, processed) {
                    Ok(_) => println!("\nUpdated pyproject.toml file saved"),
                    Err(err) => bail!("Could not write pyproject.toml file: {}", err),
                };
            }
        }
        Err(err) => {
            eprintln!("Error reading the pyproject.toml file: {}", err);
            exit(1);
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::format_file;

    fn pyproject_toml_str() -> String {
        String::from(
            r#"[tool.poetry]
name = "test"
version = "0.1.0"
description = "Test"
authors = ["Paul Sanders <psanders1@gmail.com>"]
license = "MIT"
readme = "README.md"

[tool.poetry.dependencies]
python = "^3.8"
camel-converter = {version = "^3.0.0", extras = ["pydantic"]}
meilisearch-python-async = "^1.0.0"
twilio-python-async = "^0.2.0"

[tool.poetry.group.dev.dependencies]
black = "^23.1.0"
isort = "^5.12.0"
mypy = "^1.0.0"
pre-commit = "^3.0.4"
pytest = "^7.2.1"
pytest-cov = "^4.0.0"
tox = "^4.4.5"
ruff = "^0.0.247"

[build-system]
requires = ["poetry-core>=1.0.0"]
build-backend = "poetry.core.masonry.api"

[tool.black]
line-length = 100
include = '\.pyi?$'
exclude = '''
/(
    \.egg
  | \.git
  | \.hg
  | \.mypy_cache
  | \.nox
  | \.tox
  | \.venv
  | \venv
  | _build
  | buck-out
  | build
  | dist
  | setup.py
)/
'''

[tool.isort]
profile = "black"
line_length = 100
src_paths = ["test", "tests"]

[tool.mypy]
check_untyped_defs = true
disallow_untyped_defs = true

[[tool.mypy.overrides]]
module = ["tests.*"]
disallow_untyped_defs = false

[tool.pytest.ini_options]
minversion = "6.0"
addopts = "--cov=test --cov-report term-missing"

[tool.ruff]
select=["E", "F", "T201", "T203"]
ignore=["E501"]
}
"#,
        )
    }

    #[test]
    fn test_format_file_no_pin() {
        let expected = r#"[tool.poetry]
name = "test"
version = "0.1.0"
description = "Test"
authors = ["Paul Sanders <psanders1@gmail.com>"]
license = "MIT"
readme = "README.md"

[tool.poetry.dependencies]
python = "^3.8"
camel-converter = {version = ">=3.0.0", extras = ["pydantic"]}
meilisearch-python-async = ">=1.0.0"
twilio-python-async = ">=0.2.0"

[tool.poetry.group.dev.dependencies]
black = ">=23.1.0"
isort = ">=5.12.0"
mypy = ">=1.0.0"
pre-commit = ">=3.0.4"
pytest = ">=7.2.1"
pytest-cov = ">=4.0.0"
tox = ">=4.4.5"
ruff = ">=0.0.247"

[build-system]
requires = ["poetry-core>=1.0.0"]
build-backend = "poetry.core.masonry.api"

[tool.black]
line-length = 100
include = '\.pyi?$'
exclude = '''
/(
    \.egg
  | \.git
  | \.hg
  | \.mypy_cache
  | \.nox
  | \.tox
  | \.venv
  | \venv
  | _build
  | buck-out
  | build
  | dist
  | setup.py
)/
'''

[tool.isort]
profile = "black"
line_length = 100
src_paths = ["test", "tests"]

[tool.mypy]
check_untyped_defs = true
disallow_untyped_defs = true

[[tool.mypy.overrides]]
module = ["tests.*"]
disallow_untyped_defs = false

[tool.pytest.ini_options]
minversion = "6.0"
addopts = "--cov=test --cov-report term-missing"

[tool.ruff]
select=["E", "F", "T201", "T203"]
ignore=["E501"]
}
"#;
        assert_eq!(format_file(&pyproject_toml_str(), false), expected);
    }

    #[test]
    fn test_format_file_pin() {
        let expected = r#"[tool.poetry]
name = "test"
version = "0.1.0"
description = "Test"
authors = ["Paul Sanders <psanders1@gmail.com>"]
license = "MIT"
readme = "README.md"

[tool.poetry.dependencies]
python = "^3.8"
camel-converter = {version = "3.0.0", extras = ["pydantic"]}
meilisearch-python-async = "1.0.0"
twilio-python-async = "0.2.0"

[tool.poetry.group.dev.dependencies]
black = "23.1.0"
isort = "5.12.0"
mypy = "1.0.0"
pre-commit = "3.0.4"
pytest = "7.2.1"
pytest-cov = "4.0.0"
tox = "4.4.5"
ruff = "0.0.247"

[build-system]
requires = ["poetry-core>=1.0.0"]
build-backend = "poetry.core.masonry.api"

[tool.black]
line-length = 100
include = '\.pyi?$'
exclude = '''
/(
    \.egg
  | \.git
  | \.hg
  | \.mypy_cache
  | \.nox
  | \.tox
  | \.venv
  | \venv
  | _build
  | buck-out
  | build
  | dist
  | setup.py
)/
'''

[tool.isort]
profile = "black"
line_length = 100
src_paths = ["test", "tests"]

[tool.mypy]
check_untyped_defs = true
disallow_untyped_defs = true

[[tool.mypy.overrides]]
module = ["tests.*"]
disallow_untyped_defs = false

[tool.pytest.ini_options]
minversion = "6.0"
addopts = "--cov=test --cov-report term-missing"

[tool.ruff]
select=["E", "F", "T201", "T203"]
ignore=["E501"]
}
"#;
        assert_eq!(format_file(&pyproject_toml_str(), true), expected);
    }
}
