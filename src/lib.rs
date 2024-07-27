pub fn roman_to_int(s: &str) -> i32 {
    let mut iter = s.chars().rev().peekable();
    let mut res = 0;
    while let Some(ch) = iter.next() {
        res += decode(ch);
        match ch {
            'V' | 'X' if iter.peek().is_some_and(|c| *c == 'I') => {
                res -= decode('I');
                iter.next();
            }

            'L' | 'C' if iter.peek().is_some_and(|c| *c == 'X') => {
                res -= decode('X');
                iter.next();
            }

            'D' | 'M' if iter.peek().is_some_and(|c| *c == 'C') => {
                res -= decode('C');
                iter.next();
            }
            _ => (),
        }
    }
    res
}

const fn decode(ch: char) -> i32 {
    match ch {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(roman_to_int("III"), 3);
        debug_assert_eq!(roman_to_int("LVIII"), 58);
        debug_assert_eq!(roman_to_int("MCMXCIV"), 1994);
    }

    #[test]
    fn test() {}
}
