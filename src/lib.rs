pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    if k == 1 {
        (1..=n).map(|n| vec![n]).collect()
    } else if k < n {
        // use n
        combine(n - 1, k - 1)
            .into_iter()
            .map(|mut v| {
                v.push(n);
                v
            })
            .chain(combine(n - 1, k)) // skip n
            .collect()
    } else {
        // k == n
        vec![(1..=n).collect()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     combine(4, 2),
        //     [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
        // );
        debug_assert_eq!(combine(1, 1), [[1]])
    }

    #[test]
    fn test() {}
}
