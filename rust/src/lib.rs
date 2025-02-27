mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_even_numbers(digits: &[i32]) -> Vec<i32> {
    let freq = digits.iter().fold([0; 10], |mut acc, &d| {
        acc[d as usize] += 1;
        acc
    });
    let mut res = vec![];
    'outer: for num in (100..999).step_by(2) {
        let mut x = num;
        let mut f = freq.clone();
        while x > 0 {
            let d = (x % 10) as usize;
            f[d] -= 1;
            if f[d] < 0 {
                continue 'outer;
            }
            x /= 10;
        }
        res.push(num);
    }
    res
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {
        assert_eq!(
            find_even_numbers(&[2, 1, 3, 0]),
            [102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
    }

    #[test]
    fn test() {}
}
