/*!
Narc is a dependently-typed programming language with Agda
style dependent pattern matching.

# Goal

The purpose of this language is to realize the elaboration algorithm described
in [this paper][paper].

 [paper]: https://dl.acm.org/citation.cfm?id=3236770
 [Agda]: http://hackage.haskell.org/package/Agda-2.6.0.1

The implementation is heavily inspired from [Agda version 2.6.0.1][Agda].

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
