mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_beauty(
    flowers: &[i32],
    mut new_flowers: i64,
    target: i32,
    full: i32,
    partial: i32,
) -> i64 {
    let (mut flowers, completed) =
        flowers
            .iter()
            .fold((vec![], 0_i64), |(mut acc, count), &flo| {
                if flo >= target {
                    (acc, 1 + count)
                } else {
                    acc.push(i64::from(flo));
                    (acc, count)
                }
            });
    let [target, full, partial] = [target, full, partial].map(i64::from);
    let completed = completed * full;
    if flowers.is_empty() {
        return completed;
    }

    let n = flowers.len();
    flowers.sort_unstable_by(|a, b| b.cmp(a));
    let mut left = 0;
    while flowers.get(left).is_some_and(|v| target - v <= new_flowers) {
        new_flowers -= target - flowers[left];
        left += 1;
    }
    if left == n {
        return completed + (n as i64 * full).max((n as i64 - 1) * full + (target - 1) * partial);
    }
    let mut right = n - 1;
    let mut sum = 0;
    let mut res = 0;
    let mut min_flower = flowers[right];
    while min_flower < target {
        while left <= right && flowers[right] <= min_flower {
            sum += flowers[right];
            let Some(v) = right.checked_sub(1) else {
                break;
            };
            right = v;
        }
        let need = (n - right - 1) as i64 * min_flower - sum;
        if need <= new_flowers {
            res = res.max(left as i64 * full + min_flower * partial);
            min_flower += 1;
        } else if let Some(v) = left.checked_sub(1) {
            left = v;
            new_flowers += target - flowers[left];
        } else {
            break;
        }
    }
    res + completed
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
        assert_eq!(maximum_beauty(&[1, 3, 1, 1], 7, 6, 12, 1), 14);
        assert_eq!(maximum_beauty(&[2, 4, 5, 3], 10, 5, 2, 6), 30);
    }

    #[test]
    fn test() {}
}
