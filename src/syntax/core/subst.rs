use super::{Elim, Term, Val};

/// Substitution.
pub type Subst = Vec<Elim>;

impl Term {
    /// Use `Term` instead of `Self` to emphasize that it's not `Elim`.
    pub fn apply(self, args: Vec<Term>) -> Self {
        self.apply_elim(args.into_iter().map(Elim::app).collect())
    }

    pub fn apply_elim(self, mut args: Vec<Elim>) -> Self {
        match match self {
            Term::Whnf(val) => val,
            Term::Redex(def, a) => unimplemented!(),
        } {
            Val::App(f, mut a) => {
                a.append(&mut args);
                Term::Whnf(Val::App(f, a))
            }
            Val::Meta(m, mut a) => {
                a.append(&mut args);
                Term::meta(m, a)
            }
            Val::Cons(c, mut a) => {
                for arg in args {
                    match arg {
                        Elim::App(arg) => a.push(*arg),
                        Elim::Proj(field) => panic!("Does not support record constructors yet!"),
                    }
                }
                Term::cons(c, a)
            }
            e => panic!("Cannot eliminate `{}`.", e),
        }
    }
}
