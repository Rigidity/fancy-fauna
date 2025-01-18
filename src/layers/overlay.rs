use crate::nft_trait::Trait;

use super::Animal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Overlay {
    None,
    Halo,
    Sunglasses,
    Lasers,
    Heart,
    Sprout,
    Rust,
    Xch,
}

impl Trait for Overlay {
    fn choices() -> Vec<Self> {
        vec![
            Self::None,
            Self::Halo,
            Self::Sunglasses,
            Self::Lasers,
            Self::Heart,
            Self::Sprout,
            Self::Rust,
            Self::Xch,
        ]
    }

    fn probability(&self) -> usize {
        match self {
            Self::None => 10,
            Self::Halo => 6,
            Self::Sunglasses => 6,
            Self::Lasers => 2,
            Self::Heart => 4,
            Self::Sprout => 4,
            Self::Rust => 1,
            Self::Xch => 1,
        }
    }
}

impl Overlay {
    pub fn position(&self, animal: Animal) -> (u32, u32) {
        match (self, animal) {
            (Self::None, _) => (0, 0),
            (Self::Halo, Animal::Cat) => (11, 4),
            (Self::Halo, Animal::Dog) => (8, 3),
            (Self::Halo, Animal::Fox) => (11, 4),
            (Self::Halo, Animal::Rabbit) => (10, 3),
            (Self::Halo, Animal::Budgie) => (12, 3),
            (Self::Sunglasses | Self::Lasers, Animal::Cat) => (12, 8),
            (Self::Sunglasses | Self::Lasers, Animal::Dog) => (10, 9),
            (Self::Sunglasses | Self::Lasers, Animal::Fox) => (12, 11),
            (Self::Sunglasses | Self::Lasers, Animal::Rabbit) => (11, 11),
            (Self::Sunglasses | Self::Lasers, Animal::Budgie) => (16, 5),
            (Self::Heart, _) => (5, 5),
            (Self::Sprout, _) => (20, 19),
            (Self::Rust, _) => (14, 18),
            (Self::Xch, Animal::Fox) => (8, 20),
            (Self::Xch, Animal::Rabbit) => (8, 20),
            (Self::Xch, Animal::Budgie) => (11, 18),
            (Self::Xch, Animal::Dog) => (12, 8),
            (Self::Xch, Animal::Cat) => (12, 19),
        }
    }
}
