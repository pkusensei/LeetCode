mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn strange_printer(s: String) -> i32 {
    let mut s = Vec::from(s);
    s.dedup();
    let size = s.len();
    // let mut memo = vec![vec![None; size]; size];
    // min_turns(&s, &mut memo, 0, size - 1)
    let mut dp = vec![vec![0; size]; size];
    for (i, row) in dp.iter_mut().enumerate().take(size) {
        row[i] = 1;
    }
    for length in 2..=size {
        for start in 0..=size - length {
            let end = start + length - 1;
            dp[start][end] = length;
            for split in 0..=length - 2 {
                let mut curr = dp[start][start + split] + dp[start + split + 1][end];
                if s[start + split] == s[end] {
                    curr -= 1
                }
                dp[start][end] = curr.min(dp[start][end]);
            }
        }
    }
    dp[0][size - 1] as i32
}

fn min_turns(s: &[u8], memo: &mut [Vec<Option<i32>>], start: usize, end: usize) -> i32 {
    if start > end {
        return 0;
    }
    if let Some(v) = memo[start][end] {
        return v;
    }
    // worst case: one by one
    let mut curr = 1 + min_turns(s, memo, start + 1, end);
    for i in start + 1..=end {
        if s[i] == s[start] {
            // for "abacd" it splits into "aba" and "cd"
            // "aba" requires the same amount as "ab"
            // "cd" is thrown into another recursion
            let with_match = min_turns(s, memo, start, i - 1) + min_turns(s, memo, i + 1, end);
            curr = curr.min(with_match);
        }
    }
    memo[start][end] = Some(curr);
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(strange_printer("aaabbb".to_string()), 2);
        debug_assert_eq!(strange_printer("aba".to_string()), 2);
    }

    #[test]
    fn test() {}
}
