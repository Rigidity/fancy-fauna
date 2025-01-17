use rand::{seq::SliceRandom, Rng};

pub trait Trait: Sized + Clone {
    fn choices() -> Vec<Self>;
    fn probability(&self) -> usize;

    fn random(rng: &mut impl Rng) -> Self {
        let mut choices = Self::choices();
        choices.shuffle(rng);

        let mut total_weight = 0isize;
        for choice in choices.iter() {
            total_weight += choice.probability() as isize;
        }

        let mut random_weight: isize = rng.gen_range(0..total_weight);
        for choice in choices.iter() {
            random_weight -= choice.probability() as isize;
            if random_weight < 0 {
                return choice.clone();
            }
        }

        unreachable!()
    }
}
