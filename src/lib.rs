use std::ops::Range;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BrailleChar {
    UnOrdered(u8),
    Ordered(u8)
}

impl BrailleChar {
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);

    pub fn unordered(&self) -> u8 {
        return match self {
           &Self::UnOrdered(b) => b,
            Self::Ordered(b) => {
                b & 0b_0001_0000
                    | (b & 0b_1000_0000) >> 7
                    | (b & 0b_0100_0000) >> 5
                    | (b & 0b_0010_0000) >> 3
                    | (b & 0b_0000_1000) << 3
                    | (b & 0b_0000_0100) << 1
                    | (b & 0b_0000_0010) << 4
                    | (b & 0b_0000_0001) << 7
            }
        };
    }

    pub fn ordered(&self) -> u8 {
        return match self {
            Self::UnOrdered(b) => {
                b & 0b_0001_0000
                    | (b & 0b_1000_0000) >> 7
                    | (b & 0b_0100_0000) >> 3
                    | (b & 0b_0010_0000) >> 4
                    | (b & 0b_0000_1000) >> 1
                    | (b & 0b_0000_0100) << 3
                    | (b & 0b_0000_0010) << 5
                    | (b & 0b_0000_0001) << 7
            },
           &Self::Ordered(b) => b
        };
    }

    #[inline(always)]
    pub fn into_unordered(&self) -> Self {
        return Self::UnOrdered(self.unordered());
    }

    #[inline(always)]
    pub fn into_ordered(&self) -> Self {
        return Self::Ordered(self.ordered());
    }

    #[inline(always)]
    pub fn into_ordered_assign(&mut self) {
        *self = self.into_ordered();
    }

    #[inline(always)]
    pub fn into_unordered_assign(&mut self) {
        *self = self.into_unordered();
    }

    #[inline(always)]
    pub fn u32_char(&self) -> u32 {
        return Self::CHAR_RANGE.start + self.ordered() as u32;
    }

    #[inline(always)]
    pub fn char(&self) -> char {
        return unsafe { char::from_u32_unchecked(self.u32_char()) };
    }

    pub fn from_u32_char(char: u32) -> Option<Self> {
        const MIN: u32 = BrailleChar::CHAR_RANGE.start;
        const MAX: u32 = BrailleChar::CHAR_RANGE.end;

        return match char {
            MIN..MAX => Some(Self::Ordered((char - Self::CHAR_RANGE.start) as u8)),
            _ => None
        };
    }

    #[inline(always)]
    pub fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::Ordered((char - Self::CHAR_RANGE.start) as u8);
    }

    #[inline(always)]
    pub fn from_char(char: char) -> Option<Self> {
        return Self::from_u32_char(char as u32);
    }

    #[inline(always)]
    pub fn from_char_unchecked(char: char) -> Self {
        return Self::from_u32_char_unchecked(char as u32);
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        assert!(x < 2);
        assert!(y < 4);

        return self.get_unchecked(x, y);
    }

    #[inline(always)]
    pub fn get_unchecked(&self, x: u8, y: u8) -> bool {
        return (self.unordered() & (0b_1000_0000 >> (x + y * 2))) != 0;
    }


    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        assert!(x < 2);
        assert!(y < 4);

        self.set_unchecked(x, y, value);
    }

    #[inline(always)]
    pub fn set_unchecked(&mut self, x: u8, y: u8, value: bool) {
        let o = 7 - x - y * 2;
        *self = Self::UnOrdered(self.unordered() & !(1 << o) | (value as u8) << o);
    }
}

pub struct BrailleCharGridArray<const COLUMNS: usize, const ROWS: usize> {
    pub array: [[BrailleChar; COLUMNS]; ROWS]
}

impl<const COLUMNS: usize, const ROWS: usize> BrailleCharGridArray<COLUMNS, ROWS> {
    pub fn new() -> Self {
        return Self {
            array: [[BrailleChar::Ordered(0u8); COLUMNS]; ROWS]
        };
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(COLUMNS);
        let c_ = x - c * COLUMNS;

        let r = y.div_euclid(ROWS);
        let r_ = y - r * ROWS;

        return self.array[r][c].get(c_ as u8, r_ as u8);
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(COLUMNS);
        let c_ = x - c * COLUMNS;

        let r = y.div_euclid(ROWS);
        let r_ = y - r * ROWS;

        return self.array[r][c].get_unchecked(c_ as u8, r_ as u8);
    }

    pub fn get_char(&self, x: usize, y: usize) -> BrailleChar {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        return self.array[y][x];
    }

    pub fn get_char_unchecked(&self, x: usize, y: usize) -> BrailleChar {
        return self.array[y][x];
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        let c = x.div_euclid(COLUMNS);
        let c_ = x - c * COLUMNS;

        let r = y.div_euclid(ROWS);
        let r_ = y - r * ROWS;

        return self.array[r][c].set(c_ as u8, r_ as u8, value);
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(COLUMNS);
        let c_ = x - c * COLUMNS;

        let r = y.div_euclid(ROWS);
        let r_ = y - r * ROWS;

        return self.array[r][c].set_unchecked(c_ as u8, r_ as u8, value);
    }

    pub fn set_char(&mut self, x: usize, y: usize, value: BrailleChar) {
        assert!(x < COLUMNS * 2);
        assert!(y < ROWS * 4);

        self.array[y][x] = value;
    }

    pub fn set_char_unchecked(&mut self, x: usize, y: usize, value: BrailleChar) {
        self.array[y][x] = value;
    }
}

