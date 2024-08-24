mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn nearest_palindromic(n: &str) -> String {
    let num: u64 = n.parse().unwrap();
    let a = search_left(num);
    let b = search_right(num);
    if a.abs_diff(num) <= b.abs_diff(num) {
        a.to_string()
    } else {
        b.to_string()
    }
}

fn search_left(num: u64) -> u64 {
    let (mut left, mut right) = (0, num);
    let mut res = 0;
    while left <= right {
        let mid = (right - left) / 2 + left;
        let p = build(mid);
        if p < num {
            res = p;
            left = mid + 1
        } else {
            right = mid - 1;
        }
    }
    res
}

fn search_right(num: u64) -> u64 {
    let (mut left, mut right) = (num, u64::MAX);
    let mut res = 0;
    while left <= right {
        let mid = (right - left) / 2 + left;
        let p = build(mid);
        if p > num {
            res = p;
            right = mid - 1
        } else {
            left = mid + 1
        }
    }
    res
}

fn build(num: u64) -> u64 {
    let mut v: Vec<_> = num.to_string().chars().collect();
    let size = v.len();
    v.truncate((size + 1) / 2);
    let half: Vec<_> = if size & 1 == 1 {
        v[..(size - 1) / 2].iter().copied().rev().collect()
    } else {
        v[..size / 2].iter().copied().rev().collect()
    };
    v.extend(half);
    v.into_iter().collect::<String>().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(nearest_palindromic("123"), "121");
        debug_assert_eq!(nearest_palindromic("1"), "0");
    }

    #[test]
    fn test() {
        debug_assert_eq!(nearest_palindromic("1213"), "1221");
    }
}
