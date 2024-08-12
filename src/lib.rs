pub fn min_cut(s: &str) -> i32 {
    if is_palindrome(s) {
        return 0;
    }

    let size = s.len();
    let s = s.as_bytes();
    let mut dp: Vec<i32> = (0..size as i32).collect();
    for mid in 0..size {
        let mut left = mid;
        let mut right = mid;
        while right < size && s[left] == s[right] {
            let temp = if left == 0 { 0 } else { dp[left - 1] + 1 };
            dp[right] = dp[right].min(temp);
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }

        left = mid;
        right = mid + 1;
        while right < size && s[left] == s[right] {
            let temp = if left == 0 { 0 } else { dp[left - 1] + 1 };
            dp[right] = dp[right].min(temp);
            if left == 0 {
                break;
            }
            left -= 1;
            right += 1;
        }
    }
    dp.pop().unwrap()
}

// ...[left..right]..
// if [left..right] is palindrome
// to cut at ...right ..
//           ........^..
// it needs 1+cut[left-1]
// if [left..right].. is palindrome
// cut[right] is 0

fn is_palindrome(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }
    s.bytes()
        .rev()
        .zip(s.bytes().take(s.len() / 2 + 1))
        .all(|(b1, b2)| b1 == b2)
}

// type Coord = (usize, usize);

// fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
//     [
//         (a.saturating_sub(1), b),
//         (a + 1, b),
//         (a, b.saturating_sub(1)),
//         (a, b + 1),
//     ]
//     .into_iter()
//     .filter(move |&p| p != (a, b))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_cut("aab"), 1);
        debug_assert_eq!(min_cut("a"), 0);
        debug_assert_eq!(min_cut("ab"), 1);
    }

    #[test]
    fn test() {}
}
