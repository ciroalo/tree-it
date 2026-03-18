# Architecture Notes

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 18, 2026  
**Version:** 0.1.0  
**Status:** Accepted  

## 1. Overview

`tree-it` is a CLI tool that generates a visual representation of a directory structure.

The system is designed around a modular pipeline architecture, where each stage is 
responsible for a well-defined transformation:

```
CLI -> Config -> Ignore -> Matcher -> FS -> Filter -> Tree -> Render
```

The tool prioritizes:

- deterministic behavior
- modularity
- separation of concerns
- extensibility for future features (image output, tags, etc.)


## 2. Module Decomposition

The system is divided inot the following modules:

### `cli`

Handles CLI argument parsing and user input.

Responsibilities:

- parse command-line arguments
- normalize profile names
- construct the initial request


### `app`

Owns application orchestration.

Responsibilities:

- coordinate execution flow
- connect all modules
- handle error propagation
- produce final output

### `config`

Handles configuration source detection and parsing

Responsibilities:

- resolve `.treeignore` / `.gitignore` / none
- parse configuration files
- produce normalized configuration models

### `ignore`

Owns ignore rule composition.

Responsibilities:

- combine global and profile exclusions
- produce effective ignore configuration per execution

### `matcher`

Implements ignore pattern matching

Responsibilities:

- compile ignore patterns
- normalize paths
- evaluate pattern matches

Notes:

- implemented in-house
- supports a defined subset of `.gitignore` semantics in P0


### `fs`

Handles filesystem traversal

Responsibilities:

- walk the directory structure
- collect file and directory entries
- avoid symlink traversal

### `filter`

Applies inclusion and exclusion rules

Responsibilities:

- exclude hidden files
- apply ignore matcher
- prune ignored directories
- determine final set of entries


### `tree`

Builds the in-memory representation of the directory tree

Responsibilities:

- represent directories and files
- preserve hierarchy
- maintain deterministic ordering


### `render`

Handles output generation

Responsibilities:

- convert tree into terminal output
- use Unicode box-drawing characters
- support future renderers (e.g. image output)


### `shared`

Contain shared utilities 

Responsibilities:

- path normalization
- cross-module helpers


## 3. Core Data Flow

The system follows a linear pipeline:

```
CliRequest
-> ConfigSource resolution
-> ParsedConfig
-> TreeJob planning
-> EffectiveIgnoreConfig
-> CompiledMatcher
-> FileSystem traversal (FsEntry)
-> Filtered entries
-> TreeNode
-> Rendered output
```

Each stage transforms data into a more refined representation

## 4. Domain Models

### CLI

```rust
struct CliRequest {
    target_path: PathBuf,
    profile: Option<String>,
}
```

### Config

```rust
enum ConfigSource {
    TreeIgnore(PathBuf),
    GitIgnore(PathBuf),
    None,
}

struct ParsedConfig {
    global_excludes: Vec<String>,
    profiles: Vec<ProfileConfig>,
    global_tags: Vec<String>,
}

struct ProfileConfig {
    name: String,
    excludes: Vec<String>,
    tags: Vec<String>,
}
```

### Execution Planning

```rust
Struct TreeJob {
    label: String,
    effective_ignore: EffectiveIgnoreConfig,
}
```

### Ignore

```rust
struct EffectiveIgnoreConfig {
    patterns: Vec<String>,
}
```

### Matcher

```rust
struct CompiledMatcher {
    patterns: Vec<CompiledPattern>,
}

struct CompiledPattern {
    raw: String,
    directory_only: bool,
}
```

### Filesystem

```rust
enum EntryKind {
    Directory,
    File,
}

struct FsEntry {
    relative_path: String,
    name: String,
    kind: EntryKind,
}
```

### Tree

```rust
enum TreeNode {
    Directory(DirectoryNode),
    File(FileNode),
}

struct DirectoryNode {
    name: String,
    children: Vec<TreeNode>,
}

struct FileNode {
    name: String,
}
```

## 5. Execution Modes

### Default execution

Command:

```bash
tree-it [path]
```

Behavior:

- generate general tree (global exclusions)
- generate all profile trees if `.treeignore` exists


### Profile execution

Command: 

```
tree-it [path] --profile <name>
```

Behavior:

- require `.treeignore`
- generate only the selected profile tree

## 6. Ignore Matching Strategy

- implemented in-house
- inspired by `.gitignore`
- P0 supports a defined subset

Supported: 

- `*`, `?`, `**`
- trailing `/` for directory-only
- normalized relative paths
- matching anywhere in tree

Not supported in P0:

- negation (`!pattern`)
- advanced escaping
- full git compatibility


## 7. Design Principles

### 1. Separation of concerns

Each module has a single responsibility

### 2. Deterministic behavior

Same input and configuration must always produce identical output

### 3. Modular Architecture

Each stage of the pipline can be tested independently

### 4. Extensibility

Future features (image output, tags, new renderers) should not require redesigning the core system 