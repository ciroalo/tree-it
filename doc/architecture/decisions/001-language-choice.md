# ADR-001: Choose Rust as the Primary Implementation Language

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 17, 2026  
**Version:** 0.1.0  
**Status:** Accepted  


## Context 

`tree-it` is a CLI-first developer tool designed to generate structured directory trees with configurable behavior.

The product has the following key characteristics:

- CLI-based tool with terminal output as v1
- Configuration-driven via `.treeignore` and `.gitignore`
- Custom configuration language supporting:
    - variables
    - profiles
    - tags
    - precedence rules
- Core responsibilities
    - configuration parsing
    - filesystem traversal
    - filtering logic
    - tree modeling
    - output rendering
- Designed to be modular, allowing future renderers (e.g. image output)

Additionally, the project has explicit non-functional goals:

- Long-term maintainability
- Robustness as the feature set grows
- Cross-platform support (macOS, Linux, Windows)
- Potential to evolve into a more advanced developer tool over time

## Decision

We will implement `tree-it` in **Rust**.


## Rationale

1. **Long-term robustness**

The project is expected to grow beyond a simple CLI into a tool with:

- a custom configurable language
- multiple sources of truth (`.treeignore`, `.gitignore`, CLI flags)
- precedence and override rules
- extensible rendering capabilities

Rust's type system and compile-time guarantees provide:

- strong correctness guarantees
- safer refactoring as complexity increases
- reduced likelihood of runtime errors

This is particularly valuable for logic-heavy areas such as:

- config parsing
- rule resolution
- filtering behavior


2. **Maintainability as complexity grows**

`tree-it` is not just I/O - it contains domain logic

Rust enforces:

- explicit error handling
- clear ownership of data
- strong boundaries between models

This aligns well with the intended architecture:

- config -> parsed model -> effective settings
- traversal -> tree structure
- filtering -> deterministic rules
- rendering -> pluggable outputs

The stricter model reduces "implicit behavior" that could otherwise accumulate over time.

3. **Cross-platform distribution**

Rust compiles to native binaries for:

- macOS
- Linux
- Windows

This allows:

- single-binary distribution
- easy installation for users
- no runtime dependencies

This is critical for developer tooling, where frictionless installation is expected.

4. **Architectural alignment with modular design**

The product definition requires a clean separation of concerns:

- parsing
- traversal
- filtering
- rendering

Rust's strengths in:

- enums
- traits
- pattern matching

make it well-suited for modeling:

- configuration states
- rule systems
- renderer abstractions

This supports future extensibility without major refactors.

5. **Performance characteristics (secondary but beneficial)**

While performance is not the primary driver, Rust provides:

- efficient filesystem traversal
- low memory overhead
- predictable performance

This ensures the tool remains responsive even on large directories

## Alternatives Considered

### Go

**Pros:**

- Faster development speed
- Simpler language
- Excellent CLI ecosystem
- Very easy cross-platform distribution

**Cons:**

- Less strict guarantees around correctness
- More reliance on developer discipline for long-term maintainability
- Less expressive type system for modeling complex domain logic

**Reasons not choosen:**

Go is an excellent choice for CLI tools, but given the expected growth in configuration complexity and rule handling, Rust provides a stronger long-term foundation.

### Python

**Pros:**

- Fasters prototyping
- Very easy to implement parsers and logic

**Cons:**

- Distribution is harder (no single binary by default)
- Runtime errors more likely
- Less suitable for long-term CLI tooling at scale

**Reasons not choosen:**  
Does not meet robustness and distribution goals.

### Typescript/Node.js

**Pros:**

- High developer productivity
- Large ecosystem
- Good for rapid iteration

**Cons:**

- Requires runtime (Node.js)
- Packaging into a single binary is less straightforward
- Weaker guarantees for long-term correctness

**Reasons not choosen:**

Does not align as well with the goal of a robust, standalone CLI tool.

### #C/.NET

**Pros:**

- Strong tooling
- Good architecture support
- Cross-platform via .NET

**Cons:**

- Larger runtime footprint
- Less common choice for lightweight CLI tools in this space

**Reasons not choosen:**

Not as aligned with the lightweight distribution and ecosystem expectations for developer CLI tools.


## Consequences

### Positive

- Strong foundation for long-term evolution
- Safer refactoring as features grow
- Clear architectural boundaries 
- High-quality cross-platform binaries
- Increased confidence in correctness

### Negative

- Slower initial development
- Higher learning curve
- More effort required for early iterations
- Potential friction when implementing recursive or mutable structures