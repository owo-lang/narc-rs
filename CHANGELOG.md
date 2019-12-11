# Change Log

# 0.0.6

+ Fix a bunch of index-relevant problems
  + Add local DBI lifting
  + Lift DBI when generate `AsBind`
  + Lift meta context during unification
  + Lift DBI when substituting inside `Closure`
+ Add Agda's `AddContext Telescope` instance (`TCS::under`)
+ Lhs-splitting now support variables
+ Pretty print `Abs`
+ Support tracing the type-checking process

# 0.0.5

+ Migrate from CircleCI + AppVeyor to GitHub Actions
+ Check datatype declaration (#15)
+ Check constructor declaration (#17)
+ Test desugar
+ Check declaration list (#19)
  + Projection & Codata are missing
+ The CLI now check files, there are tests now
+ Don't require `:` in front of constructor tele
+ Rename `ast_cons` to `ast_util`
+ Add `Ident` info to `Term::Whnf`
+ Unfold constructor applications (#27)
+ Introduce `DeBruijn` trait
+ WIP LHS checking (equations, lhs-results, lhs-states, etc.)
  + Pattern splitting is missing
+ `Bind` now has optional `val` field

# 0.0.4

+ Conversion check (#3)
  + Subtyping for universe types
  + Covariance for pi types' return parts
  + Solve metas
+ Ast traversal functions (#5)
+ Local context lookup by uid (#7)
+ `Val::App` is renamed to `Val::Var`
+ Fix evaluation for app (#12)
+ Inference now respect implicit parameters (#24)
+ Inference produces well-typed term as well (#23)
+ Application inference now supports projections (#25)
+ Unfold data and codata applications (#27)
+ Parsing (expr parsing and file parsing) (#20, #31)
+ `Abs::App` is now chained (#33)
+ Desugar surface into abstract, scope check (#36, #42, #43)

# 0.0.3

+ Add the substitution type
+ Implement the substitution operation
+ Document the design of this language (#2)
+ Initial type-checking code, including:
  + Application inference
  + Core language's definition improvements

# 0.0.2

+ Add core language definition from Voile,
  with row-polymorphic terms and sigma type removed (#1)
+ Initial CLI support

# 0.0.1

+ Create package on crates.io
