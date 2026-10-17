use serde::{Deserialize, Serialize};

/// A group of calculators in the sidebar.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::IntoStaticStr,
    strum::VariantArray,
)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Category {
    /// Circuits with amplifiers and level conversion.
    Analog = 0,

    /// General-purpose calculators.
    Basic = 1,
}

impl Category {
    /// Every category, in the order of the sidebar.
    pub const ALL: &'static [Category] = <Category as strum::VariantArray>::VARIANTS;

    /// The position of the category in [`Category::ALL`].
    #[inline]
    pub const fn idx(self) -> usize {
        self as usize
    }
}
