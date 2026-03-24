

#[derive(Copy, Clone, Debug)]
pub enum BrailleChar {
    UnOrdered(u8),
    Ordered(u8)
}

impl BrailleChar {
    const OFFSET: u32 = 0x2800;

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

    pub fn into_unordered(&self) -> Self {
        return Self::UnOrdered(self.unordered());
    }

    pub fn into_ordered(&self) -> Self {
        return Self::Ordered(self.ordered());
    }

    pub fn into_ordered_assign(&mut self) {
        *self = Self::Ordered(self.ordered());
    }

    pub fn into_unordered_assign(&mut self) {
        *self = Self::UnOrdered(self.unordered());
    }

    pub fn u32_char(&self) -> u32 {
        return Self::OFFSET + self.ordered() as u32;
    }

    pub fn char(&self) -> char {
        return unsafe { char::from_u32_unchecked(self.u32_char()) };
    }

    pub fn from_u32_char(char: u32) -> Option<Self> {
        const MAX: u32 = BrailleChar::OFFSET + u8::MAX as u32;
        return match char {
            Self::OFFSET..MAX => Some(Self::Ordered((char - Self::OFFSET) as u8)),
            _ => None
        };
    }

    pub fn from_u32_char_unchecked(char: u32) -> Self {
        return Self::Ordered((char - Self::OFFSET) as u8);
    }

    pub fn from_char(char: char) -> Option<Self> {
        return Self::from_u32_char(char as u32);
    }

    pub fn from_char_unchecked(char: char) -> Self {
        return Self::from_u32_char_unchecked(char as u32);
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        assert!(x < 2);
        assert!(y < 4);

        return (self.unordered() & (0b_1000_0000 >> (x + y * 2))) != 0;
    }

    pub fn get_unchecked(&self, x: u8, y: u8) -> bool {
        return (self.unordered() & (0b_1000_0000 >> (x + y * 2))) != 0;
    }


    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        assert!(x < 2);
        assert!(y < 4);

        *self = Self::UnOrdered(self.unordered() & (0b_1111_1111 ^ ((!value as u8) << (7 - (x + y * 2)))));
    }

    pub fn set_unchecked(&mut self, x: u8, y: u8, value: bool) {
        *self = Self::UnOrdered(self.unordered() & (0b_1111_1111 ^ ((!value as u8) << (7 - (x + y * 2)))));
    }
}

