use super::*;

macro_rules! simple_to_loc {
    ($name:ident) => {
        impl ToLoc for $name {
            fn loc(&self) -> Loc {
                self.loc
            }
        }
    };
}

simple_to_loc!(DataInfo);
simple_to_loc!(CodataInfo);
simple_to_loc!(ConsInfo);
simple_to_loc!(TermInfo);
simple_to_loc!(FuncInfo);
