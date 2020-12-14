pub struct PrimeFactors {
    x: usize,
    factor: usize,
}

impl PrimeFactors {
    pub fn of(x: usize) -> Self {
        PrimeFactors { x, factor: 2 }
    }
}

impl Iterator for PrimeFactors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.x % self.factor != 0 && self.x >= self.factor {
            if self.factor == 2 {
                self.factor += 1;
            } else {
                self.factor += 2;
            }
        }

        if self.x < self.factor {
            return None;
        }

        self.x /= self.factor;

        let f = self.factor;

        Some(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, &[])]
    #[test_case(2, &[2])]
    #[test_case(3, &[3])]
    #[test_case(4, &[2, 2])]
    #[test_case(5, &[5])]
    #[test_case(6, &[2, 3])]
    #[test_case(7, &[7])]
    #[test_case(8, &[2, 2, 2])]
    #[test_case(9, &[3, 3])]
    #[test_case(10, &[2, 5])]
    #[test_case(12, &[2, 2, 3])]
    #[test_case(13, &[13])]
    #[test_case(17, &[17])]
    #[test_case(19, &[19])]
    #[test_case(36, &[2, 2, 3, 3])]
    fn prime_factors(x: usize, expected: &[usize]) {
        let factors: Vec<_> = PrimeFactors::of(x).collect();

        assert_eq!(factors, expected)
    }
}
