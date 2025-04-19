mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_sets(n: i32, max_distance: i32, roads: &[[i32; 3]]) -> i32 {
    let n = n as usize;
    let adj = roads.iter().fold(vec![vec![i32::MAX; n]; n], |mut acc, r| {
        let [a, b] = [0, 1].map(|i| r[i] as usize);
        acc[a][b] = acc[a][b].min(r[2]);
        acc[b][a] = acc[b][a].min(r[2]);
        acc
    });
    let mut res = 0;
    for mask in 0..(1 << n) {
        res += i32::from(check(&adj, max_distance, mask));
    }
    res
}

fn check(adj: &[Vec<i32>], max_distance: i32, mask: i32) -> bool {
    let mut mat = adj.to_vec();
    let n = adj.len();
    for mid in 0..n {
        if (mask >> mid) & 1 == 1 {
            continue;
        }
        for src in 0..n {
            if (mask >> src) & 1 == 1 {
                continue;
            }
            for desc in 0..n {
                if (mask >> desc) & 1 == 1 {
                    continue;
                }
                if mat[src][mid] < i32::MAX && mat[mid][desc] < i32::MAX {
                    mat[src][desc] = mat[src][desc].min(mat[src][mid] + mat[mid][desc]);
                }
            }
        }
    }
    for a in 0..n {
        for b in 0..n {
            if a != b && (mask >> a) & 1 == 0 && (mask >> b) & 1 == 0 && mat[a][b] > max_distance {
                return false;
            }
        }
    }
    true
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
            number_of_sets(3, 5, &[[0, 1, 2], [1, 2, 10], [0, 2, 10]]),
            5
        );
        assert_eq!(
            number_of_sets(3, 5, &[[0, 1, 20], [0, 1, 10], [1, 2, 2], [0, 2, 2]]),
            7
        );
        assert_eq!(number_of_sets(1, 10, &[]), 2);
    }

    #[test]
    fn test() {}
}
