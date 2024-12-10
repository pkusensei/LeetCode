mod dsu;
mod helper;
mod trie;

use std::{
    collections::{HashSet, VecDeque},
    iter,
};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_moves(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let start = State::default();
    let mut queue = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::from([start]);
    while let Some((curr, dist)) = queue.pop_front() {
        if curr.y == n - 1 && curr.x == n - 2 {
            return dist;
        }
        for node in curr.next(grid) {
            if seen.insert(node) {
                queue.push_back((node, 1 + dist));
            }
        }
    }
    -1
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: u8,
}

impl State {
    pub fn next(self, grid: &[&[i32]]) -> impl Iterator<Item = Self> {
        iter::once(self.walk(grid))
            .flatten()
            .chain(iter::once(self.rotate(grid)))
            .flatten()
    }

    fn walk<T: AsRef<[i32]>>(self, grid: &[T]) -> [Option<Self>; 2] {
        let mut res = [None; 2];
        match self.dir {
            0 => {
                if grid[self.y]
                    .as_ref()
                    .get(2 + self.x)
                    .is_some_and(|&v| v == 0)
                {
                    res[0] = Some(Self {
                        x: 1 + self.x,
                        ..self
                    });
                }
                if grid.get(1 + self.y).is_some_and(|r| {
                    let r = r.as_ref();
                    r[self.x] == 0 && r[1 + self.x] == 0
                }) {
                    res[1] = Some(Self {
                        y: 1 + self.y,
                        ..self
                    })
                }
            }
            1 => {
                if grid
                    .get(2 + self.y)
                    .is_some_and(|r| r.as_ref()[self.x] == 0)
                {
                    res[0] = Some(Self {
                        y: 1 + self.y,
                        ..self
                    });
                }
                if grid[self.y]
                    .as_ref()
                    .get(1 + self.x)
                    .is_some_and(|&v| v == 0)
                    && grid
                        .get(1 + self.y)
                        .is_some_and(|r| r.as_ref().get(1 + self.x).is_some_and(|&v| v == 0))
                {
                    res[1] = Some(Self {
                        x: 1 + self.x,
                        ..self
                    })
                }
            }
            _ => (),
        }
        res
    }

    fn rotate<T: AsRef<[i32]>>(self, grid: &[T]) -> Option<Self> {
        match self.dir {
            0 if grid.get(1 + self.y).is_some_and(|r| {
                let row = r.as_ref();
                row[self.x] == 0 && row.get(1 + self.x).is_some_and(|&v| v == 0)
            }) =>
            {
                Some(Self {
                    dir: 1 - self.dir,
                    ..self
                })
            }
            1 if 1 + self.x < grid[0].as_ref().len()
                && grid[self.y].as_ref()[1 + self.x] == 0
                && grid[1 + self.y].as_ref()[1 + self.x] == 0 =>
            {
                Some(Self {
                    dir: 1 - self.dir,
                    ..self
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            minimum_moves(&[
                &[0, 0, 0, 0, 0, 1],
                &[1, 1, 0, 0, 1, 0],
                &[0, 0, 0, 0, 1, 1],
                &[0, 0, 1, 0, 1, 0],
                &[0, 1, 1, 0, 0, 0],
                &[0, 1, 1, 0, 0, 0]
            ]),
            11
        );
        assert_eq!(
            minimum_moves(&[
                &[0, 0, 1, 1, 1, 1],
                &[0, 0, 0, 0, 1, 1],
                &[1, 1, 0, 0, 0, 1],
                &[1, 1, 1, 0, 0, 1],
                &[1, 1, 1, 0, 0, 1],
                &[1, 1, 1, 0, 0, 0]
            ]),
            9
        )
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
