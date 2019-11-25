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
