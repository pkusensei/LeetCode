pub fn minimum_total(triangle: &[&[i32]]) -> i32 {
    let mut res = vec![triangle[0][0]];
    for row in triangle.iter().skip(1) {
        let mut curr = vec![];
        for (idx, v) in row.iter().enumerate() {
            let (n1, n2) = (
                res.get(idx.saturating_sub(1)).copied(),
                res.get(idx).copied(),
            );
            let n = n1.unwrap_or(i32::MAX).min(n2.unwrap_or(i32::MAX));
            curr.push(*v + n);
        }
        res = curr;
    }
    res.into_iter().min().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            minimum_total(&[&[2], &[3, 4], &[6, 5, 7], &[4, 1, 8, 3]]),
            11
        );
        debug_assert_eq!(minimum_total(&[&[-10]]), -10);
    }

    #[test]
    fn test() {}
}
