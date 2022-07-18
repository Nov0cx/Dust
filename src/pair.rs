use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct Pair<A, B>(pub A, pub B);

impl<A, B> fmt::Display for Pair<A, B>
    where A: fmt::Display, B: fmt::Display {

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "A: {}, B: {}", self.0, self.1)
    }
}