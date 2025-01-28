mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn get_max_grid_happiness(m: i32, n: i32, intro_count: i32, extro_count: i32) -> i32 {
    dfs(m, n, 0, intro_count, extro_count, 0, 0, &mut HashMap::new())
}

fn dfs(
    m: i32,
    n: i32,
    col: i32,
    intro: i32,
    extro: i32,
    prev_row: i32,
    curr_row: i32,
    memo: &mut HashMap<MKey, i32>,
) -> i32 {
    if m == 0 {
        return 0;
    }
    if col == n {
        return dfs(m - 1, n, 0, intro, extro, curr_row, 0, memo);
    }
    let key = MKey {
        m,
        col,
        intro,
        extro,
        prev_row,
        curr_row,
    };
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    // Add empty cell
    let mut res = dfs(m, n, 1 + col, intro, extro, prev_row, curr_row << 2, memo);
    if intro > 0 {
        let mut score = 120;
        match State::get_at(prev_row, n, col) {
            State::Empty => (),
            State::Intro => score -= 2 * 30,
            State::Extro => score += 20 - 30,
        }
        match State::from_bits(curr_row & 0b11) {
            State::Empty => (),
            State::Intro => score -= 2 * 30,
            State::Extro => score += 20 - 30,
        }
        score += dfs(
            m,
            n,
            1 + col,
            intro - 1,
            extro,
            prev_row,
            (curr_row << 2) | State::Intro.to_bits(),
            memo,
        );
        res = res.max(score);
    }
    if extro > 0 {
        let mut score = 40;
        match State::get_at(prev_row, n, col) {
            State::Empty => (),
            State::Intro => score += 20 - 30,
            State::Extro => score += 2 * 20,
        }
        match State::from_bits(curr_row & 0b11) {
            State::Empty => (),
            State::Intro => score += 20 - 30,
            State::Extro => score += 2 * 20,
        }
        score += dfs(
            m,
            n,
            1 + col,
            intro,
            extro - 1,
            prev_row,
            (curr_row << 2) | State::Extro.to_bits(),
            memo,
        );
        res = res.max(score);
    }
    memo.insert(key, res);
    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MKey {
    m: i32,
    col: i32,
    intro: i32,
    extro: i32,
    prev_row: i32,
    curr_row: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Intro,
    Extro,
}

impl State {
    const fn to_bits(self) -> i32 {
        match self {
            State::Empty => 0,
            State::Intro => 0b01,
            State::Extro => 0b10,
        }
    }

    const fn from_bits(n: i32) -> Self {
        match n {
            0 => Self::Empty,
            0b01 => Self::Intro,
            0b10 => Self::Extro,
            _ => unreachable!(),
        }
    }

    const fn get_at(num: i32, n: i32, col: i32) -> Self {
        Self::from_bits((num >> (2 * (n - col - 1))) & 0b11)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(get_max_grid_happiness(2, 3, 1, 2), 240);
        assert_eq!(get_max_grid_happiness(3, 1, 2, 1), 260);
        assert_eq!(get_max_grid_happiness(2, 2, 4, 0), 240);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
