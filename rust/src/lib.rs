mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_num_of_substrings(s: String) -> Vec<String> {
    let [mut first, mut last] = [[None; 26]; 2];
    for (i, b) in s.bytes().enumerate() {
        let bi = usize::from(b - b'a');
        first[bi].get_or_insert(i);
        last[bi] = Some(i);
    }
    let mut st: Vec<[usize; 2]> = vec![];
    for (idx, b) in s.bytes().enumerate() {
        let bi = usize::from(b - b'a');
        if first[bi] != Some(idx) {
            continue;
        }
        let Some(right) = expand(&first, &last, s.as_bytes(), idx, last[bi].unwrap()) else {
            continue;
        };
        while let Some(&[a, b]) = st.last()
            && (a < idx && right < b)
        {
            st.pop();
        }
        st.push([idx, right]);
    }
    st.iter().map(|&[a, b]| s[a..=b].to_owned()).collect()
}

fn expand(
    first: &[Option<usize>; 26],
    last: &[Option<usize>; 26],
    s: &[u8],
    left: usize,
    mut right: usize,
) -> Option<usize> {
    let mut idx = 1 + left;
    while idx < right {
        let bi = usize::from(s[idx] - b'a');
        if first[bi].is_some_and(|v| v < left) {
            return None; // This letter starts before `left
        }
        right = right.max(last[bi].unwrap());
        idx += 1;
    }
    Some(right)
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
