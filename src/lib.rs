use std::ops::Range;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BrailleChar(pub u8);

impl BrailleChar {
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 4;

    pub const fn ordered(&self) -> u8 {
        return self.0;
    }

    pub const fn unordered(&self) -> u8 {
        let Self(b) = self;
        let b = Self::ordered_to_unordered(*b);

        return b;
    }

    pub const fn unordered_to_ordered(b: u8) -> u8 {
        let b = b & 0b_0001_0000
            |  (b & 0b_1000_0000) >> 7
            |  (b & 0b_0100_0000) >> 3
            |  (b & 0b_0010_0000) >> 4
            |  (b & 0b_0000_1000) >> 1
            |  (b & 0b_0000_0100) << 3
            |  (b & 0b_0000_0010) << 5
            |  (b & 0b_0000_0001) << 7;

        return b;
    }

    pub const fn ordered_to_unordered(b: u8) -> u8 {
        let b = b & 0b_0001_0000
            |  (b & 0b_1000_0000) >> 7
            |  (b & 0b_0100_0000) >> 5
            |  (b & 0b_0010_0000) >> 3
            |  (b & 0b_0000_1000) << 3
            |  (b & 0b_0000_0100) << 1
            |  (b & 0b_0000_0010) << 4
            |  (b & 0b_0000_0001) << 7;

        return b;
    }

    pub const fn from_unordered(b: u8) -> Self {
        let b = Self::unordered_to_ordered(b);

        return Self(b);
    }

    #[inline(always)]
    pub const fn u32_char(&self) -> u32 {
        return Self::CHAR_RANGE.start + self.ordered() as u32;
    }

    #[inline(always)]
    pub const fn char(&self) -> char {
        return unsafe { char::from_u32_unchecked(self.u32_char()) };
    }

    pub const fn from_u32_char(char: u32) -> Option<Self> {
        const MIN: u32 = BrailleChar::CHAR_RANGE.start;
        const MAX: u32 = BrailleChar::CHAR_RANGE.end;

        return match char {
            MIN..MAX => Some(Self((char - Self::CHAR_RANGE.start) as u8)),
            _ => None
        };
    }

    #[inline(always)]
    pub const fn from_u32_char_unchecked(char: u32) -> Self {
        return Self((char - Self::CHAR_RANGE.start) as u8);
    }

    #[inline(always)]
    pub const fn from_char(char: char) -> Option<Self> {
        return Self::from_u32_char(char as u32);
    }

    #[inline(always)]
    pub const fn from_char_unchecked(char: char) -> Self {
        return Self::from_u32_char_unchecked(char as u32);
    }

    pub const fn get(&self, x: u8, y: u8) -> bool {
        assert!(x < 2);
        assert!(y < 4);

        return self.get_unchecked(x, y);
    }

    #[inline(always)]
    pub const fn get_unchecked(&self, x: u8, y: u8) -> bool {
        return (self.unordered() & (0b_1000_0000 >> (x + y * 2))) != 0;
    }


    pub const fn set(&mut self, x: u8, y: u8, value: bool) {
        assert!(x < 2);
        assert!(y < 4);

        self.set_unchecked(x, y, value);
    }

    #[inline(always)]
    pub const fn set_unchecked(&mut self, x: u8, y: u8, value: bool) {
        let o = 7 - x - y * 2;

        *self = Self::from_unordered(self.unordered() & !(1 << o) | (value as u8) << o);
    }
}

#[derive(Clone, Debug)]
pub struct BrailleCharGridArray<const COLUMNS: usize, const ROWS: usize> {
    pub array: [[BrailleChar; COLUMNS]; ROWS]
}

impl<const COLUMNS: usize, const ROWS: usize> BrailleCharGridArray<COLUMNS, ROWS> {
    pub const fn new() -> Self {
        return Self {
            array: [[BrailleChar(0u8); COLUMNS]; ROWS]
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

