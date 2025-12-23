use grains::{square, total, Error};

#[test]
fn square_1() {
    assert_eq!(square(1), Ok(1));
}

#[test]
#[ignore]
fn square_2() {
    assert_eq!(square(2), Ok(2));
}

#[test]
#[ignore]
fn square_3() {
    assert_eq!(square(3), Ok(4));
}

#[test]
#[ignore]
fn square_64() {
    assert_eq!(square(64), Ok(9_223_372_036_854_775_808));
}

#[test]
#[ignore]
fn total_grains() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
}

#[test]
#[ignore]
fn invalid_square_0() {
    assert_eq!(square(0), Err(Error::InvalidSquare));
}

#[test]
#[ignore]
fn invalid_square_65() {
    assert_eq!(square(65), Err(Error::InvalidSquare));
}

