use voile_util::level::Level;
use voile_util::loc::*;
use voile_util::uid::GI;

use super::*;

impl AbsConsInfo {
    pub fn new(source: Loc, name: Ident, tele: AbsTele, data_index: GI) -> Self {
        Self {
            source,
            name,
            tele,
            data_ix: data_index,
        }
    }
}

impl AbsProjInfo {
    pub fn new(source: Loc, name: Ident, proj_ty: Abs, codata_index: GI) -> Self {
        Self {
            source,
            name,
            ty: proj_ty,
            codata_ix: codata_index,
        }
    }
}

impl AbsDataInfo {
    pub fn new(source: Loc, name: Ident, level: Level, tele: AbsTele, conses: Vec<GI>) -> Self {
        AbsDataInfo {
            source,
            name,
            level,
            tele,
            conses,
        }
    }
}

impl AbsClause {
    pub fn new(
        source: Loc,
        name: Ident,
        patterns: Vec<AbsCopat>,
        definition: GI,
        body: Abs,
    ) -> Self {
        Self {
            source,
            name,
            patterns,
            definition,
            body,
        }
    }
}

impl AbsCodataInfo {
    pub fn new(
        source: Loc,
        name: Ident,
        me: Option<Ident>,
        level: Level,
        tele: AbsTele,
        fields: Vec<GI>,
    ) -> Self {
        Self {
            source,
            name,
            self_ref: me,
            level,
            tele,
            fields,
        }
    }
}

macro_rules! simple_to_loc {
    ($name:ident) => {
        impl ToLoc for $name {
            fn loc(&self) -> Loc {
                self.source
            }
        }
    };
}

simple_to_loc!(AbsClause);
simple_to_loc!(AbsConsInfo);
simple_to_loc!(AbsDataInfo);
simple_to_loc!(AbsProjInfo);
simple_to_loc!(AbsCodataInfo);
