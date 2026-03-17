# Requirements

**Project Name:** tree-it  
**Project Manager:** @ciroalo  
**Last Revision Date:** March 17, 2026  
**Version:** 0.1.0  
**Status:** Draft  

Legend:  
[P0] = MVP  
[P1] = First version
[P2] = Second version  
[P3] = Future/optional  

## Functional Requirements (FR)

### CLI Core Behavior

[P0] FR-1 - Default execution

- When the user runs `tree-it` without arguments, the tool analyzes the current working directory.

[P0] FR-2 - Target path execution

- When the user runs `tree-it <path>`, the tool analyzes the specified directory.
- The path must exist and be accessible.

[P0] FR-3 - Invalid path handling

- If the provided path does not exist or cannot be accessed:
    - The tool must print a clear error message.
    - The tool must exit with a non-zero status code.

[P0] FR-4 - Root node inclusion

- The output must include the root directory name as the first node of the tree.

[P0] FR-5 - Deterministic output

- Same input + same configuration must always produce identical output.

[P0] FR-6 - Default ordering

- Directories must be listed before files
- Entries must be sorted alphabetically.
- Sorting must be case-insensitive.

[P0] FR-7 - Empty directories

- Empty directories must be included in the output.

[P0] FR-8 - Symbolic links

- Symbolic links must be ignored.
- They must not appear in the output.
- The tool must not traverse symbolic links.

[P0] FR-9 - File and directory indicators

- The output must visually distinguish directories from files.

## Ignore and Configuration

[P0] FR-10 - Ignore source resolution

- Priority order:
    1. `.treeignore`
    2. `.gitignore`
    3. no ignore rules
- If `.treeignore` exists, `.gitignore` must be ignored.

[P0] FR-11 - Ignore file lookup location

- Ignore files must be resolved in the target directory

[P0] FR-12 - `.treeignore` capabilities

- Must support:
    - global exclusions
    - profiles
    - tags (future)

[P0] FR-13 - Global exclusions

- Lines outside variables must be treated as exclusion patterns.

[P0] FR-14 - `.gitignore` fallback

- Used only for exclusion.
- No profiles or tags supported

[P0] FR-15 - Pattern matching semantics

- Must follow `.gitignore`-style matching.

[P0] FR-16 - Pattern scope

- Patterns must match anywhere in the tree.

[P0] FR-17 - No re-inclusion

- Negative patterns (e.g. `!file`) are not supported in P0.

[P0] FR-18 - Profile definition

- Profiles must be defined using variables starting with `tree`.

[P0] FR-19 - Profile selection

- CLI must support `--profile <name>`.

[P0] FR-20 - Missing profile

- Must error and exit non-zero.

[P0] FR-21 - Profile requires `.treeignore`

- Must error if not found `.treeignore` and exit with non-zero.


## Tree Generation and Rendering

[P0] FR-22 - Ignored directory behavior

- Ignored directories must not appear in output
- Must not be traversed

[P0] FR-23 - Ignored file behavior

- Ignored files must not appear in output

[P0] FR-24 - Hidden entries

- Hidden files and directories must be excluded by default.

[P0] FR-25 - Empty directories

- Must be rendered if not excluded

[P0] FR-26 - Symbolic links

- Must be ignored completely

[P0] FR-27 - Rendering style

- Must use Unicode box-drawingn characters

[P0] FR-28 - Deterministic rendering

- Output must remain stable after filtering


# Profiles

[P0] FR-29 - Supported CLI options

- Only `--profile <name>` is supported in MVP.

[P0] FR-30 - Single profile execution

- Only one profile may be selected at a time

[P0] FR-31 - Profile name handling

- Must be case-insensitive
- Internally normallized to lowercase

[P0] FR-32 - Default execution behavior

- Without `--profile`, the tool must generate:

    - the general tree
    - all profile trees

[P0] FR-33 - Profile resolution errors

- Missing profile must produce an error and exit.

## Tags (Future behavior)

[P0] FR-34 - Global tags

- `.treeignore` must support global CLI-style tags.

[P1] FR-35 - Profile tags

- Must merge with global tags.
- Must override conflicting values.

[P1] FR-36 - CLI tag precedence

- CLI > profile > global

[P1] FR-37 - Unknown tags

- Must be treated as errors.


## Image Output (Future behavior)

[P2] FR-38 - Image generation

- The tool must support exporting the tree output as an image.

[P2] FR-39 - Image consistency

- Image output must reflect the same structure as terminal output

[P2] FR-40 - Output format

- At least one image format must be supported (e.g. PNG).

[P2] FR-41 - Styling

- Image must be readable and suitable for documentation


## Non-Functional Requirements (NFR)

[P0] NFR-01 - Performance

- Must handle typical project directories without noticeable delay (<1s for small/medium projects)

[P0] NFR-02 - Determinism

- Output must be consistent across runs and environments.

[P0] NFR-03 - Reliability

- Tool must not crash on invalid input.
- All errors must be handled gracefully.

[P0] NFR-04 - Cross-platform support

- Must run on macOS, Linux, Windows.

[P0] NFR-05 - Distribution

- Must be distributed as a single binary.

[P0] NFR-06 - Maintainability

- Codebase must follow modular architecture.

    - parsing
    - traversal
    - filtering
    - rendering

[P0] NFR-07 - Usability

- CLI must be simple and intuitive.
- Error messages must be clear and actionable.

[P0] NFR-08 - Open-source readiness

- Clean repository structure
- Clear README
- Versioned releases

[P1] NFR-09 - Extensibility

- Architecture must allow adding:

    - new renderers (image)
    - new CLI options
    - new config features

[P2] NFR-10 - Scalability

- Must handle large directory trees efficiently