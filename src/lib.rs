pub fn my_pow(x: f64, n: i32) -> f64 {
    solve(x, n.into())
}

fn solve(x: f64, n: i64) -> f64 {
    match (x, n) {
        (_, 0) | (1.0, _) => 1.0,
        (-1.0, _) => {
            if n & 1 == 0 {
                1.0
            } else {
                -1.0
            }
        }
        (0.0, _) => 0.0,
        _ => {
            if n < 0 {
                solve(1.0 / x, -n)
            } else if n & 1 == 0 {
                solve(x * x, n >> 1)
            } else {
                x * solve(x * x, n >> 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(my_pow(2.0000, 10), 1024.0000);
        // debug_assert_eq!(my_pow(2.1000, 3), 9.26100);
        debug_assert_eq!(my_pow(2.00000, -2), 0.25000);
    }

    #[test]
    fn test() {}
}
