# Contributing to Aurorite

---
Welcome! We are excited to see your interest in contribution to this project! 

## Project setup

---
The repository consists of two parts: `ui` and Rust `crates`. Make sure you have rust toolchain installed if you plan
to develop API, or node.js for Vue development. You need to clone the repository on local machine.

## Tasks to complete

---

Currently team is focused on providing basic functionality. Nevertheless we will be grateful for any help!
There is a lot of undocumented code, so contributions related to docs will be appreciated.

## Write a commit message

---

Commit messages should follow the [Semantic Commit Messages](https://www.conventionalcommits.org/en/v1.0.0/) format:

```
label(namespace): title

description

footer
```

1. *label* is one of the following:
   - `fix` - bug fixes
   - `feat` - new features
   - `docs` - documentation-only changes
   - `test` - test-only changes
   - `ci` - changes to the CI or build
   - `chore` - everything that doesn't fall under previous categories
2. *namespace* is put in parentheses after label and is one of the following:
   - `ui` - ui (aka frontend) related changes
   - `data` - aurorite-dataflow crate changes
   - `runtime` - aurorite-vismut crate changes
   - `util` - aurorite-util crate changes
   - `agsp` - aurorite-agsp crate changes
   - no *namespace* for aurorite-server changes
3. *title* is a brief summary of changes.
4. *description* is **optional**, new-line separated from title and is in present tense.
5. *footer* is **optional**, new-line separated from *description* and contains "fixes" / "references" attribution to GitHub issues.

Example:

```
feat: add new endpoint for dice rolls

This patch adds a new handler for rolls to the API.

Fixes #123, references #234.
```