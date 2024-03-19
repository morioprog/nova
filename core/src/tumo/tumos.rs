use std::{array::from_fn, ops::Index};

use super::{tumo::Pair, TUMO_LOOP};
use crate::color::{Color, PuyoColor, RealColor};

pub type Tumos = PairQueue<PuyoColor>;
pub type RealTumos = PairQueue<RealColor>;

pub struct PairQueue<C: Color> {
    len: usize,
    head: usize,
    pairs: [Pair<C>; TUMO_LOOP],
}

impl<C: Color> Default for PairQueue<C> {
    fn default() -> Self {
        Self {
            len: 0,
            head: 0,
            pairs: from_fn(|_| Default::default()),
        }
    }
}

impl<C: Color> PairQueue<C> {
    fn len(&self) -> usize {
        self.len
    }

    fn push(&mut self, pair: &Pair<C>) {
        debug_assert_ne!(
            self.len, TUMO_LOOP,
            "len has already reached the maximum ({})",
            TUMO_LOOP
        );

        self.pairs[self.len] = pair.clone();
        self.len += 1;
    }

    fn rotate(&mut self) {
        debug_assert_eq!(
            self.len, TUMO_LOOP,
            "len should be {} but {} when rotating",
            TUMO_LOOP, self.len
        );

        // this works since `TUMO_LOOP` is a power of two
        self.head = (self.head + 1) & (TUMO_LOOP - 1);
    }
}

impl<C: Color> Index<usize> for PairQueue<C> {
    type Output = Pair<C>;

    fn index(&self, index: usize) -> &Self::Output {
        // this works since `TUMO_LOOP` is a power of two
        &self.pairs[(self.head + index) & (TUMO_LOOP - 1)]
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;
    use crate::{
        color::PuyoColor::*,
        tumo::{RealTumo, Tumo},
    };

    #[test]
    fn is_same_type() {
        assert_eq!(
            Tumos::default().pairs[0].type_id(),
            Tumo::default().type_id()
        );
        assert_eq!(
            RealTumos::default().pairs[0].type_id(),
            RealTumo::default().type_id()
        );
    }

    #[test]
    fn basic() {
        let mut tumos = Tumos::default();
        assert_eq!(tumos.len(), 0);

        let tumo = Tumo::new(RED, GREEN);
        tumos.push(&tumo);
        assert_eq!(tumos.len(), 1);
        assert_eq!(tumos[0], tumo);

        let tumo = Tumo::new(BLUE, YELLOW);
        tumos.push(&tumo);
        assert_eq!(tumos.len(), 2);
        assert_eq!(tumos[1], tumo);
    }

    #[test]
    fn rotate() {
        let mut tumos = Tumos::default();
        for i in 0..TUMO_LOOP {
            let color = match i % 4 {
                0 => RED,
                1 => GREEN,
                2 => BLUE,
                3 => YELLOW,
                _ => unreachable!(),
            };
            tumos.push(&Tumo::new_zoro(color));
        }
        assert_eq!(tumos[0].axis(), RED);
        assert_eq!(tumos[1].axis(), GREEN);

        tumos.rotate();
        assert_eq!(tumos[0].axis(), GREEN);
        assert_eq!(tumos[1].axis(), BLUE);

        tumos.rotate();
        assert_eq!(tumos[0].axis(), BLUE);
        assert_eq!(tumos[1].axis(), YELLOW);
    }

    #[test]
    #[should_panic]
    fn push_max_out() {
        let mut tumos = Tumos::default();
        for _ in 0..TUMO_LOOP {
            tumos.push(&Tumo::default());
        }

        // should panic here
        tumos.push(&Tumo::default());
    }

    #[test]
    #[should_panic]
    fn rotate_hasnt_maxed_out() {
        let mut tumos = Tumos::default();

        // should panic here
        tumos.rotate();
    }
}
