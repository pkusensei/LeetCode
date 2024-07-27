pub fn letter_combinations(digits: &str) -> Vec<String> {
    let digits: Vec<_> = digits.chars().rev().collect();
    solve(&digits)
}

fn solve(digits: &[char]) -> Vec<String> {
    match digits.first() {
        None => vec![],
        Some(&digit) => {
            let right = solve(&digits[1..]);
            if right.is_empty() {
                encode(digit).map(|c| c.to_string()).collect()
            } else {
                encode(digit)
                    .flat_map(|ch| right.iter().map(move |s| format!("{s}{ch}")))
                    .collect()
            }
        }
    }
}

fn encode(digit: char) -> impl Iterator<Item = char> {
    match digit {
        '2' => "abc".chars(),
        '3' => "def".chars(),
        '4' => "ghi".chars(),
        '5' => "jkl".chars(),
        '6' => "mno".chars(),
        '7' => "pqrs".chars(),
        '8' => "tuv".chars(),
        '9' => "wxyz".chars(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basics() {
        {
            let mut res = letter_combinations("23");
            res.sort_unstable();
            debug_assert_eq!(res, ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        }
        {
            debug_assert_eq!(letter_combinations(""), Vec::<String>::new());
        }
        {
            let mut res = letter_combinations("2");
            res.sort_unstable();
            debug_assert_eq!(res, ["a", "b", "c"]);
        }
    }

    #[test]
    fn test() {}
}
