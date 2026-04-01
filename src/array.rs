use crate::BrailleCharTrait;


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

    pub const fn width(&self) -> usize {
        return COLUMNS * 2;
    }

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

        return self.array[r][c].get(c_ as u8, r_ as u8);
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].get_unchecked(c_ as u8, r_ as u8);
    }

    pub const fn get_char(&self, x: usize, y: usize) -> &T {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        return &self.array[y][x];
    }

    pub const fn get_char_unchecked(&self, x: usize, y: usize) -> &T {
        return &self.array[y][x];
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set(c_ as u8, r_ as u8, value);
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[r][c].set_unchecked(c_ as u8, r_ as u8, value);
    }

    pub const fn set_char(&mut self, x: usize, y: usize, value: T) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        self.array[y][x] = value;
    }

    pub const fn set_char_unchecked(&mut self, x: usize, y: usize, value: T) {
        self.array[y][x] = value;
    }

    pub const fn get_char_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        return &mut self.array[y][x];
    }

    pub const fn get_char_mut_unchecked(&mut self, x: usize, y: usize) -> &mut T {
        return &mut self.array[y][x];
    }
}

