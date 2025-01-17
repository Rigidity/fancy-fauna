use image::Rgba;

use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForegroundColor {
    Water,
    Lava,
    Sand,
    Wood,
}

impl ForegroundColor {
    pub fn rgba(&self) -> Rgba<u8> {
        let [r, g, b] = match self {
            Self::Water => [100, 100, 240],
            Self::Lava => [255, 110, 20],
            Self::Sand => [255, 200, 128],
            Self::Wood => [161, 102, 47],
        };
        Rgba([r, g, b, 255])
    }
}

impl Trait for ForegroundColor {
    fn choices() -> Vec<Self> {
        vec![
            ForegroundColor::Water,
            ForegroundColor::Lava,
            ForegroundColor::Sand,
            ForegroundColor::Wood,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Water => 5,
            Self::Lava => 1,
            Self::Sand => 4,
            Self::Wood => 3,
        }
    }
}
