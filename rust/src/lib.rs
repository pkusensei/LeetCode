mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn die_simulator(n: i32, roll_max: [i32; 6]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut dp = Vec::with_capacity(n);
    dp.push([1i64; 6]);
    for dice in 1..n {
        dp.push([0; 6]);
        for face in 0..6 {
            // Without constraints
            // current dice could attach all face to previous one
            // i.e whichever previous face is, this face could be picked
            let mut curr: i64 = dp[dice - 1].iter().sum();
            match (roll_max[face] as usize).cmp(&dice) {
                std::cmp::Ordering::Greater => (),
                // roll_max[face] == current_dice => this face cannot be used
                std::cmp::Ordering::Equal => curr -= 1,
                std::cmp::Ordering::Less => {
                    // For all face i!= current_face
                    // Consider the situation
                    // that previous roll_max[face] dices all rolled current face
                    // dp[dice -1 -max[face]][i] must be eliminated
                    for i in (0..6).filter(|&i| i != face) {
                        curr -= dp[dice - 1 - roll_max[face] as usize][i];
                    }
                }
            }
            dp[dice][face] = curr.rem_euclid(MOD);
        }
    }
    dp[n - 1].iter().fold(0, |acc, v| (acc + v) % MOD) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(die_simulator(2, [1, 1, 2, 2, 2, 3]), 34);
        assert_eq!(die_simulator(2, [1, 1, 1, 1, 1, 1]), 30);
        assert_eq!(die_simulator(3, [1, 1, 1, 2, 2, 3]), 181);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
