#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Chain(u32, u32, u32);

impl Chain {
    pub const fn new(chain: u32, score: u32, frame: u32) -> Self {
        Self(chain, score, frame)
    }

    pub const fn chain(&self) -> u32 {
        self.0
    }

    pub const fn score(&self) -> u32 {
        self.1
    }

    pub const fn frame(&self) -> u32 {
        self.2
    }
}

impl std::ops::Add for Chain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl PartialOrd for Chain {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/*
   1. asc score
   2. asc chain
   3. dsc frame
*/
impl Ord for Chain {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1
            .cmp(&other.1)
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| other.2.cmp(&self.2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ord() {
        let mut v = vec![
            Chain::new(3, 4, 5),
            Chain::new(3, 4, 6),
            Chain::new(2, 6, 3),
            Chain::new(3, 6, 2),
            Chain::new(6, 5, 8),
        ];
        let expected = vec![
            Chain::new(3, 4, 6),
            Chain::new(3, 4, 5),
            Chain::new(6, 5, 8),
            Chain::new(2, 6, 3),
            Chain::new(3, 6, 2),
        ];

        v.sort();

        assert_eq!(v, expected);
    }
}
