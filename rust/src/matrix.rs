use std::ops::Mul;

pub fn mat_mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let m = a.len();
    let n = a[0].len();
    let p = b[0].len();
    // m*n n*p => m*p
    let mut res = vec![vec![0; p]; m];
    for i1 in 0..m {
        for i2 in 0..p {
            for i3 in 0..n {
                res[i1][i2] += a[i1][i3] * b[i3][i2];
                res[i1][i2] %= M;
            }
        }
    }
    res
}

pub fn mat_pow(mut mat: Vec<Vec<i64>>, mut pow: i32) -> Vec<Vec<i64>> {
    let n = mat.len();
    let mut res = identity_mat(n);
    while pow > 0 {
        if pow & 1 == 1 {
            res = mat_mul(&res, &mat);
        }
        pow >>= 1;
        mat = mat_mul(&mat, &mat);
    }
    res
}

pub fn identity_mat(n: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        res[i][i] = 1;
    }
    res
}

pub struct Matrix {
    mat: Vec<Vec<i64>>,
}

pub const M: i64 = 1_000_000_007;

impl Matrix {
    pub fn new(m: usize, n: usize) -> Self {
        assert!(m > 0 && n > 0, "Empty matrix disallowed");
        Self::with_default(m, n, 0)
    }

    pub fn identity(n: usize) -> Self {
        Self {
            mat: identity_mat(n),
        }
    }

    pub fn with_default(m: usize, n: usize, val: i64) -> Self {
        Self {
            mat: vec![vec![val % M; n]; m],
        }
    }

    pub fn with_mat(mat: Vec<Vec<i64>>) -> Self {
        // Could use validation here
        Self { mat }
    }

    pub fn assign(&mut self, row: usize, col: usize, val: i64) -> bool {
        let Some(v) = self.mat.get_mut(row).and_then(|r| r.get_mut(col)) else {
            return false;
        };
        // Could use validation here
        *v = val;
        true
    }

    pub fn get(&self, row: usize, col: usize) -> Option<i64> {
        self.mat.get(row).and_then(|r| r.get(col)).copied()
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let m = self.mat.len();
        let n = self.mat[0].len();
        let p = rhs.mat[0].len();
        assert_eq!(
            n,
            rhs.mat.len(),
            "Matrix multiplication must work on matrices with {m}*{n} and {n}*{p}",
        ); // m*n n*p => m*p
        let res = mat_mul(&self.mat, &rhs.mat);
        Self { mat: res }
    }

    pub fn pow(self, pow: i32) -> Self {
        let n = self.mat.len();
        assert_eq!(
            n,
            self.mat[0].len(),
            "Matrix exponentiation must work on square matrix"
        );
        let res = mat_pow(self.mat, pow);
        Self { mat: res }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix::mul(self, rhs)
    }
}
