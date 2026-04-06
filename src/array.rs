use crate::BrailleCharTrait;

use std::ops::{Index, IndexMut};


#[derive(Clone, Debug)]
pub struct BrailleCharGridArray<T: BrailleCharTrait, const COLUMNS: usize, const ROWS: usize> {
    pub array: [[T; COLUMNS]; ROWS]
}

impl<T: BrailleCharTrait, const COLUMNS: usize, const ROWS: usize> BrailleCharGridArray<T, COLUMNS, ROWS> {
    pub fn new() -> Self {
        return Self {
            array: [[T::EMPTY; COLUMNS]; ROWS]
        };
    }

    #[inline(always)]
    pub const fn width(&self) -> usize {
        return COLUMNS * 2;
    }

    #[inline(always)]
    pub const fn height(&self) -> usize {
        return ROWS * 4;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get_at_xy(c_ as u8, r_ as u8);
    }

    pub unsafe fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return unsafe { self.array.get_unchecked(r).get_unchecked(c).get_at_xy_unchecked(c_ as u8, r_ as u8) };
    }

    pub fn get_char(&self, x: usize, y: usize) -> Option<&T> {
        return self.array.get(y).map(|row| row.get(x)).flatten();
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set_at_xy(c_ as u8, r_ as u8, value);
    }

    pub unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return unsafe { self.array.get_unchecked_mut(r).get_unchecked_mut(c).set_at_xy_unchecked(c_ as u8, r_ as u8, value) };
    }

    pub const fn set_char(&mut self, x: usize, y: usize, value: T) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        self.array[y][x] = value;
    }

    pub const unsafe fn set_char_unchecked(&mut self, x: usize, y: usize, value: T) {
        self.array[y][x] = value;
    }

    pub fn get_char_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        return self.array.get_mut(y).map(|row| row.get_mut(x)).flatten();
    }

    pub const unsafe fn get_char_mut_unchecked(&mut self, x: usize, y: usize) -> &mut T {
        return &mut self.array[y][x];
    }

    #[inline(always)]
    pub const fn fill(&mut self, value: T) {
        self.array = [[value; COLUMNS]; ROWS];
    }

    #[inline(always)]
    pub fn fill_with<F: FnMut() -> T>(&mut self, mut f: F) {
        self.array = std::array::from_fn(|_| std::array::from_fn(|_| f()));
    }
}

impl<T: BrailleCharTrait, const COLUMNS: usize, const ROWS: usize> Index<(usize, usize)> for BrailleCharGridArray<T, COLUMNS, ROWS> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;

        return &self.array[y][x];
    }
}

impl<T: BrailleCharTrait, const COLUMNS: usize, const ROWS: usize> IndexMut<(usize, usize)> for BrailleCharGridArray<T, COLUMNS, ROWS> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;

        return &mut self.array[y][x];
    }
}

