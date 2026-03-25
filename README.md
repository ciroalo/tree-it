# tree-it

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
cargo build --release
```

The binary will be available at:

```bash
target/release/tree-it
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
TBD