pub mod array;
pub mod vector;

pub use array::BrailleCharGridArray;
pub use vector::BrailleCharGridVector;

use std::ops::Range;


#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
pub const fn byte_to_array(b: u8) -> [bool; 8] {
    return [
        ((b & 0b_1000_0000) >> 7) != 0,
        ((b & 0b_0100_0000) >> 6) != 0,
        ((b & 0b_0010_0000) >> 5) != 0,
        ((b & 0b_0001_0000) >> 4) != 0,
        ((b & 0b_0000_1000) >> 3) != 0,
        ((b & 0b_0000_0100) >> 2) != 0,
        ((b & 0b_0000_0010) >> 1) != 0,
         (b & 0b_0000_0001) != 0
    ];
}

#[inline(always)]
pub const fn array_to_byte(array: &[bool; 8]) -> u8 {
    return (array[0] as u8) << 7
         | (array[1] as u8) << 6
         | (array[2] as u8) << 5
         | (array[3] as u8) << 4
         | (array[4] as u8) << 3
         | (array[5] as u8) << 2
         | (array[6] as u8) << 1
         | (array[7] as u8);
}

#[inline(always)]
pub const fn array_ordered_to_byte_unordered(array: &[bool; 8]) -> u8 {
    return (array[0] as u8)
         | (array[1] as u8) << 1
         | (array[2] as u8) << 2
         | (array[3] as u8) << 4
         | (array[4] as u8) << 6
         | (array[5] as u8) << 3
         | (array[6] as u8) << 5
         | (array[7] as u8) << 7;
}

#[inline(always)]
pub const fn array_unordered_to_byte_ordered(array: &[bool; 8]) -> u8 {
    return (array[0] as u8)
         | (array[1] as u8) << 3
         | (array[2] as u8) << 1
         | (array[3] as u8) << 4
         | (array[4] as u8) << 2
         | (array[5] as u8) << 5
         | (array[6] as u8) << 6
         | (array[7] as u8) << 7;
}

#[inline(always)]
pub const fn slice_to_byte(slice: &[bool]) -> u8 {
    let mut byte = 0u8;

    let len = slice.len();
    let mut i = 0;

    while i < len {
        let b = slice[i];

        byte |= (b as u8) << (7 - i);

        i += 1;
    }

    return byte;
}

#[inline(always)]
pub const fn slice_ordered_to_byte_unordered(slice: &[bool]) -> u8 {
    const OFFSET: [usize; 8] = [0, 1, 2, 4, 6, 3, 5, 7];

    let mut byte = 0u8;

    let len = slice.len();
    let mut i = 0;

    while i < len {
        let b = slice[i];

        let o = OFFSET[i];
        byte |= (b as u8) << o;

        i += 1;
    }

    return byte;
}

#[inline(always)]
pub const fn slice_unordered_to_byte_ordered(slice: &[bool]) -> u8 {
    const OFFSET: [usize; 8] = [0, 3, 1, 4, 2, 5, 6, 7];

    let mut byte = 0u8;

    let len = slice.len();
    let mut i = 0;

    while i < len {
        let b = slice[i];

        let o = OFFSET[i];
        byte |= (b as u8) << o;

        i += 1;
    }

    return byte;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BrailleChar(u8);

impl BrailleChar {
    pub const EMPTY: Self = Self(0u8);
    pub const FULL: Self = Self(255u8);
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 4;

    #[inline(always)]
    pub const fn ordered(&self) -> u8 {
        return self.0;
    }

    pub const fn unordered(&self) -> u8 {
        let Self(b) = self;
        let b = ordered_to_unordered(*b);

        return b;
    }

    #[inline(always)]
    pub const fn from_ordered(b: u8) -> Self {
        return Self(b);
    }

    pub const fn from_unordered(b: u8) -> Self {
        let b = unordered_to_ordered(b);

        return Self(b);
    }

    pub const fn from_array_ordered(array: &[bool; 8]) -> Self {
        let byte = array_to_byte(array);

        return Self::from_ordered(byte);
    }

    pub const fn from_array_unordered(array: &[bool; 8]) -> Self {
        let byte = array_unordered_to_byte_ordered(array);

        return Self::from_ordered(byte);
    }

    pub const fn from_slice_ordered(slice: &[bool]) -> Self {
        let byte = slice_to_byte(slice);

        return Self::from_ordered(byte);
    }

    pub const fn from_slice_unordered(slice: &[bool]) -> Self {
        let byte = slice_unordered_to_byte_ordered(slice);

        return Self::from_ordered(byte);
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
            MIN..MAX => Some(Self::from_u32_char_unchecked(char)),
            _ => None
        };
    }

    #[inline(always)]
    pub const fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_ordered((char - Self::CHAR_RANGE.start) as u8);
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

impl BrailleCharTrait for BrailleChar {
    const EMPTY: Self = Self::EMPTY;
    const FULL: Self = Self::FULL;

    #[inline(always)]
    fn ordered(&self) -> u8 {
        return Self::ordered(self);
    }

    #[inline(always)]
    fn unordered(&self) -> u8 {
        return Self::unordered(self);
    }

    #[inline(always)]
    fn from_ordered(b: u8) -> Self {
        return Self::from_ordered(b);
    }

    #[inline(always)]
    fn from_unordered(b: u8) -> Self {
        return Self::from_unordered(b);
    }

    #[inline(always)]
    fn from_array_ordered(array: &[bool; 8]) -> Self {
        return Self::from_array_ordered(array);
    }

    #[inline(always)]
    fn from_array_unordered(array: &[bool; 8]) -> Self {
        return Self::from_array_unordered(array);
    }

    #[inline(always)]
    fn from_slice_ordered(slice: &[bool]) -> Self {
        return Self::from_slice_ordered(slice);
    }

    #[inline(always)]
    fn from_slice_unordered(slice: &[bool]) -> Self {
        return Self::from_slice_unordered(slice);
    }

    #[inline(always)]
    fn u32_char(&self) -> u32 {
        return Self::u32_char(self);
    }

    #[inline(always)]
    fn char(&self) -> char {
        return Self::char(self);
    }

    #[inline(always)]
    fn from_u32_char(char: u32) -> Option<Self> {
        return Self::from_u32_char(char);
    }

    #[inline(always)]
    fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_u32_char_unchecked(char);
    }

    #[inline(always)]
    fn from_char(char: char) -> Option<Self> {
        return Self::from_char(char);
    }

    #[inline(always)]
    fn from_char_unchecked(char: char) -> Self {
        return Self::from_char_unchecked(char);
    }

    #[inline(always)]
    fn get(&self, x: u8, y: u8) -> bool {
        return Self::get(self, x, y);
    }

    #[inline(always)]
    fn get_unchecked(&self, x: u8, y: u8) -> bool {
        return Self::get_unchecked(self, x, y);
    }

    #[inline(always)]
    fn set(&mut self, x: u8, y: u8, value: bool) {
        Self::set(self, x, y, value);
    }

    #[inline(always)]
    fn set_unchecked(&mut self, x: u8, y: u8, value: bool) {
        Self::set_unchecked(self, x, y, value);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BrailleCharUnOrdered(u8);

impl BrailleCharUnOrdered {
    pub const EMPTY: Self = Self(0u8);
    pub const FULL: Self = Self(255u8);
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    pub const WIDTH: usize = 2;
    pub const HEIGHT: usize = 4;

    pub const fn ordered(&self) -> u8 {
        let Self(b) = self;
        let b = unordered_to_ordered(*b);

        return b;
    }

    #[inline(always)]
    pub const fn unordered(&self) -> u8 {
        return self.0;
    }

    pub const fn from_ordered(b: u8) -> Self {
        let b = ordered_to_unordered(b);

        return Self(b);
    }

    #[inline(always)]
    pub const fn from_unordered(b: u8) -> Self {
        return Self(b);
    }

    pub const fn from_array_ordered(array: &[bool; 8]) -> Self {
        let byte = array_ordered_to_byte_unordered(array);

        return Self::from_unordered(byte);
    }

    pub const fn from_array_unordered(array: &[bool; 8]) -> Self {
        let byte = array_to_byte(array);

        return Self::from_unordered(byte);
    }

    pub const fn from_slice_ordered(slice: &[bool]) -> Self {
        let byte = slice_ordered_to_byte_unordered(slice);

        return Self::from_unordered(byte);
    }

    pub const fn from_slice_unordered(slice: &[bool]) -> Self {
        let byte = slice_to_byte(slice);

        return Self::from_unordered(byte);
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
            MIN..MAX => Some(Self::from_u32_char_unchecked(char)),
            _ => None
        };
    }

    #[inline(always)]
    pub const fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_ordered((char - Self::CHAR_RANGE.start) as u8);
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

impl BrailleCharTrait for BrailleCharUnOrdered {
    const EMPTY: Self = Self::EMPTY;
    const FULL: Self = Self::FULL;

    #[inline(always)]
    fn ordered(&self) -> u8 {
        return Self::ordered(self);
    }

    #[inline(always)]
    fn unordered(&self) -> u8 {
        return Self::unordered(self);
    }

    #[inline(always)]
    fn from_ordered(b: u8) -> Self {
        return Self::from_ordered(b);
    }

    #[inline(always)]
    fn from_unordered(b: u8) -> Self {
        return Self::from_unordered(b);
    }

    #[inline(always)]
    fn from_array_ordered(array: &[bool; 8]) -> Self {
        return Self::from_array_ordered(array);
    }

    #[inline(always)]
    fn from_array_unordered(array: &[bool; 8]) -> Self {
        return Self::from_array_unordered(array);
    }

    #[inline(always)]
    fn from_slice_ordered(slice: &[bool]) -> Self {
        return Self::from_slice_ordered(slice);
    }

    #[inline(always)]
    fn from_slice_unordered(slice: &[bool]) -> Self {
        return Self::from_slice_unordered(slice);
    }

    #[inline(always)]
    fn u32_char(&self) -> u32 {
        return Self::u32_char(self);
    }

    #[inline(always)]
    fn char(&self) -> char {
        return Self::char(self);
    }

    #[inline(always)]
    fn from_u32_char(char: u32) -> Option<Self> {
        return Self::from_u32_char(char);
    }

    #[inline(always)]
    fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_u32_char_unchecked(char);
    }

    #[inline(always)]
    fn from_char(char: char) -> Option<Self> {
        return Self::from_char(char);
    }

    #[inline(always)]
    fn from_char_unchecked(char: char) -> Self {
        return Self::from_char_unchecked(char);
    }

    #[inline(always)]
    fn get(&self, x: u8, y: u8) -> bool {
        return Self::get(self, x, y);
    }

    #[inline(always)]
    fn get_unchecked(&self, x: u8, y: u8) -> bool {
        return Self::get_unchecked(self, x, y);
    }

    #[inline(always)]
    fn set(&mut self, x: u8, y: u8, value: bool) {
        Self::set(self, x, y, value);
    }

    #[inline(always)]
    fn set_unchecked(&mut self, x: u8, y: u8, value: bool) {
        Self::set_unchecked(self, x, y, value);
    }
}

pub trait BrailleCharTrait: Sized + Copy + Clone + PartialEq + Eq + std::fmt::Debug {
    const EMPTY: Self;
    const FULL: Self;
    const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    const WIDTH: usize = 2;
    const HEIGHT: usize = 4;

    fn ordered(&self) -> u8;

    fn unordered(&self) -> u8;

    fn from_ordered(b: u8) -> Self;

    fn from_unordered(b: u8) -> Self;

    fn from_array_ordered(array: &[bool; 8]) -> Self;

    fn from_array_unordered(array: &[bool; 8]) -> Self;

    fn from_slice_ordered(slice: &[bool]) -> Self;

    fn from_slice_unordered(slice: &[bool]) -> Self;

    fn u32_char(&self) -> u32;

    fn char(&self) -> char;

    fn from_u32_char(char: u32) -> Option<Self>;

    fn from_u32_char_unchecked(char: u32) -> Self;

    fn from_char(char: char) -> Option<Self>;

    fn from_char_unchecked(char: char) -> Self;

    fn get(&self, x: u8, y: u8) -> bool;

    fn get_unchecked(&self, x: u8, y: u8) -> bool;

    fn set(&mut self, x: u8, y: u8, value: bool);

    fn set_unchecked(&mut self, x: u8, y: u8, value: bool);
}

impl From<BrailleCharUnOrdered> for BrailleChar {
    #[inline(always)]
    fn from(value: BrailleCharUnOrdered) -> Self {
        return Self::from_unordered(value.unordered());
    }
}

impl From<BrailleChar> for BrailleCharUnOrdered {
    #[inline(always)]
    fn from(value: BrailleChar) -> Self {
        return Self::from_ordered(value.ordered());
    }
}

impl Into<char> for BrailleChar {
    #[inline(always)]
    fn into(self) -> char {
        return self.char();
    }
}

impl Into<char> for BrailleCharUnOrdered {
    #[inline(always)]
    fn into(self) -> char {
        return self.char();
    }
}

impl Into<u32> for BrailleChar {
    #[inline(always)]
    fn into(self) -> u32 {
        return self.u32_char();
    }
}

impl Into<u32> for BrailleCharUnOrdered {
    #[inline(always)]
    fn into(self) -> u32 {
        return self.u32_char();
    }
}

