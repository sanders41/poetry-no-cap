import poetry_no_cap
import pytest
import tomlkit


def test_load_toml(tmp_path):
    pyproject_toml = tomlkit.loads(
        """[tool.poetry]
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
    """
    )

    file_path = tmp_path / "pyproject_toml"

    with open(file_path) as f:
        tomlkit.dump(pyproject_toml, f)

    poetry_no_cap.load_toml(file_path, file_path)

    with open(file_path) as f:
        result = tomlkit.load(f)

    expected = tomlkit.loads(
        """[tool.poetry]
name = "test"
version = "0.1.0"
description = "Test"
authors = ["Paul Sanders <psanders1@gmail.com>"]
license = "MIT"
readme = "README.md"

[tool.poetry.dependencies]
python = ">=3.8"
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
        """
    )

    assert result == expected
