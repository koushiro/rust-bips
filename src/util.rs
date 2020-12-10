/// A helper function for returning the left `idx` bit of a byte.
pub(crate) const fn left_bit_mask(idx: usize) -> u8 {
    1 << (7 - (idx % 8))
}

/// A helper function for converting 11 bits to integer.
pub(crate) fn bits11_to_index(bits: &[bool]) -> usize {
    bits.iter()
        .enumerate()
        .map(|(i, bit)| if *bit { 1 << (10 - i) } else { 0 })
        .sum::<usize>()
}

#[test]
#[allow(clippy::identity_op)]
fn test_bit_mask() {
    assert_eq!(0b1111_1111 & left_bit_mask(0), 0b1000_0000);
    assert_eq!(0b1111_1111 & left_bit_mask(3), 0b0001_0000);
    assert_eq!(0b1111_1111 & left_bit_mask(7), 0b0000_0001);
    assert_eq!(0b1111_1111 & left_bit_mask(8), 0b1000_0000);
    assert_eq!(0b1000_0000 & left_bit_mask(0), 0b1000_0000);
    assert_eq!(0b0100_0000 & left_bit_mask(0), 0b0000_0000);
}

#[test]
fn test_bits11_to_index() {
    assert_eq!(bits11_to_index(&[false; 11]), 0b000_0000_0000); // 0
    assert_eq!(bits11_to_index(&[true; 11]), 0b111_1111_1111); // 2047
    let mut bits = [false; 11];
    bits[0] = true;
    bits[1] = true;
    bits[2] = true;
    bits[3] = true;
    bits[4] = true;
    assert_eq!(bits11_to_index(&bits), 0b111_1100_0000); //1984
}
