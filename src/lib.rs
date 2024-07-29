pub fn count_and_say(n: i32) -> String {
    let start = "1".to_string();
    if n < 2 {
        start
    } else {
        let mut res = start;
        for _ in 0..n - 1 {
            let pairs = str_to_pairs(&res);
            res = pairs_to_str(&pairs);
        }
        res
    }
}

fn str_to_pairs(s: &str) -> Vec<(char, i32)> {
    let mut curr = '0';
    let mut count = 0;
    let mut res = vec![];
    for ch in s.chars() {
        if curr == ch {
            count += 1
        } else if curr != ch && count > 0 {
            res.push((curr, count));
            curr = ch;
            count = 1;
        } else {
            curr = ch;
            count = 1
        }
    }
    res.push((curr, count));
    res
}

fn pairs_to_str(pairs: &[(char, i32)]) -> String {
    pairs
        .iter()
        .fold(String::new(), |acc, p| format!("{}{}{}", acc, p.1, p.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_and_say(4), "1211");
        debug_assert_eq!(count_and_say(1), "1");
    }

    #[test]
    fn test() {}
}
