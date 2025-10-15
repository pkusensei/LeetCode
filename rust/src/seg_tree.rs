pub struct SegTree<T: Copy> {
    tree: Vec<T>,
    f: fn(T, T) -> T,
    n: usize,
    identity: T,
}

#[allow(dead_code)]
impl<T: Copy> SegTree<T> {
    pub fn new(nums: &[T], f: fn(T, T) -> T, identity: T) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![identity; 4 * n],
            f,
            n,
            identity,
        };
        if n > 0 {
            s.build(1, 0, n - 1, nums);
        }
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize, nums: &[T]) {
        if left == right {
            self.tree[node] = nums[left];
            return;
        }
        let mid = (left + right) / 2;
        self.build(2 * node, left, mid, nums);
        self.build(2 * node + 1, mid + 1, right, nums);
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    pub fn query(&self, ql: usize, qr: usize) -> T {
        if self.n == 0 {
            return self.identity;
        }
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> T {
        if qr < left || right < ql {
            return self.identity;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = left.midpoint(right);
        (self.f)(
            self._query(2 * node, left, mid, ql, qr),
            self._query(2 * node + 1, mid + 1, right, ql, qr),
        )
    }

    pub fn update(&mut self, idx: usize, val: T) {
        if self.n == 0 || idx >= self.n {
            return;
        }
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: T) {
        if left == right {
            self.tree[node] = val;
            return;
        }
        let mid = left.midpoint(right);
        if idx <= mid {
            self._update(2 * node, left, mid, idx, val);
        } else {
            self._update(2 * node + 1, mid + 1, right, idx, val);
        }
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    pub fn range_update(&mut self, ql: usize, qr: usize, val: T) {
        self._range_update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _range_update(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
        val: T,
    ) {
        if qr < left || right < ql {
            return;
        }
        if ql <= left && right <= qr {
            self.tree[node] = val;
            return;
        }
        let mid = left.midpoint(right);
        self._range_update(2 * node, left, mid, ql, qr, val);
        self._range_update(2 * node + 1, 1 + mid, right, ql, qr, val);
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }
}
