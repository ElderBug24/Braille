use crate::{BrailleChar, BrailleCharUnOrdered};



#[derive(Clone, Debug)]
pub struct BrailleCharGridArray<const COLUMNS: usize, const ROWS: usize> {
    pub array: [[BrailleChar; COLUMNS]; ROWS]
}

impl<const COLUMNS: usize, const ROWS: usize> BrailleCharGridArray<COLUMNS, ROWS> {
    pub const fn new() -> Self {
        return Self {
            array: [[BrailleChar::from_ordered(0u8); COLUMNS]; ROWS]
        };
    }

    pub const fn width(&self) -> usize {
        return COLUMNS * 2;
    }

    pub const fn height(&self) -> usize {
        return ROWS * 4;
    }

    pub const fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get(c_ as u8, r_ as u8);
    }

    pub const fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get_unchecked(c_ as u8, r_ as u8);
    }

    pub const fn get_char(&self, x: usize, y: usize) -> BrailleChar {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        return self.array[y][x];
    }

    pub const fn get_char_unchecked(&self, x: usize, y: usize) -> BrailleChar {
        return self.array[y][x];
    }

    pub const fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set(c_ as u8, r_ as u8, value);
    }

    pub const fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set_unchecked(c_ as u8, r_ as u8, value);
    }

    pub const fn set_char(&mut self, x: usize, y: usize, value: BrailleChar) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        self.array[y][x] = value;
    }

    pub const fn set_char_unchecked(&mut self, x: usize, y: usize, value: BrailleChar) {
        self.array[y][x] = value;
    }
}

#[derive(Clone, Debug)]
pub struct BrailleCharGridArrayUnOrdered<const COLUMNS: usize, const ROWS: usize> {
    pub array: [[BrailleCharUnOrdered; COLUMNS]; ROWS]
}

impl<const COLUMNS: usize, const ROWS: usize> BrailleCharGridArrayUnOrdered<COLUMNS, ROWS> {
    pub const fn new() -> Self {
        return Self {
            array: [[BrailleCharUnOrdered::from_ordered(0u8); COLUMNS]; ROWS]
        };
    }

    pub const fn width(&self) -> usize {
        return COLUMNS * 2;
    }

    pub const fn height(&self) -> usize {
        return ROWS * 4;
    }

    pub const fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get(c_ as u8, r_ as u8);
    }

    pub const fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get_unchecked(c_ as u8, r_ as u8);
    }

    pub const fn get_char(&self, x: usize, y: usize) -> BrailleCharUnOrdered {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        return self.array[y][x];
    }

    pub const fn get_char_unchecked(&self, x: usize, y: usize) -> BrailleCharUnOrdered {
        return self.array[y][x];
    }

    pub const fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set(c_ as u8, r_ as u8, value);
    }

    pub const fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set_unchecked(c_ as u8, r_ as u8, value);
    }

    pub const fn set_char(&mut self, x: usize, y: usize, value: BrailleCharUnOrdered) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        self.array[y][x] = value;
    }

    pub const fn set_char_unchecked(&mut self, x: usize, y: usize, value: BrailleCharUnOrdered) {
        self.array[y][x] = value;
    }
}

