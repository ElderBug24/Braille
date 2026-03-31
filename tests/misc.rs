use braille::{byte_to_array, array_to_byte};


#[test]
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

