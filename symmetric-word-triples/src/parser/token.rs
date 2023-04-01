use dashmap::DashMap;

use super::wordfilter::Hr;

pub struct Tkns {
    tkn_to_string: DashMap<Tkn, String, Hr>,

}

pub struct Tkn(u16);

impl Tkn {
    fn to_string(&self) -> String {
        todo!()
    }
}
