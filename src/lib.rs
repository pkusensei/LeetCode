use std::collections::HashMap;

pub fn num_decodings(s: &str) -> i32 {
    solve(s, &mut HashMap::new())
}

fn solve<'a>(s: &'a str, seen: &mut HashMap<&'a str, i32>) -> i32 {
    if let Some(res) = seen.get(s) {
        return *res;
    }

    let res = match s.len() {
        0 => unreachable!(),
        1 => {
            if (b'1'..=b'9').contains(&s.as_bytes()[0]) {
                1
            } else {
                0
            }
        }
        2 => {
            let double = s
                .parse::<u8>()
                .map(|num| if (10..=26).contains(&num) { 1 } else { 0 })
                .unwrap_or(0);
            let singles = solve(&s[..1], seen) * solve(&s[1..], seen);
            double + singles
        }
        _ => {
            let single = solve(&s[..1], seen);
            let double = s[..2]
                .parse::<u8>()
                .map(|num| if (10..=26).contains(&num) { 1 } else { 0 })
                .unwrap_or(0);
            single * solve(&s[1..], seen) + double * solve(&s[2..], seen)
        }
    };
    seen.insert(s, res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(num_decodings("12"), 2);
        debug_assert_eq!(num_decodings("226"), 3);
        debug_assert_eq!(num_decodings("06"), 0);
    }

    #[test]
    fn test() {}
}
