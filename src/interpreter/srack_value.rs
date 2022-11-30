use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub(crate) struct SrackValue(pub String);

impl Display for SrackValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}