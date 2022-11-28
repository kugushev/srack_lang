use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub enum SrackItem {
    Str(String),
    // todo: Num(i64)
}

impl Display for SrackItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self
        {
            SrackItem::Str(val) => { write!(f, "{}", val) }
        }
    }
}