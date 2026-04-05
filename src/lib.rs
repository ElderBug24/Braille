mod array;
mod vector;

pub use array::BrailleCharGridArray;
pub use vector::BrailleCharGridVector;

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign, Range};
use std::fmt::{self, Debug};


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
         (b & 0b_0000_0001)       != 0
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
    const OFFSET: [u8; 8] = [0, 1, 2, 4, 6, 3, 5, 7];

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
    const OFFSET: [u8; 8] = [0, 3, 1, 4, 2, 5, 6, 7];

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
pub const fn get_bit(byte: u8, index: u8) -> bool {
    assert!(index < 8);

    return unsafe { get_bit_unchecked(byte, index) };
}

#[inline(always)]
pub const unsafe fn get_bit_unchecked(byte: u8, index: u8) -> bool {
    return ((byte >> (7 - index)) & 1) != 0;
}

#[inline(always)]
pub const fn get_bit_2d(byte: u8, x: u8, y: u8) -> bool {
    assert!(x < 2);
    assert!(y < 4);

    return unsafe { get_bit_2d_unchecked(byte, x, y) };
}

#[inline(always)]
pub const unsafe fn get_bit_2d_unchecked(byte: u8, x: u8, y: u8) -> bool {
    return unsafe { get_bit_unchecked(byte, x + y * 2) };
}

#[inline(always)]
pub const fn set_bit(byte: u8, index: u8, value: bool) -> u8 {
    assert!(index < 8);

    return unsafe { set_bit_unchecked(byte, index, value) };
}

#[inline(always)]
pub const unsafe fn set_bit_unchecked(byte: u8, index: u8, value: bool) -> u8 {
    return (byte & !(0b_1000_0000 >> index)) | ((value as u8) << (7 - index));
}

#[inline(always)]
pub const fn set_bit_2d(byte: u8, x: u8, y: u8, value: bool) -> u8 {
    assert!(x < 2);
    assert!(y < 4);

    return unsafe { set_bit_2d_unchecked(byte, x, y, value) };
}

#[inline(always)]
pub const unsafe fn set_bit_2d_unchecked(byte: u8, x: u8, y: u8, value: bool) -> u8 {
    return unsafe { set_bit_unchecked(byte, x + y * 2, value) };
}

pub const MAP_ORDERED_TO_UNORDERED: [u8; 8] = [7, 6, 5, 3, 1, 4, 2, 0];
pub const MAP_UNORDERED_TO_ORDERED: [u8; 8] = [7, 4, 6, 3, 5, 2, 1, 0];
pub const MAP_TRANSPARENT:          [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BrailleChar(u8);

impl BrailleChar {
    pub const WIDTH:  usize = 2;
    pub const HEIGHT: usize = 4;
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    pub const EMPTY: Self = Self(0u8);
    pub const FULL:  Self = Self(255u8);
    pub const IS_ORDERED: bool = true;
    pub const MAP_TO_UNORDERED: [u8; 8] = MAP_ORDERED_TO_UNORDERED;
    pub const MAP_TO_ORDERED:   [u8; 8] = MAP_TRANSPARENT;

    #[inline(always)]
    pub const fn ordered(&self) -> u8 {
        return self.0;
    }

    #[inline(always)]
    pub const fn unordered(&self) -> u8 {
        let Self(b) = self;
        let b = ordered_to_unordered(*b);

        return b;
    }

    #[inline(always)]
    pub const fn from_ordered(b: u8) -> Self {
        return Self(b);
    }

    #[inline(always)]
    pub const fn from_unordered(b: u8) -> Self {
        let b = unordered_to_ordered(b);

        return Self(b);
    }

    #[inline(always)]
    pub const fn from_array_ordered(array: &[bool; 8]) -> Self {
        let byte = array_to_byte(array);

        return Self::from_ordered(byte);
    }

    #[inline(always)]
    pub const fn from_array_unordered(array: &[bool; 8]) -> Self {
        let byte = array_unordered_to_byte_ordered(array);

        return Self::from_ordered(byte);
    }

    #[inline(always)]
    pub const fn from_slice_ordered(slice: &[bool]) -> Self {
        let byte = slice_to_byte(slice);

        return Self::from_ordered(byte);
    }

    #[inline(always)]
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

    #[inline(always)]
    pub const fn from_u32_char(char: u32) -> Option<Self> {
        const MIN: u32 = BrailleChar::CHAR_RANGE.start;
        const MAX: u32 = BrailleChar::CHAR_RANGE.end;

        return match char {
            MIN..MAX => Some(unsafe { Self::from_u32_char_unchecked(char) }),
            _ => None
        };
    }

    #[inline(always)]
    pub const unsafe fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_ordered((char - Self::CHAR_RANGE.start) as u8);
    }

    #[inline(always)]
    pub const fn from_char(char: char) -> Option<Self> {
        return Self::from_u32_char(char as u32);
    }

    #[inline(always)]
    pub const unsafe fn from_char_unchecked(char: char) -> Self {
        return unsafe { Self::from_u32_char_unchecked(char as u32) };
    }

    #[inline(always)]
    pub const fn to_unordered(&self) -> BrailleCharUnOrdered {
        return BrailleCharUnOrdered(self.unordered());
    }

    #[inline(always)]
    pub const fn to_unordered_raw(&self) -> BrailleCharUnOrdered {
        return BrailleCharUnOrdered(self.0);
    }

    #[inline(always)]
    pub const fn get_at(&self, index: u8) -> bool {
        return get_bit(self.0, index);
    }

    #[inline(always)]
    pub const unsafe fn get_at_unchecked(&self, index: u8) -> bool {
        return unsafe { get_bit_unchecked(self.0, index) };
    }

    #[inline(always)]
    pub const fn get_at_xy(&self, x: u8, y: u8) -> bool {
        return get_bit_2d(self.unordered(), x, y);
    }

    #[inline(always)]
    pub const unsafe fn get_at_xy_unchecked(&self, x: u8, y: u8) -> bool {
        return unsafe { get_bit_2d_unchecked(self.unordered(), x, y) };
    }

    #[inline(always)]
    pub const fn set_at(&mut self, index: u8, value: bool) {
        self.0 = set_bit(self.0, index, value);
    }

    #[inline(always)]
    pub const unsafe fn set_at_unchecked(&mut self, index: u8, value: bool) {
        self.0 = unsafe { set_bit_unchecked(self.0, index, value) };
    }

    #[inline(always)]
    pub const fn set_at_xy(&mut self, x: u8, y: u8, value: bool) {
        *self = Self::from_unordered(set_bit_2d(self.unordered(), x, y, value));
    }

    #[inline(always)]
    pub const unsafe fn set_at_xy_unchecked(&mut self, x: u8, y: u8, value: bool) {
        *self = Self::from_unordered(unsafe { set_bit_2d_unchecked(self.unordered(), x, y, value) });
    }

    #[inline(always)]
    pub const fn bitand(&self, rhs: &Self) -> Self {
        return Self(self.0 & rhs.0);
    }

    #[inline(always)]
    pub const fn bitand_assign(&mut self, rhs: &Self) {
        *self = Self::bitand(self, rhs);
    }

    #[inline(always)]
    pub const fn bitor(&self, rhs: &Self) -> Self {
        return Self(self.0 | rhs.0);
    }

    #[inline(always)]
    pub const fn bitor_assign(&mut self, rhs: &Self) {
        *self = Self::bitor(self, rhs);
    }

    #[inline(always)]
    pub const fn bitxor(&self, rhs: &Self) -> Self {
        return Self(self.0 ^ rhs.0);
    }

    #[inline(always)]
    pub const fn bitxor_assign(&mut self, rhs: &Self) {
        *self = Self::bitxor(self, rhs);
    }

    #[inline(always)]
    pub const fn not(&self) -> Self {
        return Self(!self.0);
    }

    #[inline(always)]
    pub const fn not_assign(&mut self) {
        *self = Self::not(self);
    }

    #[inline(always)]
    pub const fn shl(&self, rhs: u8) -> Self {
        return Self(self.0 << rhs);
    }

    #[inline(always)]
    pub const fn shl_assign(&mut self, rhs: u8) {
        *self = Self::shl(self, rhs);
    }

    #[inline(always)]
    pub const fn shr(&self, rhs: u8) -> Self {
        return Self(self.0 >> rhs);
    }

    #[inline(always)]
    pub const fn shr_assign(&mut self, rhs: u8) {
        *self = Self::shr(self, rhs);
    }
}

impl BrailleCharTrait for BrailleChar {
    const WIDTH:  usize = Self::WIDTH;
    const HEIGHT: usize = Self::HEIGHT;
    const CHAR_RANGE: Range<u32> = Self::CHAR_RANGE;
    const EMPTY: Self = Self::EMPTY;
    const FULL:  Self = Self::FULL;
    const IS_ORDERED: bool = Self::IS_ORDERED;
    const MAP_TO_UNORDERED: [u8; 8] = Self::MAP_TO_UNORDERED;
    const MAP_TO_ORDERED:   [u8; 8] = Self::MAP_TO_ORDERED;

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
    fn to_ordered(&self) -> BrailleChar {
        return *self;
    }

    #[inline(always)]
    fn to_unordered(&self) -> BrailleCharUnOrdered {
        return Self::to_unordered(self);
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
    unsafe fn from_u32_char_unchecked(char: u32) -> Self {
        return unsafe { Self::from_u32_char_unchecked(char) };
    }

    #[inline(always)]
    fn from_char(char: char) -> Option<Self> {
        return Self::from_char(char);
    }

    #[inline(always)]
    unsafe fn from_char_unchecked(char: char) -> Self {
        return unsafe { Self::from_char_unchecked(char) };
    }

    #[inline(always)]
    fn get_at(&self, index: u8) -> bool {
        return Self::get_at(self, index);
    }

    #[inline(always)]
    unsafe fn get_at_unchecked(&self, index: u8) -> bool {
        unsafe { return Self::get_at_unchecked(self, index) };
    }

    #[inline(always)]
    fn get_at_xy(&self, x: u8, y: u8) -> bool {
        return Self::get_at_xy(self, x, y);
    }

    #[inline(always)]
    unsafe fn get_at_xy_unchecked(&self, x: u8, y: u8) -> bool {
        return unsafe { Self::get_at_xy_unchecked(self, x, y) };
    }

    #[inline(always)]
    fn set_at(&mut self, index: u8, value: bool) {
        Self::set_at(self, index, value);
    }

    #[inline(always)]
    unsafe fn set_at_unchecked(&mut self, index: u8, value: bool) {
        unsafe { Self::set_at_unchecked(self, index, value) };
    }

    #[inline(always)]
    fn set_at_xy(&mut self, x: u8, y: u8, value: bool) {
        Self::set_at_xy(self, x, y, value);
    }

    #[inline(always)]
    unsafe fn set_at_xy_unchecked(&mut self, x: u8, y: u8, value: bool) {
        unsafe { Self::set_at_xy_unchecked(self, x, y, value) };
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BrailleCharUnOrdered(u8);

impl BrailleCharUnOrdered {
    pub const WIDTH:  usize = 2;
    pub const HEIGHT: usize = 4;
    pub const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    pub const EMPTY: Self = Self(0u8);
    pub const FULL:  Self = Self(255u8);
    pub const IS_ORDERED: bool = false;
    pub const MAP_TO_UNORDERED: [u8; 8] = MAP_TRANSPARENT;
    pub const MAP_TO_ORDERED:   [u8; 8] = MAP_UNORDERED_TO_ORDERED;

    #[inline(always)]
    pub const fn ordered(&self) -> u8 {
        let Self(b) = self;
        let b = unordered_to_ordered(*b);

        return b;
    }

    #[inline(always)]
    pub const fn unordered(&self) -> u8 {
        return self.0;
    }

    #[inline(always)]
    pub const fn from_ordered(b: u8) -> Self {
        let b = ordered_to_unordered(b);

        return Self(b);
    }

    #[inline(always)]
    pub const fn from_unordered(b: u8) -> Self {
        return Self(b);
    }

    #[inline(always)]
    pub const fn from_array_ordered(array: &[bool; 8]) -> Self {
        let byte = array_ordered_to_byte_unordered(array);

        return Self::from_unordered(byte);
    }

    #[inline(always)]
    pub const fn from_array_unordered(array: &[bool; 8]) -> Self {
        let byte = array_to_byte(array);

        return Self::from_unordered(byte);
    }

    #[inline(always)]
    pub const fn from_slice_ordered(slice: &[bool]) -> Self {
        let byte = slice_ordered_to_byte_unordered(slice);

        return Self::from_unordered(byte);
    }

    #[inline(always)]
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

    #[inline(always)]
    pub const fn from_u32_char(char: u32) -> Option<Self> {
        const MIN: u32 = BrailleChar::CHAR_RANGE.start;
        const MAX: u32 = BrailleChar::CHAR_RANGE.end;

        return match char {
            MIN..MAX => Some(unsafe { Self::from_u32_char_unchecked(char) }),
            _ => None
        };
    }

    #[inline(always)]
    pub const unsafe fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::from_ordered((char - Self::CHAR_RANGE.start) as u8);
    }

    #[inline(always)]
    pub const fn from_char(char: char) -> Option<Self> {
        return Self::from_u32_char(char as u32);
    }

    #[inline(always)]
    pub const unsafe fn from_char_unchecked(char: char) -> Self {
        return unsafe { Self::from_u32_char_unchecked(char as u32) };
    }

    #[inline(always)]
    pub const fn to_ordered(&self) -> BrailleChar {
        return BrailleChar(self.ordered());
    }

    #[inline(always)]
    pub const fn to_ordered_raw(&self) -> BrailleChar {
        return BrailleChar(self.0);
    }

    #[inline(always)]
    pub const fn get_at(&self, index: u8) -> bool {
        return get_bit(self.0, index);
    }

    #[inline(always)]
    pub const unsafe fn get_at_unchecked(&self, index: u8) -> bool {
        return unsafe { get_bit_unchecked(self.0, index) };
    }

    #[inline(always)]
    pub const fn get_at_xy(&self, x: u8, y: u8) -> bool {
        return get_bit_2d(self.unordered(), x, y);
    }

    #[inline(always)]
    pub const unsafe fn get_at_xy_unchecked(&self, x: u8, y: u8) -> bool {
        return unsafe { get_bit_2d_unchecked(self.unordered(), x, y) };
    }

    #[inline(always)]
    pub const fn set_at(&mut self, index: u8, value: bool) {
        self.0 = set_bit(self.0, index, value);
    }

    #[inline(always)]
    pub const unsafe fn set_at_unchecked(&mut self, index: u8, value: bool) {
        self.0 = unsafe { set_bit_unchecked(self.0, index, value) };
    }

    #[inline(always)]
    pub const fn set_at_xy(&mut self, x: u8, y: u8, value: bool) {
        *self = Self::from_unordered(set_bit_2d(self.unordered(), x, y, value));
    }

    #[inline(always)]
    pub const unsafe fn set_at_xy_unchecked(&mut self, x: u8, y: u8, value: bool) {
        *self = Self::from_unordered(unsafe { set_bit_2d_unchecked(self.unordered(), x, y, value) });
    }

    #[inline(always)]
    pub const fn bitand(&self, rhs: &Self) -> Self {
        return Self(self.0 & rhs.0);
    }

    #[inline(always)]
    pub const fn bitand_assign(&mut self, rhs: &Self) {
        *self = Self::bitand(self, rhs);
    }

    #[inline(always)]
    pub const fn bitor(&self, rhs: &Self) -> Self {
        return Self(self.0 | rhs.0);
    }

    #[inline(always)]
    pub const fn bitor_assign(&mut self, rhs: &Self) {
        *self = Self::bitor(self, rhs);
    }

    #[inline(always)]
    pub const fn bitxor(&self, rhs: &Self) -> Self {
        return Self(self.0 ^ rhs.0);
    }

    #[inline(always)]
    pub const fn bitxor_assign(&mut self, rhs: &Self) {
        *self = Self::bitxor(self, rhs);
    }

    #[inline(always)]
    pub const fn not(&self) -> Self {
        return Self(!self.0);
    }

    #[inline(always)]
    pub const fn not_assign(&mut self) {
        *self = Self(!self.0);
    }

    #[inline(always)]
    pub const fn shl(&self, rhs: u8) -> Self {
        return Self(self.0 << rhs);
    }

    #[inline(always)]
    pub const fn shl_assign(&mut self, rhs: u8) {
        *self = Self::shl(self, rhs);
    }

    #[inline(always)]
    pub const fn shr(&self, rhs: u8) -> Self {
        return Self(self.0 >> rhs);
    }

    #[inline(always)]
    pub const fn shr_assign(&mut self, rhs: u8) {
        *self = Self::shr(self, rhs);
    }
}

impl BrailleCharTrait for BrailleCharUnOrdered {
    const WIDTH:  usize = Self::WIDTH;
    const HEIGHT: usize = Self::HEIGHT;
    const CHAR_RANGE: Range<u32> = Self::CHAR_RANGE;
    const EMPTY: Self = Self::EMPTY;
    const FULL:  Self = Self::FULL;
    const IS_ORDERED: bool = Self::IS_ORDERED;
    const MAP_TO_UNORDERED: [u8; 8] = Self::MAP_TO_UNORDERED;
    const MAP_TO_ORDERED:   [u8; 8] = Self::MAP_TO_ORDERED;

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
    unsafe fn from_u32_char_unchecked(char: u32) -> Self {
        return unsafe { Self::from_u32_char_unchecked(char) };
    }

    #[inline(always)]
    fn from_char(char: char) -> Option<Self> {
        return Self::from_char(char);
    }

    #[inline(always)]
    unsafe fn from_char_unchecked(char: char) -> Self {
        return unsafe { Self::from_char_unchecked(char) };
    }

    #[inline(always)]
    fn to_ordered(&self) -> BrailleChar {
        return Self::to_ordered(self);
    }

    #[inline(always)]
    fn to_unordered(&self) -> BrailleCharUnOrdered {
        return *self;
    }

    #[inline(always)]
    fn get_at(&self, index: u8) -> bool {
        return Self::get_at(self, index);
    }

    #[inline(always)]
    unsafe fn get_at_unchecked(&self, index: u8) -> bool {
        unsafe { return Self::get_at_unchecked(self, index) };
    }

    #[inline(always)]
    fn get_at_xy(&self, x: u8, y: u8) -> bool {
        return Self::get_at_xy(self, x, y);
    }

    #[inline(always)]
    unsafe fn get_at_xy_unchecked(&self, x: u8, y: u8) -> bool {
        return unsafe { Self::get_at_xy_unchecked(self, x, y) };
    }

    #[inline(always)]
    fn set_at(&mut self, index: u8, value: bool) {
        Self::set_at(self, index, value);
    }

    #[inline(always)]
    unsafe fn set_at_unchecked(&mut self, index: u8, value: bool) {
        unsafe { Self::set_at_unchecked(self, index, value) };
    }

    #[inline(always)]
    fn set_at_xy(&mut self, x: u8, y: u8, value: bool) {
        Self::set_at_xy(self, x, y, value);
    }

    #[inline(always)]
    unsafe fn set_at_xy_unchecked(&mut self, x: u8, y: u8, value: bool) {
        unsafe { Self::set_at_xy_unchecked(self, x, y, value) };
    }
}

pub trait BrailleCharTrait: Sized + Copy + Clone + PartialEq + Eq + Debug + Default + Into<char> + Into<u32> + BitAnd<Self> + BitAndAssign<Self> + BitOr<Self> + BitOrAssign<Self> + BitXor<Self> + BitXorAssign<Self> + Not + Shl<u8> + ShlAssign<u8> + Shr<u8> + ShrAssign<u8> {
    const WIDTH:  usize = 2;
    const HEIGHT: usize = 4;
    const CHAR_RANGE: Range<u32> = 0x2800..(0x2800 + u8::MAX as u32);
    const EMPTY: Self;
    const FULL:  Self;
    const IS_ORDERED: bool;
    const MAP_TO_UNORDERED: [u8; 8];
    const MAP_TO_ORDERED:   [u8; 8];

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

    unsafe fn from_u32_char_unchecked(char: u32) -> Self;

    fn from_char(char: char) -> Option<Self>;

    unsafe fn from_char_unchecked(char: char) -> Self;

    fn to_ordered(&self) -> BrailleChar;

    fn to_unordered(&self) -> BrailleCharUnOrdered;

    fn get_at(&self, index: u8) -> bool;

    unsafe fn get_at_unchecked(&self, index: u8) -> bool;

    fn get_at_xy(&self, x: u8, y: u8) -> bool;

    unsafe fn get_at_xy_unchecked(&self, x: u8, y: u8) -> bool;

    fn set_at(&mut self, index: u8, value: bool);

    unsafe fn set_at_unchecked(&mut self, index: u8, value: bool);

    fn set_at_xy(&mut self, x: u8, y: u8, value: bool);

    unsafe fn set_at_xy_unchecked(&mut self, x: u8, y: u8, value: bool);
}

impl Default for BrailleChar {
    fn default() -> Self {
        return Self::EMPTY;
    }
}

impl Default for BrailleCharUnOrdered {
    fn default() -> Self {
        return Self::EMPTY;
    }
}

impl Debug for BrailleChar {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "BrailleChar({:08b}) = '{}'", self.0, self.char());
    }
}

impl Debug for BrailleCharUnOrdered {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "BrailleCharUnOrdered({:08b}) = '{}'", self.0, self.char());
    }
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
        return Self::char(&self);
    }
}

impl Into<char> for BrailleCharUnOrdered {
    #[inline(always)]
    fn into(self) -> char {
        return Self::char(&self);
    }
}

impl Into<u32> for BrailleChar {
    #[inline(always)]
    fn into(self) -> u32 {
        return Self::u32_char(&self);
    }
}

impl Into<u32> for BrailleCharUnOrdered {
    #[inline(always)]
    fn into(self) -> u32 {
        return Self::u32_char(&self);
    }
}

impl BitAnd<Self> for BrailleChar {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        return Self::bitand(&self, &rhs);
    }
}

impl BitAnd<Self> for BrailleCharUnOrdered {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        return Self::bitand(&self, &rhs);
    }
}

impl BitAndAssign<Self> for BrailleChar {
    fn bitand_assign(&mut self, rhs: Self) {
        Self::bitand_assign(self, &rhs);
    }
}

impl BitAndAssign<Self> for BrailleCharUnOrdered {
    fn bitand_assign(&mut self, rhs: Self) {
        Self::bitand_assign(self, &rhs);
    }
}

impl BitOr<Self> for BrailleChar {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Self::bitor(&self, &rhs);
    }
}

impl BitOr<Self> for BrailleCharUnOrdered {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        return Self::bitor(&self, &rhs);
    }
}

impl BitOrAssign<Self> for BrailleChar {
    fn bitor_assign(&mut self, rhs: Self) {
        Self::bitor_assign(self, &rhs);
    }
}

impl BitOrAssign<Self> for BrailleCharUnOrdered {
    fn bitor_assign(&mut self, rhs: Self) {
        Self::bitor_assign(self, &rhs);
    }
}

impl BitXor<Self> for BrailleChar {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return Self::bitxor(&self, &rhs);
    }
}

impl BitXor<Self> for BrailleCharUnOrdered {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return Self::bitxor(&self, &rhs);
    }
}

impl BitXorAssign<Self> for BrailleChar {
    fn bitxor_assign(&mut self, rhs: Self) {
        Self::bitxor_assign(self, &rhs);
    }
}

impl BitXorAssign<Self> for BrailleCharUnOrdered {
    fn bitxor_assign(&mut self, rhs: Self) {
        Self::bitxor_assign(self, &rhs);
    }
}

impl Not for BrailleChar {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self::not(&self);
    }
}

impl Not for BrailleCharUnOrdered {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self::not(&self);
    }
}

impl Shl<u8> for BrailleChar {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        return Self::shl(&self, rhs);
    }
}

impl Shl<u8> for BrailleCharUnOrdered {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        return Self::shl(&self, rhs);
    }
}

impl ShlAssign<u8> for BrailleChar {
    fn shl_assign(&mut self, rhs: u8) {
        Self::shl_assign(self, rhs);
    }
}

impl ShlAssign<u8> for BrailleCharUnOrdered {
    fn shl_assign(&mut self, rhs: u8) {
        Self::shl_assign(self, rhs);
    }
}

impl Shr<u8> for BrailleChar {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        return Self::shr(&self, rhs);
    }
}

impl Shr<u8> for BrailleCharUnOrdered {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        return Self::shr(&self, rhs);
    }
}

impl ShrAssign<u8> for BrailleChar {
    fn shr_assign(&mut self, rhs: u8) {
        Self::shr_assign(self, rhs);
    }
}

impl ShrAssign<u8> for BrailleCharUnOrdered {
    fn shr_assign(&mut self, rhs: u8) {
        Self::shr_assign(self, rhs);
    }
}

