
# Development Roadmap

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 18, 2026  
**Version:** 0.1.0  
**Status:** Accepted  

## 1. Goal

Define the implementation sequence for `tree-it` so development progresses in a 
controlled, testable and maintainable way.

This roadmap prioritizes:

- early stabilization of core domain logic
- short feedback loops
- clear milestones boundaries
- delivery of a focused P0 (MVP)

## 2. Milestones

### Milestone 0 - Repository foundation

**Objective**  

Create the initial repository structure and documentation baseline.

**Deliverable**

- Initialize rust project
- Create repo folders
    - `src/`
    - `docs/`
    - `docs/architecture`
    - `docs/architecture/decisions`
    - `tests/`
- Add initial project documents:
    - product definition
    - requirements
    - architecture notes
    - adr-001
    - adr-002
- Add `.gitignore`

**Done Criteria**

- Repository builds successfully
- Project structure exists
- Documentation is comitted and organized

### Milestone 1 - Core Models and App Skeleton

#### Objective

Create the foundational domain models and the top-level application skeleton

#### Deliverable

- Add core models:
    - `CliRequest`
    - `ConfigSource`
    - `ParsedConfig`
    - `ProfileConfig`
    - `TreeJob`
    - `EffectiveIgnoreConfig`
    - `EntryKind`
    - `FsEntry`
    - `TreeNode`
- Add `app::run`
- Add top-level `AppError`
- Keep `main.rs` thin

#### Done Criteria

- All major domain models compile
- main.rs calls app::run
- Project builds succesfully

### Milestone 2 - Path Utilities

#### Objective 

Implement shared path normalization behavior

#### Deliverables

- normalize paths relative to analyzed root
- normalize separtors to `/`
- add helpers for:
    - hidden-entry detection
    - display-name extraction

#### Done Criteria

- path utility tests pass
- behavior is consisten across representative path inputs

### Milestone 3 - Config Source Resolution

#### Objective

Implement detection of .treeignore, .gitignore, or no config source

#### Deliverables

- resolve config source with priority
    - .treeignore
    - .gitignore
    - none
- restrict lookup to target directory only

#### Done criteria

- source resolution tests pass
- resolution behavior matches requirements

### Milestone 4 - Config Parsing

#### Objective 

Parse .treeignore and .gitignore into the normalized config model

#### Deliverables

- parse .gitignore as exclusion-only
- parse .treeignore global exclusions
- parse .treeignore profiles
- normalize profile names to lowercase
- return structured parse errors for invalid syntax

#### Done Criteria

- valid examples parse correctly
- invalid examples fail cleanly
- .gitignore and .treeignore both map to ParsedConfig

### Milestone 5 - Execution Planning and Ignore Composition

#### Objective

Translate parsed configuration into executable tree jobs

#### Deliverables

- build general job
- build all profile jobs for default execution
- build single profile job for --profile
- validate missing profile errors
- merge exclusions inot EffectiveIgnoreConfig

#### Done Criteria

- job planning tests pass
- general/profile effective patterns are correct

### Milestone 6 - Ignore Matcher v0

#### Objective

Implement the in-house matcher for the P0 subset.

#### Deliverables

- support *
- support ?
- support **
- support trailing `/` as directory-only
- match against normalized relative paths
- use first-match-wins behavior
- do not support negation

#### Done Criteria

- matcher unit tests pass
- directory-only and nested matching work correctly
- unsupported constructs are rejected or documented clearly


### Milestone 7 - Filesytem Traversal and Filtering

#### Objective

Walk the filesystem and apply inclusion/exclusion rules

#### Deliverables

- traverse directories 
- exclude symlinks
- exclude hidden entries by default
- apply matcher-based exclusion
- prune ignored directories
- sort entries deterministically
    - directories first
    - case-insensitive alphabetical ordering

#### Done Criteria

- traversal/filtering tests pass against fixtures
- ignored directories disappear completely
- hidden entries and symlinks are excluded


### Milestone 8 - Tree building

#### Objective

Build the in-memory tree structure from filtered entries

#### Deliverables 

- create root node
- insert directories and files correctly
- preserve empty directories 
- preserve deterministic ordering in tree nodes

#### Done Criteria

- tree-building tests pass
- output structure matches expected fixtures

### Milestone 9 - Terminal rendering v0

#### Objective

Render the tree model into terminal output

#### Deliverables

- unicode box-drawing style output
- clear directory/file distinction
- stable, deterministic formatting
- render a single tree cleanly

#### Done Criteria

- renderer golden tests pass
- output is readable and consistent

### Milestone 10 - End-to-end Integration

#### Objective

Wire the full application pipeline together

#### Deliverables

- full app::run flow
- correct success/failure exit behavior
- clear error printing
- deafult multi-output mode
- single-profile mode

#### Done Criteria

- integration tests pass for:
    - default execution
    - profile execution
    - .gitignore fallback
    - invalid path
    - invalid profile
    - no-config execution


### Milestone 11 - P0 hardening

#### Objective

Stabilize MVP quality before first tagged release

#### Deliverable

- expand test coverage
- improve error messages
- update README usage examples
- document matcher subset explicitly
- sync docs with implementation
- clean up module boundaries if needed 

#### Done Criteria

- p0 requirements are implemented and documented
- known limitations are explicit
- repository is ready for first MVP release

## 3. Implementation Order

1. repository scaffold
2. core models
3. path utilities
4. config source resolution
5. config parsing
6. job planning
7. matcher
8. filesystem traversal and filtering
9. tree building
10. terminal rendering
11. app orchestration
12. integration tests and polish

## 4. Out of Scope for P0

- tag behavior
- image output
- alternate renderers
- summary/footer output
- advanced CLI flags
- negation in ignored matching

## 5. Exit Condition

P0 is complete when:

- the tool generates terminal tree output
- .treeignore and .gitignore resolution work correctly
- profiles work according to requirements
- filtering is deterministic
- hidden entries and symlinks are excluded 
- errors are handled clearly
- documentation reflects implented behavior