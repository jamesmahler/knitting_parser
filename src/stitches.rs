//! Holds the definition and details for the supported stitches

/// The supported stitches
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Stitch {
    // Single
    K,
    P,
    K2Tog,
    P2Tog,
    Ssk,
    Ssp,
    SlKwise,
    SlPwise,
    Yo,
    Bo,
    Mr,
    Ml,
    MKwise,
    MPwise,
    Kfb,
    Kbf,
    Pfb,
    Pbf,
    Ktbl,
    Ptbl,
    NoStitch,
    Bobble,
    Bead,

    // Dual
    Lcf1,
    Rcb1,

    // Quad
    Lcf2,
    Rcb2,

    // Six
    Lcf3,
    Rcb3,

    // Eight
    Lcf4,
    Rcb4,
}

impl Stitch {
    /// Returns the width of the stitch.
    ///
    /// This is useful when trying to layout the stitches next to each other.
    pub fn width(&self) -> usize {
        match self {
            Stitch::K => 1,
            Stitch::P => 1,
            Stitch::K2Tog => 1,
            Stitch::P2Tog => 1,
            Stitch::Ssk => 1,
            Stitch::Ssp => 1,
            Stitch::SlKwise => 1,
            Stitch::SlPwise => 1,
            Stitch::Yo => 1,
            Stitch::Bo => 1,
            Stitch::Mr => 1,
            Stitch::Ml => 1,
            Stitch::MKwise => 1,
            Stitch::MPwise => 1,
            Stitch::Kfb => 1,
            Stitch::Kbf => 1,
            Stitch::Pfb => 1,
            Stitch::Pbf => 1,
            Stitch::Ktbl => 1,
            Stitch::Ptbl => 1,
            Stitch::NoStitch => 1,
            Stitch::Bobble => 1,
            Stitch::Bead => 1,

            Stitch::Lcf1 => 2,
            Stitch::Rcb1 => 2,

            Stitch::Lcf2 => 4,
            Stitch::Rcb2 => 4,

            Stitch::Lcf3 => 6,
            Stitch::Rcb3 => 6,

            Stitch::Lcf4 => 8,
            Stitch::Rcb4 => 8,
        }
    }
}
