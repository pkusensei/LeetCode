pub struct SegTree {
    tree: Vec<i32>,
    f: fn(i32, i32) -> i32,
    n: usize,
    identity: i32,
}

#[allow(dead_code)]
impl SegTree {
    pub fn new(nums: &[i32], f: fn(i32, i32) -> i32, identity: i32) -> Self {
        let n = nums.len();
        let mut s = Self {
            tree: vec![0; 4 * n],
            f,
            n,
            identity,
        };
        if n > 0 {
            s.build(1, 0, n - 1, nums);
        }
        s
    }

    fn build(&mut self, node: usize, left: usize, right: usize, nums: &[i32]) {
        if left == right {
            self.tree[node] = nums[left];
            return;
        }
        let mid = (left + right) / 2;
        self.build(2 * node, left, mid, nums);
        self.build(2 * node + 1, mid + 1, right, nums);
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    pub fn query(&self, ql: usize, qr: usize) -> i32 {
        if self.n == 0 {
            return self.identity;
        }
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
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

    pub fn update(&mut self, idx: usize, val: i32) {
        if self.n == 0 || idx >= self.n {
            return;
        }
        self._update(1, 0, self.n - 1, idx, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, idx: usize, val: i32) {
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

    pub fn range_update(&mut self, ql: usize, qr: usize, val: i32) {
        self._range_update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _range_update(
        &mut self,
        node: usize,
        left: usize,
        right: usize,
        ql: usize,
        qr: usize,
        val: i32,
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

pub struct LazySegTree {
    tree: Vec<i32>,
    lazy: Vec<i32>,
    f: fn(i32, i32) -> i32,
    n: usize,
    identity: i32,
}

#[allow(dead_code)]
impl LazySegTree {
    pub fn new(n: usize, f: fn(i32, i32) -> i32, identity: i32) -> Self {
        Self {
            tree: vec![0; 4 * n],
            lazy: vec![0; 4 * n],
            f,
            n,
            identity,
        }
    }

    pub fn update(&mut self, ql: usize, qr: usize, val: i32) {
        self._update(1, 0, self.n - 1, ql, qr, val);
    }

    fn _update(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize, val: i32) {
        self.push(node, left, right);
        if right < ql || qr < left {
            return;
        }
        if ql <= left && right <= qr {
            self.lazy[node] += val;
            self.push(node, left, right);
            return;
        }
        let mid = left.midpoint(right);
        self._update(2 * node, left, mid, ql, qr, val);
        self._update(2 * node + 1, 1 + mid, right, ql, qr, val);
        self.tree[node] = (self.f)(self.tree[2 * node], self.tree[2 * node + 1]);
    }

    pub fn query(&mut self, ql: usize, qr: usize) -> i32 {
        self._query(1, 0, self.n - 1, ql, qr)
    }

    fn _query(&mut self, node: usize, left: usize, right: usize, ql: usize, qr: usize) -> i32 {
        self.push(node, left, right);
        if right < ql || qr < left {
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

    fn push(&mut self, node: usize, left: usize, right: usize) {
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node];
            if left < right {
                self.lazy[2 * node] += self.lazy[node];
                self.lazy[2 * node + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
    }
}
