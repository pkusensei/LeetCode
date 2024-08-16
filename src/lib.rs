mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
    if numerator == 0 {
        return "0".to_string();
    }

    let mut res = if numerator.is_negative() != denominator.is_negative() {
        "-".to_string()
    } else {
        String::new()
    };
    let (numerator, denominator) = ((numerator as i64).abs(), (denominator as i64).abs());
    let int = numerator / denominator;
    res = format!("{res}{int}");
    let mut remainder = numerator % denominator;
    if remainder == 0 {
        return res;
    }
    res.push('.');
    let mut fracts = HashMap::new();
    while remainder > 0 {
        if let Some(&i) = fracts.get(&remainder) {
            res.insert(i, '(');
            res.push(')');
            break;
        }
        fracts.insert(remainder, res.len());
        remainder *= 10;
        res = format!("{res}{}", remainder / denominator);
        remainder %= denominator;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(fraction_to_decimal(1, 2), "0.5");
        debug_assert_eq!(fraction_to_decimal(2, 1), "2");
        debug_assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }

    #[test]
    fn test() {
        debug_assert_eq!(fraction_to_decimal(7, -12), "-0.58(3)");
        debug_assert_eq!(
            fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125"
        );
        debug_assert_eq!(fraction_to_decimal(-2147483648, 1), "-2147483648");
    }
}
