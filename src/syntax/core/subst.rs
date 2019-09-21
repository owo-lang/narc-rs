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
            Term::Redex(f, mut a) => {
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
                return Term::Redex(f, a);
            }
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
            e => panic!("Cannot eliminate `{}`.", e),
        }
    }
}
