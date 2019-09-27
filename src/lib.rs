/*!
Narc is a dependently-typed programming language with Agda
style dependent pattern matching.

# Goal

The purpose of this language is to realize the elaboration algorithm described
in [this paper][paper].

 [paper]: https://dl.acm.org/citation.cfm?id=3236770
 [Agda]: http://hackage.haskell.org/package/Agda-2.6.0.1

The implementation is heavily inspired from [Agda version 2.6.0.1][Agda].

I want this language to have:

+ Conversion check should be nominal for simplicity
+ Only case-tree instantiation need to be supported
+ Surface syntax should be considerate of parsing ease
+ Simple (co)inductive types (not indexed) with an identity type as
  described in Jesper's paper
  + It's not actually (co)inductive --
    there's no termination or productivity checks yet
+ Definition by pattern matching according to Jesper's paper
+ Coverage check + case-tree generation described in Jesper's paper
+ Prefix (applying on projection) *and* postfix (projecting from data)
  projection (or maybe not?)

... and I plan to enhance everything in the *next language after* Narc,
including (but not limited to):

+ Pattern instantiation, to see if we can prove things easier
+ Structural conversion check (or a partial one, like in mlang)
+ Totality check: termination/productivity
+ ... more?

... and even next language in the future:

+ Indexed data families, remove the built-in identity type
+ IDE mode like `agda2-mode`,
  but I'll go for both Code (primary) and Emacs (secondary)
+ De-morgan cubical primitives (Interval, Path, hcomp, transport, Glue)
+ ... more?

<br/>
<span>
<details>
<summary>About the name, Narc</summary>
<span>
This name is inspired from a friend whose username is <a
href="https://www.zhihu.com/people/wu-liang-95-71"><em>Narc</em></a> (or
<em>lwoo1999</em> on
<a href="https://www.codewars.com/users/lwoo1999">CodeWars</a> and
<a href="https://github.com/lwoo1999">GitHub</a>).
</span>
</details></span>
*/

/// Core language, abstract syntax, surface syntax, and the parser.
/// Corresponds to Agda's `Agda.Syntax`.
pub mod syntax;

/// Anything relevant to type-checking.
/// Corresponds to Agda's `Agda.TypeChecking`.
pub mod check;
