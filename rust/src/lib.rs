mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn maximal_square(matrix: &[&[char]]) -> i32 {
    let (row, col) = get_dimensions(matrix);

    let mut dp = vec![vec![0; col]; row];
    for (x, &ch) in matrix[0].iter().enumerate() {
        dp[0][x] = (ch == '1').into();
    }
    for y in 0..row {
        dp[y][0] = (matrix[y][0] == '1').into()
    }

    for (y, r) in matrix.iter().enumerate().skip(1) {
        for (x, &ch) in r.iter().enumerate().skip(1) {
            if ch == '1' {
                let corner = dp[y - 1][x - 1];
                for i in (0..=corner).rev() {
                    if (x - i..=x - 1).all(|xv| matrix[y][xv] == '1')
                        && (y - i..=y - 1).all(|yv| matrix[yv][x] == '1')
                    {
                        dp[y][x] = i + 1;
                        break;
                    }
                }
            }
        }
    }
    let side = dp.into_iter().fold(0, |acc, row| {
        if let Some(v) = row.into_iter().max() {
            acc.max(v)
        } else {
            acc
        }
    });
    side.pow(2) as _
}

fn solve(matrix: &[&[char]]) -> i32 {
    let (row, col) = get_dimensions(matrix);
    let mut dp = vec![vec![0; col + 1]; row + 1];
    let mut res = 0;
    for y in 1..row + 1 {
        for x in 1..col + 1 {
            let curr: i32 = (matrix[y - 1][x - 1] == '1').into();
            dp[y][x] = curr * (dp[y - 1][x - 1].min(dp[y - 1][x]).min(dp[y][x - 1]) + curr);
            res = res.max(dp[y][x])
        }
    }
    res * res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn basics() {
        debug_assert_eq!(
            maximal_square(&[
                &['1', '0', '1', '0', '0'],
                &['1', '0', '1', '1', '1'],
                &['1', '1', '1', '1', '1'],
                &['1', '0', '0', '1', '0']
            ],),
            4
        );
        debug_assert_eq!(maximal_square(&[&['0', '1'], &['1', '0']]), 1);
        debug_assert_eq!(maximal_square(&[&['0']]), 0);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            maximal_square(&[
                &['0', '0', '0', '1'],
                &['1', '1', '0', '1'],
                &['1', '1', '1', '1'],
                &['0', '1', '1', '1'],
                &['0', '1', '1', '1']
            ]),
            9
        )
    }
}
