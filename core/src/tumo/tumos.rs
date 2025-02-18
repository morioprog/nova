use std::{array::from_fn, ops::Index};

use super::{tumo::Pair, TUMO_LOOP};
use crate::color::{Color, PuyoColor, RealColor};

pub type Tumos = PairQueue<PuyoColor>;
pub type RealTumos = PairQueue<RealColor>;

#[derive(Clone)]
pub struct PairQueue<C: Color> {
    len: usize,
    head: usize,
    pairs: [Pair<C>; TUMO_LOOP],
    looped: bool,
}

impl<C: Color> Default for PairQueue<C> {
    fn default() -> Self {
        Self {
            len: 0,
            head: 0,
            pairs: from_fn(|_| Default::default()),
            looped: false,
        }
    }
}

impl<C: Color> PairQueue<C> {
    pub fn new(pairs: &[Pair<C>]) -> Self {
        debug_assert!(pairs.len() <= TUMO_LOOP);

        let mut tumos = Self::default();
        for pair in pairs.iter() {
            tumos.push(pair);
        }
        tumos
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn available_tumo_len(&self) -> usize {
        self.len - self.head
    }

    pub fn push(&mut self, pair: &Pair<C>) {
        debug_assert_ne!(
            self.len, TUMO_LOOP,
            "len has already reached the maximum ({})",
            TUMO_LOOP
        );

        self.pairs[self.len] = *pair;
        self.len += 1;
    }

    pub fn rotate(&mut self, visible: usize) {
        if self.head + visible >= TUMO_LOOP {
            self.looped = true
        }

        // this works since `TUMO_LOOP` is a power of two
        self.head = (self.head + 1) & (TUMO_LOOP - 1);
    }

    pub fn get_raw(&self, index: usize) -> Option<Pair<C>> {
        if index < self.len {
            Some(self.pairs[index])
        } else {
            None
        }
    }

    pub fn set_raw(&mut self, index: usize, x: Pair<C>) {
        self.pairs[index] = x
    }

    pub fn set_len(&mut self, len: usize) {
        self.len = len
    }

    pub fn slice_visible_tumos(&self, visible: usize, head_idx: Option<usize>) -> Self {
        if self.looped {
            return if head_idx.is_none() {
                self.clone()
            } else {
                let mut tumos = self.clone();
                tumos.head = head_idx.unwrap();
                tumos
            };
        }

        let head_idx = head_idx.unwrap_or(self.head);
        let mut tumos = Self::default();
        for i in head_idx..(head_idx + visible) {
            if let Some(tumo) = self.get_raw(i) {
                tumos.push(&tumo);
            }
        }
        tumos
    }

    pub fn slice_visible_tumos_pvp(
        visible: usize,
        tumos_1p: &Self,
        tumos_2p: &Self,
    ) -> (Self, Self) {
        if tumos_1p.looped {
            let mut new_tumos_2p = tumos_1p.clone();
            new_tumos_2p.head = tumos_2p.head;
            return (tumos_1p.clone(), new_tumos_2p);
        }
        if tumos_2p.looped {
            let mut new_tumos_1p = tumos_2p.clone();
            new_tumos_1p.head = tumos_1p.head;
            return (new_tumos_1p, tumos_2p.clone());
        }

        let longer = if tumos_1p.head > tumos_2p.head {
            tumos_1p
        } else {
            tumos_2p
        };
        let last_visible_idx = longer.head + visible;
        (
            longer.slice_visible_tumos(last_visible_idx - tumos_1p.head, Some(tumos_1p.head)),
            longer.slice_visible_tumos(last_visible_idx - tumos_2p.head, Some(tumos_2p.head)),
        )
    }
}

impl PairQueue<PuyoColor> {
    pub fn new_random() -> Self {
        let mut tumos = Self::default();
        for _ in 0..TUMO_LOOP {
            tumos.push(&Pair::<PuyoColor>::new_random());
        }
        tumos
    }

    pub fn extend_randoms(&mut self, len: usize) {
        for _ in 0..len {
            self.push(&Pair::<PuyoColor>::new_random());
        }
    }
}

impl<C: Color> Index<usize> for PairQueue<C> {
    type Output = Pair<C>;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.len);

        // this works since `TUMO_LOOP` is a power of two
        &self.pairs[(self.head + index) & (TUMO_LOOP - 1)]
    }
}

impl<C: Color + From<char>> From<&str> for PairQueue<C> {
    fn from(value: &str) -> Self {
        debug_assert!(value.len() % 2 == 0 && value.len() >= 2 && value.len() <= 2 * TUMO_LOOP);

        Self::new(
            &value
                .chars()
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|s| Pair::<C>::from((s[0], s[1])))
                .collect::<Vec<_>>(),
        )
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
    fn new() {
        let pairs = vec![Tumo::new_zoro(RED); 10];

        let tumos = Tumos::new(&pairs);

        assert_eq!(tumos.len(), pairs.len());
        assert_eq!(tumos[0], pairs[0]);
    }

    #[test]
    fn from_str() {
        let tumos = Tumos::from("rbyygr");
        assert_eq!(tumos.len(), 3);
        assert_eq!(tumos[0], Tumo::new(RED, BLUE));
        assert_eq!(tumos[1], Tumo::new(YELLOW, YELLOW));
        assert_eq!(tumos[2], Tumo::new(GREEN, RED));
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

        tumos.rotate(0);
        assert_eq!(tumos[0].axis(), GREEN);
        assert_eq!(tumos[1].axis(), BLUE);

        tumos.rotate(0);
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
    fn new_random() {
        for _ in 0..100 {
            let tumos = Tumos::new_random();
            for i in 0..tumos.len() {
                assert!(tumos[i].is_valid());
            }
        }
    }

    #[test]
    fn slice_visible_tumos() {
        let mut tumos = Tumos::new_random();
        let testcases = [(10, 7, 3, 3), (9, 7, 3, 2)];
        for (len, head, visible, expected_len) in testcases {
            tumos.len = len;
            tumos.head = head;

            let sliced = tumos.slice_visible_tumos(visible, None);

            assert_eq!(sliced.len(), expected_len);
            for i in 0..expected_len {
                assert_eq!(tumos[i], sliced[i]);
            }
        }
    }

    #[test]
    fn slice_visible_tumos_pvp() {
        let tumos = Tumos::new_random();
        let mut tumos_1p = tumos.clone();
        let mut tumos_2p = tumos.clone();
        let testcases = [((10, 6), (9, 5), 3, (3, 4)), ((9, 5), (10, 6), 3, (4, 3))];
        for ((len_1p, head_1p), (len_2p, head_2p), visible, (expected_len_1p, expected_len_2p)) in
            testcases
        {
            tumos_1p.len = len_1p;
            tumos_1p.head = head_1p;
            tumos_2p.len = len_2p;
            tumos_2p.head = head_2p;

            let (sliced_1p, sliced_2p) =
                Tumos::slice_visible_tumos_pvp(visible, &tumos_1p, &tumos_2p);

            assert_eq!(sliced_1p.len(), expected_len_1p);
            assert_eq!(sliced_2p.len(), expected_len_2p);
            for i in 0..expected_len_1p {
                assert_eq!(tumos.get_raw(head_1p + i).unwrap(), sliced_1p[i]);
            }
            for i in 0..expected_len_2p {
                assert_eq!(tumos.get_raw(head_2p + i).unwrap(), sliced_2p[i]);
            }
        }
    }
}
