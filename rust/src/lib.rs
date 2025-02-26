mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    if rows == 1 {
        return encoded_text;
    }
    let rows = rows as usize;
    let cols = encoded_text.len() / rows;
    let mut res = vec![];
    let [mut r, mut c] = [0, 0];
    let mut prev_col = 0;
    while r < rows && c < cols {
        res.push(encoded_text.as_bytes()[r * cols + c]);
        r += 1;
        c += 1;
        if r == rows|| c == cols {
            r = 0;
            c = 1 + prev_col;
            prev_col += 1;
        }
    }
    while res.last().is_some_and(|b| b.is_ascii_whitespace()) {
        res.pop();
    }
    String::from_utf8(res).unwrap()
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
