pub struct Pairs<'a, X> {
    items: &'a [X],
    i: usize,
}

impl<'a, X> Pairs<'a, X> {
    pub fn of(items: &'a [X]) -> Self {
        Pairs { items, i: 0 }
    }
}

impl<'a, X> Iterator for Pairs<'a, X>
where
    X: Copy,
{
    type Item = (X, X);

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;

        if self.i >= self.items.len() {
            self.items = self.items.get(1..)?;
            self.i = 1;
        }

        let a = *self.items.get(0)?;

        let b = *self.items.get(self.i)?;

        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs() {
        let pairs: Vec<(u32, u32)> = Pairs::of(&[1, 2, 3, 4]).collect();

        assert_eq!(pairs, vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]);
    }
}
