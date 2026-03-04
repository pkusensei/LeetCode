mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    const M: i64 = 1_000_000_007;
    let mut st = Vec::<(usize, i64)>::new();
    // sum(subarr_min) at `i`
    let mut dp = vec![0; arr.len()];
    for (idx, &num) in arr.iter().enumerate() {
        let num = i64::from(num);
        while let Some(top) = st.last()
            && top.1 > num
        {
            st.pop();
        }
        if let Some(top) = st.last() {
            // For all subarr ending on `idx`, i.e left..=idx
            // Since [top]<=[idx]
            // if left<=top.0, min(subarr) = [top], stored in dp[top]
            // if top.0<left, min(subarr)=[idx], sum = (idx-top.0)*[idx]
            // Thus all subarrs ending on `idx` is
            let curr = (idx - top.0) as i64 * num % M;
            dp[idx] = (curr + dp[top.0]) % M;
        } else {
            // For all 0..=idx, min(subarr) = [idx]
            dp[idx] = (1 + idx) as i64 * num % M;
        }
        st.push((idx, num));
    }
    dp.iter().fold(0, |acc, v| (acc + v) % M) as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
