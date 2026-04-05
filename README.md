# tree-it

![CI](https://github.com/ciroalo/tree-it/actions/workflows/ci.yml/badge.svg)
![Release](https://img.shields.io/github/v/release/ciroalo/tree-it)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![CLI](https://img.shields.io/badge/type-CLI-informational)](#)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue)](#)
[![Status](https://img.shields.io/badge/status-v0.1.0-success)](https://github.com/OWNER/REPO/releases)

![tree-it_banner](/doc/imgs/readme-banner.svg)

`tree-it` is a Rust CLI tool for generating directory trees for documentation and project
visualization.

It is designed as a documentation-friendly alternative to the traditional `tree` command,
with support for:

- global exclusion rules
- reusable tree profiles
- `.treeignore` configuration
- `.gitignore` fallback
- deterministic output

---

## Table of Contents

- [Current Status](#current-status)
- [Installation](#installation)
- [Usage](#usage)
- [Ignore File Resolution](#ignore-file-resolution)
- [`treeignore` Format](#treeignore-format)
- [Example](#example)
- [Matching Behavior](#matching-behavior)
- [Default Behavior](#default-behavior)
- [Project Documentation](#project-documentation)
- [Development](#development)
- [Project Structure](#project-structure)
- [Roadmap](#roadmap)
- [License](#license)
- [Author](#author)

---

## Current Status

This project is currently in MVP stage.

Implemented in MVP:

- terminal tree output
- `treeignore` support
- `.gitignore` fallback
- profile selection with `--profile`
- deterministic ordering
- hidden file exclusion by default
- symlink exclusion
- Unicode tree rendering

Not yet implemented:

- tag behavior
- image output
- alternate renderers
- advanced CLI flags
- negation (`!pattern`) in ignore matching

## Installation

### From source

Clone the repo and inside:

```bash
git clone https://github.com/ciroalo/tree-it.git
cd tree-it
cargo install --path .
```

Then verify:

```bash
tree-it --help
```

If `tree-it` is not found, make sure cargo's bin directory is in your path.

### Install from a release asset

Download the macOS release archive for your machine from the Github Release page, extract it, and move the binary into a directory in your `PATH`.

Example:

```bash
tar -xzf tree-it-v0.1.0-macos-aarch64.tar.gz
mv tree-it /usr/local/bin/tree-it
```


---

## Usage

### Default execution

Run in the current directory:

```bash
tree-it
```

Run against a specific path;

```bash
tree-it ./project
```

Default execution generates:

- general tree
- all profile trees defined in `.treeignore`

### Run a specific profile

```bash
tree-it --profile tree_docs
```

Or with a path

```bash
tree-it ./project --profile tree_docs
```

When developing with Cargo, remember to pass program arguments after `--`:

```bash
cargo run -- --profile tree_docs
cargo run -- ./project --profile tree_docs
```

## Ignore file resolution

`tree-it` uses this priority order:

1. `.treeignore`
2. `.gitignore`
3. no ignore rules

Rules:

- if `.treeignore` exists, it is used and `.gitignore` is ignored
- if only `.gitignore` exists, it is used for exclusions only
- if neither exists, the full tree is generated

Lookup happens only in the analyzed target directory

## `.treeignore` format

`tree-it` supports a variable-assignment configuration style

### Global excludes

Lines outside variable assignments are treated as global exclusion patterns:

```bash
node_modules/
dist/
.git/
```

### Profiles

Profiles are variables whose names begin with `tree`:

```bash
tree_docs = [
    "tests/",
    ".github"
]

tree_public = [
    "internal/",
    ".env"
]
```

### Global tags

Tags are parsed but not yet applied in MVP:

```bash
tags = [
    "--level=2",
    "--ascii"
]
```

### Profile tags

Profile tags are also parsed but not yet applied in MVP:

```bash
tree_docs_tags = [
    "--level=2"
]
```

## Example

Given this `.treeignore`:

```bash
node_modules/
dist/

tree_docs = [
    "tests/",
    ".github/"
]
```

Default execution may produce:

```bash
[general]
project/
├── src/
└── Cargo.toml

[tree_docs]
project/
├── src/
└── Cargo.toml
```

And this command:

```bash
tree-it --profile tree_docs
```

produces only the selected profile output.

---

## Matching behavior

`tree-it` uses an in-house ignore matcher inspired by `.gitignore`.

### Supportedn in MVP

- `*`
- `?`
- `**`
- trailing `/` for directory-only patterns
- matching against a normalized relative paths
- matching anywhere in the tree

### Not supported in MVP

- negation / re-inclusion patterns such as `!file.txt`
- full git compatible edge cases
- advanced escaping semantics

## Default behavior

In MVP:

- hidden files and directories are excluded by default
- symlinks are ignored
- ignored directories disappear completely
- directories are shown before files
- sorting is case-insensitive and deterministic
- terminal output uses Unicode box-drawing characters

## Project Documentation

Additional project documents are available in `docs/`:

- `docs/product-definition.md`
- `docs/requirements.md`
- `docs/architecture.md`
- `docs/adr` for the architecture decisions

--- 

## Development

Run tests:

```bash
cargo test
```

Run the application

```bash
cargo run
```

Run with arguments

```bash
cargo run -- --profile tree_docs
```

Before making any commit or PR request, make sure to run the following (test, lint, formatting):

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Project Structure

MVP: 

```bash
tree-it/
├── doc/
│   ├── architecture/
│   │   ├── decisions/
│   │   │   ├── 001-language-choice.md
│   │   │   └── 002-pattern-matching.md
│   │   └── architecture.md
│   ├── imgs/
│   │   └── readme-banner.svg
│   ├── prd.md
│   ├── product-definition.md
│   └── roadmap.md
├── src/
│   ├── app/
│   │   ├── error.rs
│   │   ├── mod.rs
│   │   └── run.rs
│   ├── cli/
│   │   └── mod.rs
│   ├── config/
│   │   ├── error.rs
│   │   ├── loader.rs
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── parser.rs
│   ├── filter/
│   │   ├── evaluator.rs
│   │   └── mod.rs
│   ├── fs/
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── walker.rs
│   ├── ignore/
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── planner.rs
│   ├── matcher/
│   │   ├── compiler.rs
│   │   ├── error.rs
│   │   ├── evaluator.rs
│   │   ├── mod.rs
│   │   └── pattern.rs
│   ├── render/
│   │   ├── error.rs
│   │   ├── mod.rs
│   │   └── terminal.rs
│   ├── shared/
│   │   ├── mod.rs
│   │   └── path.rs
│   ├── tree/
│   │   ├── builder.rs
│   │   ├── mod.rs
│   │   └── model.rs
│   ├── lib.rs
│   └── main.rs
├── tests/
│   ├── common/
│   │   ├── fixture.rs
│   │   └── mod.rs
│   ├── default_execution.rs
│   ├── git_ignore_fallback.rs
│   ├── invalid_cases.rs
│   └── profile_execution.rs
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Roadmap

Planned future work includes:

- tag support
- image output
- additional CLI options
- alternate renderers
- expanded ignore matching support

## License

MIT License

## Author

Ciro Alonso
