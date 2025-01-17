use image::Rgba;

use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Animal {
    Cat,
    Fox,
    Rabbit,
    Budgie,
}

impl Trait for Animal {
    fn choices() -> Vec<Self> {
        vec![Animal::Cat, Animal::Fox, Animal::Rabbit, Animal::Budgie]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Cat => 4,
            Self::Fox => 4,
            Self::Rabbit => 3,
            Self::Budgie => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimalColor {
    Red,
    Green,
    Blue,
    Purple,
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
            AnimalColor::Red,
            AnimalColor::Green,
            AnimalColor::Blue,
            AnimalColor::Purple,
            AnimalColor::Gray,
            AnimalColor::Orange,
            AnimalColor::Alpha,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Red => 10,
            Self::Green => 10,
            Self::Blue => 10,
            Self::Purple => 3,
            Self::Gray => 6,
            Self::Orange => 3,
            Self::Alpha => 1,
        }
    }
}
