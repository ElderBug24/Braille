use braille::{BrailleChar, BrailleCharUnOrdered, byte_to_array, array_to_byte, get_bit, get_bit_2d, set_bit, set_bit_2d, MAP_ORDERED_TO_UNORDERED_BITWISE, MAP_UNORDERED_TO_ORDERED_BITWISE};


#[test]
#[allow(non_snake_case)]
fn byte_to_array_() {
    let a = 0b_1111_0000;
    let b = 0b_1010_1011;
    let c = 0b_1110_0011;

    let A = [true, true, true, true, false, false, false, false];
    let B = [true, false, true, false, true, false, true, true];
    let C = [true, true, true, false, false, false, true, true];

    assert_eq!(A, byte_to_array(a));
    assert_eq!(B, byte_to_array(b));
    assert_eq!(C, byte_to_array(c));
}

#[test]
#[allow(non_snake_case)]
fn array_to_byte_() {
    let A = [true, true, true, true, false, false, false, false];
    let B = [true, false, true, false, true, false, true, true];
    let C = [true, true, true, false, false, false, true, true];

    let a = 0b_1111_0000;
    let b = 0b_1010_1011;
    let c = 0b_1110_0011;

    assert_eq!(a, array_to_byte(&A));
    assert_eq!(b, array_to_byte(&B));
    assert_eq!(c, array_to_byte(&C));
}

#[test]
fn get_bit_() {
    let byte = 0b_1101_0010;

    assert_eq!(true, get_bit(byte, 0));
    assert_eq!(true, get_bit(byte, 1));
    assert_eq!(false, get_bit(byte, 2));
    assert_eq!(true, get_bit(byte, 3));
    assert_eq!(false, get_bit(byte, 4));
    assert_eq!(false, get_bit(byte, 5));
    assert_eq!(true, get_bit(byte, 6));
    assert_eq!(false, get_bit(byte, 7));
}

#[test]
#[should_panic]
fn get_bit_panic() {
    get_bit(0u8, 8);
}

#[test]
fn get_bit_2d_() {
    let byte = 0b_1101_0010;

    assert_eq!(true, get_bit_2d(byte, 0, 0));
    assert_eq!(true, get_bit_2d(byte, 1, 0));
    assert_eq!(false, get_bit_2d(byte, 0, 1));
    assert_eq!(true, get_bit_2d(byte, 1, 1));
    assert_eq!(false, get_bit_2d(byte, 0, 2));
    assert_eq!(false, get_bit_2d(byte, 1, 2));
    assert_eq!(true, get_bit_2d(byte, 0, 3));
    assert_eq!(false, get_bit_2d(byte, 1, 3));
}

#[test]
#[should_panic]
fn get_bit_2d_panic() {
    get_bit_2d(0u8, 2, 0);
}

#[test]
#[should_panic]
fn get_bit_2d_panic2() {
    get_bit_2d(0u8, 0, 4);
}

#[test]
fn set_bit_() {
    let mut byte = 0u8;

    byte = set_bit(byte, 0, true);
    assert_eq!(0b_1000_0000, byte);
    byte = set_bit(byte, 1, true);
    assert_eq!(0b_1100_0000, byte);
    byte = set_bit(byte, 3, true);
    assert_eq!(0b_1101_0000, byte);
    byte = set_bit(byte, 6, true);
    assert_eq!(0b_1101_0010, byte);

    byte = set_bit(byte, 2, true);
    assert_eq!(0b_1111_0010, byte);
    byte = set_bit(byte, 4, true);
    assert_eq!(0b_1111_1010, byte);
    byte = set_bit(byte, 5, true);
    assert_eq!(0b_1111_1110, byte);
    byte = set_bit(byte, 7, true);
    assert_eq!(0b_1111_1111, byte);
}

#[test]
#[should_panic]
fn set_bit_panic() {
    set_bit(0u8, 8, false);
}

#[test]
fn set_bit_2d_() {
    let mut byte = 0u8;

    byte = set_bit_2d(byte, 0, 0, true);
    assert_eq!(0b_1000_0000, byte);
    byte = set_bit_2d(byte, 1, 0, true);
    assert_eq!(0b_1100_0000, byte);
    byte = set_bit_2d(byte, 0, 1, true);
    assert_eq!(0b_1110_0000, byte);
    byte = set_bit_2d(byte, 1, 1, true);
    assert_eq!(0b_1111_0000, byte);
    byte = set_bit_2d(byte, 0, 2, true);
    assert_eq!(0b_1111_1000, byte);
    byte = set_bit_2d(byte, 1, 2, true);
    assert_eq!(0b_1111_1100, byte);
    byte = set_bit_2d(byte, 0, 3, true);
    assert_eq!(0b_1111_1110, byte);
    byte = set_bit_2d(byte, 1, 3, true);
    assert_eq!(0b_1111_1111, byte);
}

#[test]
#[should_panic]
fn set_bit_2d_panic() {
    set_bit_2d(0u8, 2, 0, false);
}

#[test]
#[should_panic]
fn set_bit_2d_panic2() {
    set_bit_2d(0u8, 0, 4, false);
}

#[test]
fn masks() {
    let byte = 0b_1101_0010;
    let arr = [true, true, false, true, false, false, true, false];

    let char = BrailleChar::from_unordered(byte);

    for i in 0..8 {
        let index = MAP_UNORDERED_TO_ORDERED_BITWISE[i];
        assert_eq!(arr[i], char.get_at(index));
    }

    let char = BrailleCharUnOrdered::from_ordered(byte);

    for i in 0..8 {
        let index = MAP_ORDERED_TO_UNORDERED_BITWISE[i];
        assert_eq!(arr[i], char.get_at(index));
    }
}

