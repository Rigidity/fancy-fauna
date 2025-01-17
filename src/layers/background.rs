use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Background {
    Plain,
    Vertical,
    Horizontal,
    Radial,
}

impl Trait for Background {
    fn choices() -> Vec<Self> {
        vec![
            Background::Plain,
            Background::Vertical,
            Background::Horizontal,
            Background::Radial,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Plain => 3,
            Self::Vertical => 2,
            Self::Horizontal => 2,
            Self::Radial => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundColor {
    Gray,
}

impl Trait for BackgroundColor {
    fn choices() -> Vec<Self> {
        vec![BackgroundColor::Gray]
    }

    fn probability(&self) -> usize {
        1
    }
}
