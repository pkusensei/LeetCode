pub fn get_permutation(n: i32, k: i32) -> String {
    let digits: Vec<_> = (1..=n)
        .map(|n| char::from_digit(n as _, 10).unwrap())
        .collect();
    solve(digits, n, k - 1) // 0 indexed
}

fn solve(mut digits: Vec<char>, n: i32, k: i32) -> String {
    if n == 1 {
        return digits[0].to_string();
    }
    let group = factorial(n - 1);
    let idx = k / group;
    let ch = digits.remove(idx as _);
    format!("{}{}", ch, solve(digits, n - 1, k % group))
}

const fn factorial(n: i32) -> i32 {
    if n < 2 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_permutation(3, 3), "213");
        debug_assert_eq!(get_permutation(4, 9), "2314");
        debug_assert_eq!(get_permutation(3, 1), "123");
    }

    #[test]
    fn test() {}
}
