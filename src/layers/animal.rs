use crate::nft_trait::Trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animal {
    Cat,
    Fox,
    Rabbit,
}

impl Trait for Animal {
    fn choices() -> Vec<Self> {
        vec![Animal::Cat, Animal::Fox, Animal::Rabbit]
    }

    fn probability(&self) -> usize {
        match self {
            Self::Cat => 4,
            Self::Fox => 4,
            Self::Rabbit => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalColor {
    Gray,
}

impl Trait for AnimalColor {
    fn choices() -> Vec<Self> {
        vec![AnimalColor::Gray]
    }

    fn probability(&self) -> usize {
        1
    }
}
