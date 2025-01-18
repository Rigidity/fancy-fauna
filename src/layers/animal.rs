use image::Rgba;

use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Animal {
    Cat,
    Dog,
    Fox,
    Rabbit,
    Budgie,
    Duck,
}

impl Trait for Animal {
    fn choices() -> Vec<Self> {
        vec![
            Self::Cat,
            Self::Dog,
            Self::Fox,
            Self::Rabbit,
            Self::Budgie,
            Self::Duck,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Cat => 4,
            Self::Dog => 4,
            Self::Fox => 4,
            Self::Rabbit => 3,
            Self::Budgie => 2,
            Self::Duck => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimalColor {
    Red,
    Green,
    Blue,
    Purple,
    Yellow,
    Gray,
    Orange,
    Alpha,
}

impl AnimalColor {
    pub fn rgba(&self) -> Rgba<u8> {
        let [r, g, b] = match self {
            Self::Red => [255, 120, 120],
            Self::Green => [140, 220, 140],
            Self::Blue => [145, 145, 255],
            Self::Purple => [180, 80, 230],
            Self::Yellow => [240, 240, 30],
            Self::Gray => [170, 170, 170],
            Self::Orange => [255, 165, 0],
            Self::Alpha => return Rgba([0, 0, 0, 0]),
        };
        Rgba([r, g, b, 255])
    }
}

impl Trait for AnimalColor {
    fn choices() -> Vec<Self> {
        vec![
            Self::Red,
            Self::Green,
            Self::Blue,
            Self::Purple,
            Self::Yellow,
            Self::Gray,
            Self::Orange,
            Self::Alpha,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Red => 10,
            Self::Green => 10,
            Self::Blue => 10,
            Self::Purple => 3,
            Self::Yellow => 5,
            Self::Gray => 4,
            Self::Orange => 3,
            Self::Alpha => 1,
        }
    }
}
