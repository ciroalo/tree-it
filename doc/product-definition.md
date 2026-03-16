# Product Definition

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 16, 2026  
**Version:** 0.1.0  
**Status:** Accepted  

Working title: **Tree Generator CLI Tool**

## Product Overview

The Tree Generator CLI Tool (`tree-it`) is a command-line application that generates a visual representation of a directory structure.

The tool is designed primarily for **software developers who need to visualize and document project structures**. It extends the capabilites of traditional tools like the `tree` command by introducing a configuration-driven approach through a `.treeignore` file.

Using this configuration file, developers can:

- exclude files and directories from a tree
- define multiple reusable tree profiles
- configure CLI behavior using tags
- generate consistent project tree outputs for documentation and development workflows 

The tool prioritizes terminal tree output, with additional output formats (such as images) planned for later versions.

## Problem Statement

Developers frequently need to show the structure of a project in documentation, communication, or analysis tasks. Existing tools such as the `tree` command can generate directory trees but lack flexible configuration for generating multiple views of the same project.

As a result, developers often manually edit tree outputs or repeatedly apply different command flags.

This tool solves the problem by introducing a project-specific configuration file that enables consistent and reusable tree generation workflows.


## Target Users

**Primary User**

- me, the creator. For my own personal projects

**Secondary Users**

- software developers documenting repositories
- open-source maintainers
- technical writers
- teams that need reproducible tree representations of projects

## Core Value Proposition

The tool provides a configurable, documentation-friendly directory tree generator that:

- behaves similarly to the classic `tree` command
- uses a `.treeignore` configuration file
- allows multiple tree profiles from a single configuration source
- supports configurable CLI options through tags
- enables future output formats (such as images) without redesigning the core architecture

## Ignore File Resolution Strategy

The tool determines exclusion rules using the following priority order:

1. `.treeignore`
2. `.gitignore`
3. no ignore rules

### Resolution Rules

- If `.treeignore` exists in the analyzed directory, it is used as the primary configuration file
- When `.treeignore` is present, `.gitignore` is ignored completely
- if `.treeignore` does not exist but `.gitignore` does, `.gitignore`rules are used for file exclusion
- If neither files exists, the tool generates a complete directory tree

### Lookup Location

Ignore files are searched in the target directory being analyzed

Example:

```bash
tree-it ./project
```

Lookup occurs in:

```bash
./project/.treeignore
./project/.gitignore
```

If not path is provided:
```bash
tree-it
```

The current working directory is used.

## `.treeignore` Configuration Concept

The `.treeignore` file acts as both"

- an ignore file
- a configuration source for tree profiles and tags

The syntax uses variable assignment.

## Global Exclusion Rules
Lines outside variable assignment represent global exclusion patterns.

These patterns apply to every tree output.

Example:

```bash
node_modules/
dist/
.git/
```

## Tree Profiles

Tree profiles allow generating multiple filtered views of the same project.

Profiles are defined using variables assignments whose names begin with `tree`

Examples:

```bash
tree_docs = [
    "tests/",
    ".github/"
]

tree_public = [
    "internal/",
    ".env"
]
```

**Behavior**

Default tree:
```bash
global excludes only
```

Profile tree:
```bash
global excludes + profile excludes
```

Example CLI usage of only one profile tree:
```bash
tree-it --profile tree_docs
```


## Tag System

Tags represent CLI-style options that influence how the tree is generated or displayed

Tags can be defined globally or per profile

### Global Tags

Example:

```bash
tags = [
    "--level=3",
    "--dirsfirst"
]
```

These apply to all trees unless overridden.

### Profile Tags

Profiles may define their own tag set.

Example:

```bash
tree_docs_tags = [
    "--level=2",
    "--ascii"
]
```

These override relevant global tag values or add to them when the profile is used.

## Configuration Precedence

When generating a tree, the effective configuration is determined by the following priority:

```bash
CLI flags > profile tags > global tags
```

Exclusion rules follow this order:

```bash
global exclusions + profile exclusions
```

## Output Formats
**Version 1 priority**
1. Terminal tree output

**Planned extensions**
2. Image tree output
3. Tree view functionality

## Design Philosophy

The project will follow these architectural principles:

- simple first version
- modular architecture
- separation of concerns

Core responsibilities will likely include:

- configuration parsing
- directory traversal
- filtering logic
- output rendering

This structure ensurs that additional output formats and features can be introduced without redesigning the core system.

## Development Priorities

Feature development priority:

1. Terminal tree generation
2. Image tree generation
3. Tree view functionality
4. Advanced tag functionality