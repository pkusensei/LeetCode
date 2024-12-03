mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct SnapshotArray {
    id: i32,
    data: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            id: 0,
            data: vec![vec![]; length as usize],
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        if let Some(v) = self.data[idx].last_mut() {
            if v.0 == self.id {
                v.1 = val
            } else {
                self.data[idx].push((self.id, val));
            }
        } else {
            self.data[idx].push((self.id, val))
        }
    }

    fn snap(&mut self) -> i32 {
        self.id += 1;
        self.id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.data[index as usize].binary_search_by_key(&snap_id, |v| v.0) {
            Ok(i) => self.data[index as usize][i].1,
            Err(i) if i == 0 => 0,
            Err(i) => self.data[index as usize][i - 1].1,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut sn = SnapshotArray::new(3); // set the length to be 3
        sn.set(0, 5); // Set array[0] = 5
        assert_eq!(sn.snap(), 0); // Take a snapshot, return snap_id = 0
        sn.set(0, 6);
        assert_eq!(sn.get(0, 0), 5);
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
