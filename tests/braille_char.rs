use braille::BrailleChar;


const BYTE_A_ORDERED: u8 = 0b_1100_1111;
const BYTE_A_UNORDERED: u8 = 0b_1110_1011;
const CHAR_A: char = '⣏';
const CHAR_A_U32: u32 = 0x28CF;
const BYTE_B_ORDERED: u8 = 183;

#[test]
fn from_u8_ordered() {
    let char = BrailleChar(BYTE_A_ORDERED);

    assert_eq!(CHAR_A, char.char());
}

#[test]
fn from_u8_unordered() {
    let char = BrailleChar::from_unordered(BYTE_A_UNORDERED);

    assert_eq!(CHAR_A, char.char());
}

#[test]
fn to_u8_ordered() {
    let char = BrailleChar::from_unordered(BYTE_A_UNORDERED);

    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn to_u8_unordered() {
    let char = BrailleChar(BYTE_A_ORDERED);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
}

#[test]
fn ordered_unordered() {
    let char = BrailleChar(BYTE_A_ORDERED);
    let char2 = BrailleChar::from_unordered(char.unordered());

    assert_eq!(BYTE_A_ORDERED, char2.ordered());
}

#[test]
fn unordered_ordered() {
    let char = BrailleChar::from_unordered(BYTE_A_UNORDERED);
    let char2 = BrailleChar(char.ordered());

    assert_eq!(BYTE_A_UNORDERED, char2.unordered());
}

#[test]
fn from_char() {
    let char = BrailleChar::from_char(CHAR_A);

    assert_eq!(Some(BrailleChar(BYTE_A_ORDERED)), char);

    let char = char.unwrap();

    assert_eq!(CHAR_A, char.char());
    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn from_char_u32() {
    let char = BrailleChar::from_u32_char(CHAR_A_U32).unwrap();

    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
#[should_panic]
fn from_char_u32_panic() {
    let _char = BrailleChar::from_u32_char(0xFF69).unwrap();
}

#[test]
fn get() {
    let char = BrailleChar(BYTE_A_ORDERED);

    assert_eq!(true, char.get(0, 0));
    assert_eq!(true, char.get(1, 0));
    assert_eq!(true, char.get(0, 1));
    assert_eq!(false, char.get(1, 1));
    assert_eq!(true, char.get(0, 2));
    assert_eq!(false, char.get(1, 2));
    assert_eq!(true, char.get(0, 3));
    assert_eq!(true, char.get(1, 3));
}

#[test]
fn set() {
    let mut char = BrailleChar::from_unordered(BYTE_A_UNORDERED);

    char.set(1, 0, false);
    char.set(1, 1, true);
    char.set(1, 2, true);
    char.set(0, 3, false);

    assert_eq!(BYTE_B_ORDERED, char.ordered());
}

#[test]
#[should_panic]
fn get_panic() {
    let char = BrailleChar(BYTE_A_ORDERED);

    char.get(2, 0);
}

#[test]
#[should_panic]
fn get_panic_2() {
    let char = BrailleChar(BYTE_A_ORDERED);

    char.get(0, 4);
}

#[test]
#[should_panic]
fn set_panic() {
    let mut char = BrailleChar(BYTE_A_ORDERED);

    char.set(2, 0, false);
}

#[test]
#[should_panic]
fn set_panic_2() {
    let mut char = BrailleChar(BYTE_A_ORDERED);

    char.set(0, 4, false);
}

