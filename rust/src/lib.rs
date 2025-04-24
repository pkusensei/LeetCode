mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn result_grid(image: &[&[i32]], threshold: i32) -> Vec<Vec<i32>> {
    let [rows, cols] = get_dimensions(image);
    let [mut sum, mut freq] = [0, 1].map(|_| vec![vec![0; cols]; rows]);
    for r in 0..=rows - 3 {
        for c in 0..=cols - 3 {
            let Some(v) = sum_grid(image, r, c, threshold as u32) else {
                continue;
            };
            for row in r..r + 3 {
                for col in c..c + 3 {
                    sum[row][col] += v / 9;
                    freq[row][col] += 1;
                }
            }
        }
    }
    let mut res = vec![vec![0; cols]; rows];
    for (r, row) in res.iter_mut().enumerate() {
        for (c, v) in row.iter_mut().enumerate() {
            if freq[r][c] == 0 {
                *v = image[r][c];
            } else {
                *v = sum[r][c] / freq[r][c];
            }
        }
    }
    res
}

fn sum_grid(image: &[&[i32]], up: usize, left: usize, thr: u32) -> Option<i32> {
    let mut sum = 0;
    for r in up..up + 3 {
        for c in left..left + 3 {
            if r > up && image[r - 1][c].abs_diff(image[r][c]) > thr {
                return None;
            }
            if c > left && image[r][c - 1].abs_diff(image[r][c]) > thr {
                return None;
            }
            sum += image[r][c];
        }
    }
    Some(sum)
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
            result_grid(&[&[5, 6, 7, 10], &[8, 9, 10, 10], &[11, 12, 13, 10]], 3),
            [[9, 9, 9, 9], [9, 9, 9, 9], [9, 9, 9, 9]]
        );
        assert_eq!(
            result_grid(
                &[&[10, 20, 30], &[15, 25, 35], &[20, 30, 40], &[25, 35, 45]],
                12
            ),
            [[25, 25, 25], [27, 27, 27], [27, 27, 27], [30, 30, 30]]
        );
        assert_eq!(
            result_grid(&[&[5, 6, 7], &[8, 9, 10], &[11, 12, 13]], 1),
            [[5, 6, 7], [8, 9, 10], [11, 12, 13]]
        );
    }

    #[test]
    fn test() {}
}
