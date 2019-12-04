use voile_util::loc::{Ident, ToLoc};
use voile_util::tags::Plicit;
use voile_util::uid::{next_uid, GI};

use crate::syntax::abs::{
    Abs, AbsClause, AbsCodataInfo, AbsConsInfo, AbsDataInfo, AbsDecl, AbsDefnInfo, AbsPat,
    AbsProjInfo, AbsTele, Bind,
};
use crate::syntax::common::Ductive;
use crate::syntax::pat::{Copat, Pat};
use crate::syntax::surf::{Expr, ExprCopat, ExprDecl, ExprPat, NamedTele, Param};

use super::{desugar_expr, DesugarErr, DesugarM, DesugarState};

type DeclM<T> = DesugarM<(T, DesugarState)>;

pub fn desugar_decls(state: DesugarState, decls: Vec<ExprDecl>) -> DesugarM {
    decls.into_iter().try_fold(state, desugar_decl)
}

/// Note: this function will not clear the local scope.
pub fn desugar_telescope(
    state: DesugarState,
    signature: NamedTele,
) -> DesugarM<(Ident, AbsTele, DesugarState)> {
    let ident = signature.name;
    let (tele, state) = desugar_params(state, signature.tele)?;
    Ok((ident, tele, state))
}

/// Note: this function will not clear the local scope.
pub fn desugar_params(mut state: DesugarState, params: Vec<Param>) -> DeclM<AbsTele> {
    // The capacity is really guessed. Who knows?
    let mut tele = AbsTele::with_capacity(params.len() + 2);
    for mut param in params {
        let (ty, new_state) = desugar_expr(state, param.ty)?;
        state = new_state;
        let mut intros = |name: Ident, licit: Plicit, ty: Abs| {
            let uid = unsafe { next_uid() };
            state.local.insert(name.text, uid);
            tele.push(Bind::new(licit, uid, ty, None));
        };
        let licit = param.licit;
        match param.names.len() {
            0 => tele.push(Bind::new(licit, unsafe { next_uid() }, ty, None)),
            1 => intros(param.names.remove(0), licit, ty),
            _ => (param.names.into_iter()).for_each(|name| intros(name, licit, ty.clone())),
        }
    }
    Ok((tele, state))
}

pub fn desugar_patterns(state: DesugarState, pats: Vec<ExprPat>) -> DeclM<Vec<AbsPat>> {
    let mut abs_pats = Vec::with_capacity(pats.len());
    let mut state = state;
    for pat in pats {
        let (pat, st) = desugar_pattern(state, pat)?;
        state = st;
        abs_pats.push(pat);
    }
    Ok((abs_pats, state))
}

pub fn desugar_pattern(state: DesugarState, pat: ExprPat) -> DeclM<AbsPat> {
    match pat {
        Pat::Var(name) => {
            let mut st = state;
            let uid = unsafe { next_uid() };
            st.local.insert(name.text, uid);
            Ok((Pat::Var(uid), st))
        }
        // The `head` is pseudo (see `surf::parse`), only `head.name` is real.
        Pat::Cons(is_forced, mut head, params) => {
            let (head_ix, cons) = state
                .lookup_by_name(&head.name.text)
                .ok_or_else(|| DesugarErr::UnresolvedReference(head.name.clone()))?;
            head.cons_ix = head_ix;
            match cons {
                AbsDecl::Cons { .. } => head.ductive = Ductive::In,
                // TODO: coinductive cons?
                _ => return Err(DesugarErr::NotCons(head.name)),
            };
            let (abs_pats, state) = desugar_patterns(state, params)?;
            Ok((Pat::Cons(is_forced, head, abs_pats), state))
        }
        Pat::Forced(term) => {
            let (abs, st) = desugar_expr(state, term)?;
            Ok((Pat::Forced(abs), st))
        }
        Pat::Refl => Ok((Pat::Refl, state)),
        Pat::Absurd => Ok((Pat::Absurd, state)),
    }
}

pub fn desugar_clause(
    mut state: DesugarState,
    defn_ix: GI,
    name: Ident,
    pats: Vec<ExprCopat>,
    body: Expr,
) -> DesugarM {
    let mut abs_pats = Vec::with_capacity(pats.len());
    for copat in pats {
        let pat = match copat {
            Copat::App(app) => {
                let (pat, st) = desugar_pattern(state, app)?;
                state = st;
                Copat::App(pat)
            }
            Copat::Proj(s) => Copat::Proj(s),
        };
        abs_pats.push(pat);
    }
    // Now `state` has been filled with local variable bindings!
    let (body, mut state) = desugar_expr(state, body)?;
    let loc = name.loc + body.loc();
    let info = AbsClause::new(loc, name, abs_pats, defn_ix, body);
    state.decls.push(AbsDecl::Clause(info));
    state.local.clear();
    Ok(state)
}

pub fn desugar_decl(state: DesugarState, decl: ExprDecl) -> DesugarM {
    use ExprDecl::*;
    match decl {
        Defn(name, sig) => {
            let (sig, mut state) = desugar_expr(state, sig)?;
            state.local.clear();
            let abs_decl = AbsDecl::Defn(AbsDefnInfo::new(name.loc + sig.loc(), name, sig));
            state.decls.push(abs_decl);
            Ok(state)
        }
        Cls(name, pats, body) => match state.lookup_by_name(&name.text) {
            Some((ix, AbsDecl::Defn { .. })) => desugar_clause(state, ix, name, pats, body),
            None => {
                let mut state = state;
                let meta = Abs::Meta(name.clone(), state.fresh_meta());
                let decl_len = state.decl_len();
                let mut state = desugar_clause(state, decl_len, name.clone(), pats, body)?;
                state.ensure_local_emptiness();
                let defn = AbsDecl::Defn(AbsDefnInfo::new(name.loc, name, meta));
                state.decls.push(defn);
                Ok(state)
            }
            Some((_, other)) => Err(DesugarErr::NotDefn(other.decl_name().clone())),
        },
        Data(signature, conses) => {
            let (name, tele, mut state) = desugar_telescope(state, signature)?;
            state.decls.reserve(conses.len());
            let loc = match tele.last() {
                None => name.loc,
                Some(loc) => name.loc + loc.ty.loc(),
            };
            let data_ix = state.decls.len();
            let cons_ices = ops_range(data_ix + 1, conses.len());
            let data = AbsDataInfo::new(loc, name, Default::default(), tele, cons_ices);
            let data = AbsDecl::Data(data);
            state.decls.push(data);
            for cons in conses {
                let (binds, new_st) = desugar_params(state, cons.tele)?;
                state = new_st;
                let name = cons.name;
                let loc = match binds.first() {
                    None => name.loc,
                    Some(a) => name.loc + a.ty.loc(),
                };
                let cons = AbsDecl::Cons(AbsConsInfo::new(loc, name, binds, GI(data_ix)));
                state.decls.push(cons);
            }
            Ok(state)
        }
        Codata(signature, fields) => {
            let (name, tele, mut state) = desugar_telescope(state, signature)?;
            state.decls.reserve(fields.len());
            let loc = match tele.last() {
                None => name.loc,
                Some(loc) => name.loc + loc.ty.loc(),
            };
            let codata_ix = state.decls.len();
            let fields_ices = ops_range(codata_ix + 1, fields.len());
            // TODO: self reference?
            let codata = AbsCodataInfo::new(loc, name, None, Default::default(), tele, fields_ices);
            let codata = AbsDecl::Codata(codata);
            state.decls.push(codata);
            for field in fields {
                let (abs, new_st) = desugar_expr(state, field.expr)?;
                state = new_st;
                let name = field.label;
                let loc = name.loc + abs.loc();
                let proj = AbsDecl::Proj(AbsProjInfo::new(loc, name, abs, GI(codata_ix)));
                state.decls.push(proj);
            }
            Ok(state)
        }
    }
}

fn ops_range(start: usize, duration: usize) -> Vec<GI> {
    let cons_ices: Vec<_> = (start..start + duration).collect();
    cons_ices.into_iter().map(GI).collect()
}
