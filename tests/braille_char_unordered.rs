use braille::BrailleCharUnOrdered;


const BYTE_A_ORDERED: u8 = 0b_1100_1111;
const BYTE_A_UNORDERED: u8 = 0b_1110_1011;
const CHAR_A: char = '⣏';
const CHAR_A_U32: u32 = 0x28CF;
const BYTE_B_ORDERED: u8 = 183;

#[test]
fn from_u8_ordered() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    assert_eq!(CHAR_A, char.char());
}

#[test]
fn from_u8_unordered() {
    let char = BrailleCharUnOrdered::from_unordered(BYTE_A_UNORDERED);

    assert_eq!(CHAR_A, char.char());
}

#[test]
fn to_u8_ordered() {
    let char = BrailleCharUnOrdered::from_unordered(BYTE_A_UNORDERED);

    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn to_u8_unordered() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
}

#[test]
fn ordered_unordered() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);
    let char2 = BrailleCharUnOrdered::from_unordered(char.unordered());

    assert_eq!(BYTE_A_ORDERED, char2.ordered());
}

#[test]
fn unordered_ordered() {
    let char = BrailleCharUnOrdered::from_unordered(BYTE_A_UNORDERED);
    let char2 = BrailleCharUnOrdered::from_ordered(char.ordered());

    assert_eq!(BYTE_A_UNORDERED, char2.unordered());
}

#[test]
fn from_array_ordered() {
    let array = [true, true, false, false, true, true, true, true];

    let char = BrailleCharUnOrdered::from_array_ordered(&array);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn from_array_unordered() {
    let array = [true, true, true, false, true, false, true, true];

    let char = BrailleCharUnOrdered::from_array_unordered(&array);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn from_slice_ordered() {
    let array = [true, true, false, false, true, true, true, true];

    let char = BrailleCharUnOrdered::from_slice_ordered(&array);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn from_slice_unordered() {
    let array = [true, true, true, false, true, false, true, true];

    let char = BrailleCharUnOrdered::from_slice_unordered(&array);

    assert_eq!(BYTE_A_UNORDERED, char.unordered());
    assert_eq!(BYTE_A_ORDERED, char.ordered());

    let array = [true, true, true, false, true];

    let char = BrailleCharUnOrdered::from_slice_unordered(&array);

    assert_eq!(BYTE_A_UNORDERED & 0b_1111_1100, char.unordered());
    assert_eq!(BYTE_A_ORDERED & 0b_0001_1111, char.ordered());
}

#[test]
fn from_char() {
    let char = BrailleCharUnOrdered::from_char(CHAR_A);

    assert_eq!(Some(BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED)), char);

    let char = char.unwrap();

    assert_eq!(CHAR_A, char.char());
    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
fn from_char_u32() {
    let char = BrailleCharUnOrdered::from_u32_char(CHAR_A_U32).unwrap();

    assert_eq!(BYTE_A_ORDERED, char.ordered());
}

#[test]
#[should_panic]
fn from_char_u32_panic() {
    let _char = BrailleCharUnOrdered::from_u32_char(0xFF69).unwrap();
}

#[test]
fn get() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    assert_eq!(true, char.get_at_xy(0, 0));
    assert_eq!(true, char.get_at_xy(1, 0));
    assert_eq!(true, char.get_at_xy(0, 1));
    assert_eq!(false, char.get_at_xy(1, 1));
    assert_eq!(true, char.get_at_xy(0, 2));
    assert_eq!(false, char.get_at_xy(1, 2));
    assert_eq!(true, char.get_at_xy(0, 3));
    assert_eq!(true, char.get_at_xy(1, 3));
}

#[test]
fn set() {
    let mut char = BrailleCharUnOrdered::from_unordered(BYTE_A_UNORDERED);

    char.set_at_xy(1, 0, false);
    char.set_at_xy(1, 1, true);
    char.set_at_xy(1, 2, true);
    char.set_at_xy(0, 3, false);

    assert_eq!(BYTE_B_ORDERED, char.ordered());
}

#[test]
#[should_panic]
fn get_panic() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    char.get_at_xy(2, 0);
}

#[test]
#[should_panic]
fn get_panic_2() {
    let char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    char.get_at_xy(0, 4);
}

#[test]
#[should_panic]
fn set_panic() {
    let mut char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    char.set_at_xy(2, 0, false);
}

#[test]
#[should_panic]
fn set_panic_2() {
    let mut char = BrailleCharUnOrdered::from_ordered(BYTE_A_ORDERED);

    char.set_at_xy(0, 4, false);
}

