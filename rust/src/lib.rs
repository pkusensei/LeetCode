mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn average_waiting_time(customers: &[[i32; 2]]) -> f64 {
    let mut chef = 0;
    let mut wait = 0;
    for c in customers.iter() {
        let [a, t] = [0, 1].map(|i| i64::from(c[i]));
        if a >= chef {
            chef = a + t;
            wait += t
        } else {
            chef += t;
            wait += chef - a;
        }
    }
    wait as f64 / customers.len() as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        float_eq!(average_waiting_time(&[[1, 2], [2, 5], [4, 3]]), 5.0);
        float_eq!(
            average_waiting_time(&[[5, 2], [5, 4], [10, 3], [20, 1]]),
            3.25
        );
    }

    #[test]
    fn test() {}
}
