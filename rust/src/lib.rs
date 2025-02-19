mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_farmland(mut land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(&land);
    let mut res = vec![];
    for r in 0..rows {
        for c in 0..cols {
            if land[r][c] == 1 {
                let mut queue = std::collections::VecDeque::from([[r, c]]);
                land[r][c] = 0;
                let [mut r1, mut c1] = [i32::MAX; 2];
                let [mut r2, mut c2] = [i32::MIN; 2];
                while let Some([r, c]) = queue.pop_front() {
                    r1 = r1.min(r as i32);
                    c1 = c1.min(c as i32);
                    r2 = r2.max(r as i32);
                    c2 = c2.max(c as i32);
                    for [nr, nc] in neighbors([r, c]) {
                        if nr < rows && nc < cols && land[nr][nc] == 1 {
                            land[nr][nc] = 0;
                            queue.push_back([nr, nc]);
                        }
                    }
                }
                res.push(vec![r1, c1, r2, c2]);
            }
        }
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
    fn basics() {}

    #[test]
    fn test() {}
}
