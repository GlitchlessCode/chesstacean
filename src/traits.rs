use rand::Rng;

pub trait ChooseTake {
    type Item;
    fn take_random<R>(&mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized;
}

impl<T> ChooseTake for Vec<T> {
    type Item = T;
    fn take_random<R>(&mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized,
    {
        if self.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.len());
            Some(self.remove(index))
        }
    }
}
