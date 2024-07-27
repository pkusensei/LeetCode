pub fn int_to_roman(num: i32) -> String {
    if !(1..=3999).contains(&num) {
        return String::new();
    }

    [
        thousands,
        |v| encode(v, 100, 'C', 'D', 'M'),
        |v| encode(v, 10, 'X', 'L', 'C'),
        |v| encode(v, 1, 'I', 'V', 'X'),
    ]
    .into_iter()
    .filter_map(|func| func(num))
    .fold(String::new(), |acc, r| format!("{acc}{r}"))
}

fn thousands(num: i32) -> Option<String> {
    match num / 1000 {
        0 => None,
        n @ 1..=3 => Some("M".repeat(n as _)),
        _ => None,
    }
}

fn encode(mut num: i32, order: i32, single: char, fifth: char, tenth: char) -> Option<String> {
    num %= 10 * order;

    match (num / (5 * order), (num % (5 * order)) / order) {
        (1, 4) => Some(format!("{single}{tenth}")),
        (0, 4) => Some(format!("{single}{fifth}")),
        (x @ 0..=1, y @ 0..=3) => {
            let mut s: String = std::iter::repeat(fifth).take(x as _).collect();
            s.extend(std::iter::repeat(single).take(y as _));
            Some(s)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
        debug_assert_eq!(int_to_roman(58), "LVIII");
        debug_assert_eq!(int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test() {}
}
