use std::ops::Mul;

pub struct Matrix {
    mat: Vec<Vec<i64>>,
}

pub const M: i64 = 1_000_000_007;

impl Matrix {
    pub fn new(m: usize, n: usize) -> Self {
        assert!(m > 0);
        assert!(n > 0);
        Self {
            mat: vec![vec![0; n]; m],
        }
    }

    pub fn identity(m: usize) -> Self {
        let mut res = Self::new(m, m);
        for i in 0..m {
            res.mat[i][i] = 1;
        }
        res
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
