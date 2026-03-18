# ADR-002: Implement an In-House Ignore Matcher with a Defined P0 Subset

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 17, 2026  
**Version:** 0.1.0  
**Status:** Draft  

## Context

`tree-it` depends heavily on ignore rule evaluation.

Ignore matching is required for:

- `.treeignore` global exclusions
- `.treeignore` profile exclusions
- `.gitignore` fallback behavior

The product definition establishes:

- `.treeignore` as the primary configuration source
- `.gitignore` as a fallback exclusion source
- profile-based filtered tree generation
- deterministic traversal and rendering behavior

The project also requires:

- modular architecture
- cross-platform behavior
- long-term maintainability
- predictable filtering rules

A major technical decision is whether to reuse an existing ignore-matching library 
or implement mathing logic in-house.

## Decision

`tree-it` will implement its ignore matcher in-house.

For P0, the matcher will support a documented subset of `.gitignore`-inspired behavior
rather than claiming full Git-compatible semantics.

## Rationale

### 1. Product ownership of matching behavior

Ignore logic is a core part of the product, not an incidental dependency.

`tree-it` introduces:

- `.treeignore`
- profile-based exclusion layers
- custom rule resolution behavior
- future tag-driven configuration

Implementing matching in-house give the project full control over how ignore logic 
integrates with the tool's domain model.

### 2. Architectural clarity

An in-house matcher encourages clean separation between:

- parsing ignore sources
- compiling patterns 
- normalizing paths
- evaluating matches
- applying filtering decisions during traversal

This supports the modular architecture already intended for the project.

### 3. Learning and long-term extensibility

Because `tree-it` is being built as a serious, production-quality tool, implementing the 
matcher directly provides a stronger understanding of:

- rules systems
- path normalization
- matching semantics
- filtering boundaries

This also makes future extensions easier to reason about, including:

- richer `.treeignore` features
- tag-driven filtering behavior
- optional future support for negation or expanded syntax

### 4. Controlled complexity through a scoped subset

Implementing a matcher in-house is a risk if the project promises full `.gitignore` 
compatibility too early.

To reduce this risk, P0 will support a documented subset only.

This keeps the feature implementable while preserving correctness and transparency.

## P0 supported Matcher Scope

The in-house matcher shall support:

- `*` wildcard
- `*` wildcard
- `*` multi-directory wildcard
- trailing `/` as directory-only matching
- matching against a normalized relative paths
- path separator normalization to `/` internally on all platforms
- patterns matching anywhere in the analyzed tree
- pruning ignored directories from traversal

## P0 Explicitly Out of Scope

The matcher shall not support in P0:

- negation / re-inclusion patterns such as `!file.txt`
- advanced escaping semantics
- full Git edge-case compatibility
- undocumented pattern behavior

## Path Normalization Rules

Before matching:

- all candidate paths shall be mae relative to the analyzed root
- all separators shall be normalized to `/`
- matching shall use normalized paths consistently across macOS, Linux and Windows

## Consequences

### Positive

- full control over ignored behavior
- strong alignment with the internal config and filtering model
- easier future extension of `.treeignore`
- better architectural separation 
- reduced dependency on external library behavior

### Negative

- higher implementation complexity on P0
- larger testing burden 
- greater risk of edge-case bugs if semantics are not documented carefully
- requires explicit documentation of supported vs unsupported patterns

## Implementation Notes

The implementation should be separated into distinct concerns, likely including:

- ignore file parsing
- pattern representation / compilation
- normalized path representation
- matcher evaluation
- traversal-time filtering
