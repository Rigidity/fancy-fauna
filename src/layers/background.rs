use image::Rgba;

use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Background {
    Plain,
    Vertical,
    Horizontal,
    Radial,
    Squares,
}

impl Trait for Background {
    fn choices() -> Vec<Self> {
        vec![
            Background::Plain,
            Background::Vertical,
            Background::Horizontal,
            Background::Radial,
            Background::Squares,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Plain => 3,
            Self::Vertical => 2,
            Self::Horizontal => 2,
            Self::Radial => 1,
            Self::Squares => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackgroundColor {
    Sky,
    Sunlight,
    Sunset,
    Night,
    Cloudy,
    Storm,
    Overcast,
}

impl BackgroundColor {
    pub fn rgba(&self) -> (Rgba<u8>, Rgba<u8>) {
        let ([pr, pg, pb], [sr, sg, sb]) = match self {
            Self::Sky => ([128, 200, 255], [90, 180, 255]),
            Self::Sunlight => ([255, 255, 150], [235, 235, 100]),
            Self::Sunset => ([255, 150, 100], [235, 135, 100]),
            Self::Night => ([90, 90, 90], [70, 70, 70]),
            Self::Cloudy => ([255, 255, 255], [220, 220, 220]),
            Self::Storm => ([128, 255, 255], [128, 200, 200]),
            Self::Overcast => ([150, 150, 150], [120, 120, 120]),
        };
        (Rgba([pr, pg, pb, 255]), Rgba([sr, sg, sb, 255]))
    }
}

impl Trait for BackgroundColor {
    fn choices() -> Vec<Self> {
        vec![
            BackgroundColor::Sky,
            BackgroundColor::Sunlight,
            BackgroundColor::Sunset,
            BackgroundColor::Night,
            BackgroundColor::Cloudy,
            BackgroundColor::Storm,
            BackgroundColor::Overcast,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Sky => 8,
            Self::Sunlight => 4,
            Self::Sunset => 2,
            Self::Night => 1,
            Self::Cloudy => 1,
            Self::Storm => 6,
            Self::Overcast => 1,
        }
    }
}
