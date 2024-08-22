mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn find_complement(num: i32) -> i32 {
    let p = num.ilog2();
    let mask = 2i32.pow(p + 1) - 1;
    num ^ mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_complement(5), 2);
        debug_assert_eq!(find_complement(1), 0);
    }

    #[test]
    fn test() {}
}
