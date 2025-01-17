pub trait Trait: Sized {
    fn choices() -> Vec<Self>;
    fn probability(&self) -> usize;
}
