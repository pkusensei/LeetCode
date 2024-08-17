mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_bits(mut x: u32) -> u32 {
    let mut res = 0;
    for _ in 0..32 {
        res <<= 1;
        res += x & 1;
        x >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            reverse_bits(0b00000010100101000001111010011100),
            0b00111001011110000010100101000000
        );
        debug_assert_eq!(
            reverse_bits(0b11111111111111111111111111111101),
            0b10111111111111111111111111111111
        );
    }

    #[test]
    fn test() {}
}
