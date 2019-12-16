use super::super::{Elim, Term, Val};
use voile_util::{loc::Ident, uid::GI};

impl Term {
    /// Use `Term` instead of `Self` to emphasize that it's not `Elim`.
    pub fn apply(self, args: Vec<Term>) -> Self {
        self.apply_elim(args.into_iter().map(Elim::app).collect())
    }

    pub fn apply_elim(self, mut args: Vec<Elim>) -> Self {
        match self {
            Term::Whnf(Val::Var(f, mut a)) => {
                a.append(&mut args);
                Term::Whnf(Val::Var(f, a))
            }
            Term::Whnf(Val::Meta(m, mut a)) => {
                a.append(&mut args);
                Term::meta(m, a)
            }
            Term::Whnf(Val::Cons(c, mut a)) => {
                let mut iter = args.into_iter();
                match iter.next() {
                    None => Term::cons(c, a),
                    Some(Elim::App(arg)) => {
                        a.push(*arg);
                        Term::cons(c, a).apply_elim(iter.collect())
                    }
                    Some(Elim::Proj(field)) => {
                        let mut fields = c.fields.iter().enumerate();
                        let msg = "Undefined field projected!";
                        let (ix, _) = fields.find(|(_, s)| *s == &field).expect(msg);
                        // Remove as we no longer need this `a` -- we only care about one field.
                        a.remove(ix).apply_elim(iter.collect())
                    }
                }
            }
            Term::Redex(f, id, a) => def_app(f, id, a, args),
            e => panic!("Cannot eliminate `{}`.", e),
        }
    }
}

/// Application on a definition.
/// [Agda](https://hackage.haskell.org/package/Agda-2.6.0.1/docs/src/Agda.TypeChecking.Substitute.html#defApp).
pub fn def_app(f: GI, id: Ident, mut a: Vec<Elim>, mut args: Vec<Elim>) -> Term {
    /* // Does not support projection using application syntax.
    match args.first() {
        Some(Elim::App(arg)) => {
            let mut iter = args.into_iter();
            let fst = iter.next().unwrap().into_app().unwrap();
        }
        _ => Term::Redex(f,name, args),
    }
    */
    a.append(&mut args);
    Term::Redex(f, id, a)
}
