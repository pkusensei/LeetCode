mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct DinnerPlates {
    cap: usize,
    stacks: Vec<Vec<i32>>,
    slots: std::collections::BTreeSet<usize>,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            cap: capacity as usize,
            ..Default::default()
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(&i) = self.slots.iter().next() {
            self.stacks[i].push(val);
            if self.stacks[i].len() == self.cap {
                self.slots.remove(&i);
            }
        } else {
            let v = vec![val];
            self.stacks.push(v);
            if self.cap > 1 {
                self.slots.insert(self.stacks.len() - 1);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        let Some((idx, st)) = self
            .stacks
            .iter_mut()
            .enumerate()
            .rfind(|(_i, st)| !st.is_empty())
        else {
            return -1;
        };
        let res = st.pop().unwrap_or(-1);
        if st.is_empty() {
            self.slots.retain(|&i| i < idx);
            self.stacks.truncate(idx);
        } else {
            self.slots.insert(idx);
        }
        res
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let idx = index as usize;
        if self.stacks.len() <= idx {
            return -1;
        }
        if self.stacks.len() - 1 == idx {
            return self.pop();
        }
        if self.stacks[idx].len() == self.cap {
            self.slots.insert(idx);
        }
        self.stacks[idx].pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut D = DinnerPlates::new(2); // Initialize with capacity = 2
        D.push(1);
        D.push(2);
        D.push(3);
        D.push(4);
        D.push(5);
        assert_eq!(D.pop_at_stack(0), 2);
        D.push(20);
        D.push(21);
        assert_eq!(D.pop_at_stack(0), 20);
        assert_eq!(D.pop_at_stack(2), 21);
        assert_eq!(D.pop(), 5);
        assert_eq!(D.pop(), 4);
        assert_eq!(D.pop(), 3);
        assert_eq!(D.pop(), 1);
        assert_eq!(D.pop(), -1);
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
