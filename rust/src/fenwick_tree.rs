use std::ops::{AddAssign, Sub};

pub struct FenwickTree<T> {
    pub tree: Vec<T>,
    pub n: usize, // The size without 1-based indexing
}

#[allow(dead_code)]
impl<T> FenwickTree<T> {
    pub fn new(n: usize) -> Self
    where
        T: Clone + Default,
    {
        Self {
            tree: vec![T::default(); 1 + n],
            n,
        }
    }

    pub fn update<U>(&mut self, idx: usize, val: U)
    where
        T: From<U> + AddAssign,
        U: Clone,
    {
        assert!(
            idx < self.n,
            "Index {} out of bounds (size: {})",
            idx,
            self.n
        );
        self._update(1 + idx, val);
    }

    // !! 1-based index !!
    fn _update<U>(&mut self, mut idx: usize, val: U)
    where
        T: From<U> + AddAssign,
        U: Clone,
    {
        while idx <= self.n {
            self.tree[idx] += val.clone().into();
            idx += idx & (!idx + 1); // same as wrapping_neg
        }
    }

    // prefix sum
    pub fn query(&self, idx: usize) -> T
    where
        T: Default + Clone + AddAssign,
    {
        assert!(
            idx < self.n,
            "Index {} out of bounds (size: {})",
            idx,
            self.n
        );
        self._query(1 + idx)
    }

    fn _query(&self, mut idx: usize) -> T
    where
        T: Default + Clone + AddAssign,
    {
        let mut res = T::default();
        while idx > 0 {
            res += self.tree[idx].clone();
            idx -= idx & idx.wrapping_neg(); // idx&(-idx)
        }
        res
    }

    // range sum [left..=right]
    pub fn range_query(&self, left: usize, right: usize) -> T
    where
        T: Default + Clone + AddAssign + Sub<Output = T>,
    {
        assert!(
            left <= right,
            "Invalid range: left ({}) > right ({})",
            left,
            right
        );
        assert!(
            right < self.n,
            "Index {} out of bounds (size: {})",
            right,
            self.n
        );
        if let Some(left) = left.checked_sub(1) {
            self.query(right) - self.query(left)
        } else {
            self.query(right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut ft = FenwickTree::<i64>::new(5);

        // Test initial state - all queries should return 0
        for i in 1..=5 {
            assert_eq!(ft._query(i), 0);
        }

        // Single update and query
        ft._update(3, 10i32);
        assert_eq!(ft._query(1), 0);
        assert_eq!(ft._query(2), 0);
        assert_eq!(ft._query(3), 10);
        assert_eq!(ft._query(4), 10);
        assert_eq!(ft._query(5), 10);
    }

    #[test]
    fn test_multiple_updates() {
        let mut ft = FenwickTree::<i64>::new(5);

        // Multiple updates
        ft._update(1, 5i32);
        ft._update(3, 10i32);
        ft._update(5, 15i32);

        assert_eq!(ft._query(1), 5); // [5]
        assert_eq!(ft._query(2), 5); // [5, 0]
        assert_eq!(ft._query(3), 15); // [5, 0, 10]
        assert_eq!(ft._query(4), 15); // [5, 0, 10, 0]
        assert_eq!(ft._query(5), 30); // [5, 0, 10, 0, 15]
    }

    #[test]
    fn test_range_sum_calculation() {
        let mut ft = FenwickTree::<i64>::new(6);

        // Build array: [1, 2, 3, 4, 5, 6]
        for i in 1..=6 {
            ft._update(i, i as i32);
        }

        // Test prefix sums
        assert_eq!(ft._query(1), 1); // sum of [1]
        assert_eq!(ft._query(2), 3); // sum of [1, 2]
        assert_eq!(ft._query(3), 6); // sum of [1, 2, 3]
        assert_eq!(ft._query(4), 10); // sum of [1, 2, 3, 4]
        assert_eq!(ft._query(5), 15); // sum of [1, 2, 3, 4, 5]
        assert_eq!(ft._query(6), 21); // sum of [1, 2, 3, 4, 5, 6]

        // Test range sum: sum from index i to j = query(j) - query(i-1)
        assert_eq!(ft._query(5) - ft._query(2), 12); // sum of [3, 4, 5] = 12
        assert_eq!(ft._query(4) - ft._query(1), 9); // sum of [2, 3, 4] = 9
    }

    #[test]
    fn test_negative_values() {
        let mut ft = FenwickTree::<i64>::new(4);

        ft._update(1, -5i32);
        ft._update(2, 10i32);
        ft._update(3, -3i32);
        ft._update(4, 8i32);

        assert_eq!(ft._query(1), -5); // [-5]
        assert_eq!(ft._query(2), 5); // [-5, 10]
        assert_eq!(ft._query(3), 2); // [-5, 10, -3]
        assert_eq!(ft._query(4), 10); // [-5, 10, -3, 8]
    }

    #[test]
    fn test_cumulative_updates() {
        let mut ft = FenwickTree::<i64>::new(3);

        // Multiple updates to same position
        ft._update(2, 5i32);
        ft._update(2, 3i32);
        ft._update(2, -2i32);

        assert_eq!(ft._query(1), 0); // [0]
        assert_eq!(ft._query(2), 6); // [0, 6] (5 + 3 - 2)
        assert_eq!(ft._query(3), 6); // [0, 6, 0]
    }

    #[test]
    fn test_large_values() {
        let mut ft = FenwickTree::<i64>::new(3);

        let large_val = i32::MAX;
        ft._update(1, large_val);
        ft._update(2, large_val);

        assert_eq!(ft._query(1), large_val as i64);
        assert_eq!(ft._query(2), (large_val as i64) * 2);
    }

    #[test]
    fn test_single_element() {
        let mut ft = FenwickTree::<i64>::new(1);

        ft._update(1, 42i32);
        assert_eq!(ft._query(1), 42);
    }

    #[test]
    fn test_empty_queries() {
        let ft = FenwickTree::<i64>::new(10);

        // All queries should return 0 for empty tree
        for i in 1..=10 {
            assert_eq!(ft._query(i), 0);
        }
    }

    #[test]
    fn test_stress_random_operations() {
        use std::collections::HashMap;

        let size = 100;
        let mut ft = FenwickTree::<i64>::new(size);
        let mut reference: HashMap<usize, i32> = HashMap::new();

        // Simulate random updates
        let operations = vec![
            (1, 10),
            (50, -5),
            (100, 15),
            (25, 8),
            (75, -3),
            (1, 5),
            (50, 10),
            (30, -7),
            (80, 12),
            (10, -2),
        ];

        for (idx, val) in operations {
            ft._update(idx, val);
            *reference.entry(idx).or_insert(0) += val;
        }

        // Verify some prefix sums
        for test_idx in [1, 10, 25, 50, 75, 100] {
            let expected: i64 = reference
                .iter()
                .filter(|&(&k, _)| k <= test_idx)
                .map(|(_, &v)| v as i64)
                .sum();

            assert_eq!(
                ft._query(test_idx),
                expected,
                "Mismatch at index {}",
                test_idx
            );
        }
    }

    #[test]
    fn test_type_conversion() {
        let mut ft = FenwickTree::<i64>::new(5);

        // Test that i32 values are properly converted to i64
        ft._update(1, i32::MIN);
        ft._update(2, i32::MAX);

        assert_eq!(ft._query(1), i32::MIN.into());
        assert_eq!(ft._query(2), i64::from(i32::MIN) + i64::from(i32::MAX));
    }

    #[test]
    fn test_zero_based_indexing() {
        let mut ft = FenwickTree::<i64>::new(5);

        // Update using 0-based indices
        ft.update(0, 10i32); // Update first element
        ft.update(2, 20i32); // Update third element
        ft.update(4, 30i32); // Update last element

        // Query using 0-based indices
        assert_eq!(ft.query(0), 10); // [10]
        assert_eq!(ft.query(1), 10); // [10, 0]
        assert_eq!(ft.query(2), 30); // [10, 0, 20]
        assert_eq!(ft.query(3), 30); // [10, 0, 20, 0]
        assert_eq!(ft.query(4), 60); // [10, 0, 20, 0, 30]
    }

    #[test]
    fn test_range_query() {
        let mut ft = FenwickTree::<i64>::new(10);

        // Build array [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        for i in 1..=10 {
            ft._update(i, i as i32);
        }

        // Test various ranges
        assert_eq!(ft.range_query(0, 4), 15); // 1+2+3+4+5
        assert_eq!(ft.range_query(2, 6), 25); // 3+4+5+6+7
        assert_eq!(ft.range_query(5, 5), 6); // single element
        assert_eq!(ft.range_query(0, 9), 55); // entire array
    }
}
