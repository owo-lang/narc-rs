# narc-rs

[![Crates.io](https://img.shields.io/crates/d/nar.svg)][crates]
[![Crates.io](https://img.shields.io/crates/v/nar.svg)][lib-rs]
[![Crates.io](https://img.shields.io/crates/l/nar.svg)][crates]
[![docs.rs](https://docs.rs/nar/badge.svg)][doc-rs]
[![cc-svg]][cc-url]
[![Build status](https://ci.appveyor.com/api/projects/status/wu6vpjhn094gd93g/branch/master?svg=true)][av-url]
[![dep-svg]][dep-rs]

 [crates]: https://crates.io/crates/nar/
 [lib-rs]: https://lib.rs/nar/
 [cc-svg]: https://circleci.com/gh/owo-lang/narc-rs/tree/master.svg?style=svg
 [cc-url]: https://circleci.com/gh/owo-lang/narc-rs/tree/master
 [doc-rs]: https://docs.rs/nar
 [dep-rs]: https://deps.rs/repo/github/owo-lang/narc-rs
 [dep-svg]: https://deps.rs/repo/github/owo-lang/narc-rs/status.svg
 [plugin]: https://github.com/owo-lang/intellij-dtlc/
 [av-url]: https://ci.appveyor.com/project/ice1000/narc-rs/branch/master

Narc is a dependently-typed programming language with Agda style dependent pattern matching.
It's called "Narc", but a rust crate `narc` has already been registered,
thus the crate name is changed to `nar` and the compiler binary is therefore `narc`.
For language description, please head to the [docs.rs][doc-rs] page.

## Resources

+ [Docs.rs][doc-rs] documentation
+ [Change Log](CHANGELOG.md), useful resource for tracking language evolution

## Install

You can install the narc type-checker by this command
(cargo installation and rust stable toolchain are assumed):

```bash
cargo install nar --bin narc
```

After installation, you can type-check a narc file by:

```bash
narc [filename]
```

You can also start a REPL:

```bash
narc -i
```
