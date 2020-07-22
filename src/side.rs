/// Represents the side of the line.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Side {
    RS,
    WS,
}

impl Side {
    /// Switch to the other side
    ///
    /// #Arguments
    /// `in_round` - If the pattern is in the round
    pub fn switch(self, in_round: bool) -> Side {
        if in_round {
            self
        } else {
            match self {
                Side::RS => Side::WS,
                Side::WS => Side::RS,
            }
        }
    }
}
