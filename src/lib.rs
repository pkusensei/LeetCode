use std::collections::HashMap;

pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut seen = HashMap::new();
    (0..num_rows)
        .map(|n| (0..=n).map(|k| n_choose_k(n, k, &mut seen)).collect())
        .collect()
}

fn n_choose_k(n: i32, k: i32, seen: &mut HashMap<(i32, i32), i32>) -> i32 {
    if k == 0 || k == n {
        1
    } else {
        if let Some(r) = seen.get(&(n, k)) {
            return *r;
        }
        let r = n_choose_k(n - 1, k - 1, seen) + n_choose_k(n - 1, k, seen);
        seen.insert((n, k), r);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            generate(5),
            &[
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
        debug_assert_eq!(generate(1), [[1]]);
    }

    #[test]
    fn test() {}
}
