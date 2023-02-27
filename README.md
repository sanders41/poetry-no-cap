# Poetry No Cap

![Tests Status](https://github.com/sanders41/poetry-no-cap/workflows/Testing/badge.svg?branch=main&event=push)

[Poetry](https://github.com/python-poetry/poetry) is a packaging and dependency management tool for
python projects that I like to use, but I want to handle my depenence versioning differntly and there
isn't a good way to do that. using `poetry add ...` adds dendencies with capped upper bounds. This
can cause unnecessary dependency conflicts for libraries, so I want to specify no upper bounds. Then
for applications I want to pin specific versions of packages rather than giving a range. This package
aims to fix this issue.

## Usage

### Adding new dependencies

#### To add a dependency with no upper bounds:

```sh
poetry-no-cap add meilisearch-python-async
```

This will result in the latest version of `meilisearch-python-async` being added in the following
format.

```toml
[tool.poetry.dependencies]
meilisearch-python-async = ">=1.0.0"
```

#### To add a pinned dependency:

```sh
poetry-no-cap add meilisearch-python-async -p
```

This will result in the latest version of `meilisearch-python-async` being added in the following
format.

```toml
[tool.poetry.dependencies]
meilisearch-python-async = "1.0.0"
```

#### To add a dependency with no upper bounds and extras:

Notice the use of quotes here. This is to let `poetry-no-cap` know the `-E` is part of the poetry
command, and not a separate argument for `poetry-no-cap`.

```sh
poetry-no-cap add "camel-converter -E pydantic"
```

This will result in the latest version of `camel-converter` being added in the following
format.

```toml
[tool.poetry.dependencies]
camel-converter = {version = ">=3.0.0", extras = ["pydantic"]}
```

#### To add a pinned dependency with extras:

Notice the use of quotes here. This is to let `poetry-no-cap` know the `-E` is part of the poetry
command, and not a separate argument for `poetry-no-cap`.

```sh
poetry-no-cap add "camel-converter -E pydantic" -p
```

This will result in the latest version of `camel-converter` being added in the following
format.

```toml
[tool.poetry.dependencies]
camel-converter = {version = "3.0.0", extras = ["pydantic"]}
```

### Update the pyproject.toml file without adding any new dependencies

#### Update the pyproject.toml file with no upper cap on dependencies

```sh
poetry-no-cap fix
```

#### Update the pyproject.toml file with pinned dependencies

```sh
poetry-no-cap fix -p
```

#### Dry run updating the pyproject.toml file with no upper cap on dependencies

Running as a dry run will print the updated file to the screen instead of saving it.

```sh
poetry-no-cap fix -d
```

### Command help

Help for commands is available by running:

```sh
poetry-no-cap --help
```

## Project Status

Currently this package is experimental. If testing goes well I will release a version that is easily
installable. If you want to help test now (this would be greatly appreciated) you can use the
following steps to get setup.

1. Fork and clone this repositry
1. Change into the cloned `poetry-no-cap` directory
1. Install the package

  ```sh
  cargo install --path .
  ```

After this `poetry-no-cap` will be available for you to run on your python projects.

## Contributing

Contributions to this project are welcome. If you are interesting in contributing please see our [contributing guide](CONTRIBUTING.md)
