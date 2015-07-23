use bit_vec::BitVec;

fn reverse_bits(value: u8) -> u8 {
  let mut reversed_value = ((value &  0xf0) >> 4) | ((value & 0x0f) << 4);
  reversed_value = (reversed_value & 0xCC) >> 2 | (reversed_value & 0x33) << 2;
  reversed_value = (reversed_value & 0xAA) >> 1 | (reversed_value & 0x55) << 1;
  reversed_value
}

pub fn from_u8(value: u8) -> BitVec {
  BitVec::from_bytes(&[reverse_bits(value)])
}

pub fn to_u8(avr_bit_vec: BitVec) -> u8 {
  let reversed_value = avr_bit_vec.to_bytes()[0];
  reverse_bits(reversed_value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_convert_from_and_to() {
      let abv = from_u8(0b00000001);
      assert_eq!(format!("{:?}", abv), "10000000");
      let value = to_u8(abv);
      assert_eq!(value, 0b00000001);

      let abv = from_u8(0b00001100);
      assert_eq!(format!("{:?}", abv), "00110000");
      let value = to_u8(abv);
      assert_eq!(value, 0b00001100);

      let abv = from_u8(0b11110000);
      assert_eq!(format!("{:?}", abv), "00001111");

      let value = to_u8(abv);
      assert_eq!(value, 0b11110000);
  }
}
