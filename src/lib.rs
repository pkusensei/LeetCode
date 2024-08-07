pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            DIGITS[0].to_string()
        } else {
            let mut res = vec![];
            let mut order = 0;
            while num > 0 {
                let curr = num % 1000;
                if let Some(s) = solve_3(curr, order) {
                    res.push(s);
                }
                num /= 1000;
                order += 1;
            }
            res.reverse();
            res.join(" ")
        }
}

#[rustfmt::skip]
const DIGITS: [&str; 20] = [
    "Zero", "One", "Two", "Three", "Four",
    "Five", "Six", "Seven", "Eight", "Nine",
    "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen",
    "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
];

const TENS: [&str; 9] = [
    "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
];

const ORDERS: [&str; 3] = ["Thousand", "Million", "Billion"];

fn solve_3(num: i32, order: i32) -> Option<String> {
    if num == 0 {
        None
    } else {
        let mut res = {
            let digit = num / 100;
            if digit == 0 {
                String::new()
            } else {
                format!("{} Hundred", DIGITS[digit as usize])
            }
        };
        let double = num % 100;
        if (1..=19).contains(&double) {
            if !res.is_empty() {
                res.push(' ');
            }
            res.push_str(DIGITS[double as usize]);
        } else if double > 0 {
            if !res.is_empty() {
                res.push(' ');
            }
            let tens = double / 10;
            res.push_str(TENS[tens as usize - 1]);
            let single = double % 10;
            if single > 0 {
                res.push(' ');
                res.push_str(DIGITS[single as usize]);
            }
        }
        if order > 0 {
            res.push(' ');
            res.push_str(ORDERS[order as usize - 1]);
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(number_to_words(123), "One Hundred Twenty Three");
        debug_assert_eq!(
            number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five"
        );
        debug_assert_eq!(
            number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
        );
    }

    #[test]
    fn test() {
        debug_assert_eq!(number_to_words(100), "One Hundred");
        debug_assert_eq!(number_to_words(1000), "One Thousand");
    }
}
