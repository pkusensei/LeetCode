mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_colored_root(edges: Vec<Vec<i32>>, colors: String) -> i32 {
    let (n, s) = (colors.len(), colors.as_bytes());
    let [mut indegs, mut outdegs] = [vec![0; n], vec![0; n]];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        match &[s[a], s[b]] {
            b"RB" | b"BW" | b"WR" => {
                indegs[b] += 1;
                outdegs[a] += 1;
            }
            b"BR" | b"WB" | b"RW" => {
                indegs[a] += 1;
                outdegs[b] += 1;
            }
            _ => return -1,
        }
    }
    let mut root = None;
    for i in 0..n {
        if indegs[i] > 1 || outdegs[i] > 2 {
            return -1;
        }
        if indegs[i] == 0 {
            if root.is_some() {
                return -1;
            }
            root = Some(i)
        }
    }
    root.map(|v| v as i32).unwrap_or(-1)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
