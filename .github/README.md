# narc-rs

[![Crates.io](https://img.shields.io/crates/d/nar.svg)][crates]
[![Crates.io](https://img.shields.io/crates/v/nar.svg)][lib-rs]
[![Crates.io](https://img.shields.io/crates/l/nar.svg)][crates]
[![docs.rs](https://docs.rs/nar/badge.svg)][doc-rs]
[![Actions Status][ga-svg]][ga-url]
[![dep-svg]][dep-rs]

 [crates]: https://crates.io/crates/nar/
 [lib-rs]: https://lib.rs/nar/
 [doc-rs]: https://docs.rs/nar
 [dep-rs]: https://deps.rs/repo/github/owo-lang/narc-rs
 [dep-svg]: https://deps.rs/repo/github/owo-lang/narc-rs/status.svg
 [plugin]: https://github.com/owo-lang/intellij-dtlc/
 [paper]: https://dl.acm.org/citation.cfm?id=3236770
 [ga-svg]: https://github.com/owo-lang/narc-rs/workflows/build/badge.svg
 [ga-url]: https://github.com/owo-lang/narc-rs/actions

Narc is a dependently-typed programming language with Agda style dependent pattern matching.
It's called "Narc", but a rust crate `narc` has already been registered,
thus the crate name is changed to `nar` and the compiler binary is therefore `narc`.
For language description, please head to the [docs.rs][doc-rs] page.

## Resources

+ [Original Paper][paper] that Narc is based on
+ [Docs.rs][doc-rs] documentation
+ [IntelliJ Plugin][plugin], which can export your code as clickable HTML
+ [Change Log](../CHANGELOG.md), useful resource for tracking language evolution

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

## Progress

+ [X] Basic dependent type (minitt/voile things)
+ [ ] Data type and codata types
+ [ ] Definition and clauses checking
+ [ ] Universe level support
+ [ ] Implicit arguments
