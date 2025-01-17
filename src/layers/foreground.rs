use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Foreground {
    Ramp,
    Wall,
    Wave,
}

impl Trait for Foreground {
    fn choices() -> Vec<Self> {
        vec![Foreground::Ramp, Foreground::Wall, Foreground::Wave]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Ramp => 1,
            Self::Wall => 1,
            Self::Wave => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundColor {
    Gray,
}

impl Trait for ForegroundColor {
    fn choices() -> Vec<Self> {
        vec![ForegroundColor::Gray]
    }

    fn probability(&self) -> usize {
        1
    }
}
