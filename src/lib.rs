pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        const SIDE: i32 = 2;
        let mut turn = 1;
        let mut res = Vec::with_capacity((rows * cols) as usize);
        let mut dir = Dir::East;
        let mut curr = [r_start, c_start];
        res.push(curr);
        let mut step = 0;
        let mut side = 0;

        while res.len() < (rows * cols) as usize {
            curr = dir.walk(curr);
            if (0..rows).contains(&curr[0]) && (0..cols).contains(&curr[1]) {
                res.push(curr);
            }
            step += 1;
            if step == turn {
                step = 0;
                side += 1;
                dir = dir.turn();
                if side == SIDE {
                    side = 0;
                    turn += 1
                }
            }
        }
        res.into_iter().map(|v| v.into()).collect()
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    fn turn(self) -> Self {
        match self {
            Dir::East => Self::South,
            Dir::South => Self::West,
            Dir::West => Self::North,
            Dir::North => Self::East,
        }
    }

    fn walk(self, [row, col]: [i32; 2]) -> [i32; 2] {
        match self {
            Dir::East => [row, col + 1],
            Dir::South => [row + 1, col],
            Dir::West => [row, col - 1],
            Dir::North => [row - 1, col],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            spiral_matrix_iii(1, 4, 0, 0),
            [[0, 0], [0, 1], [0, 2], [0, 3]]
        );
        #[rustfmt::skip]
        debug_assert_eq!(
            spiral_matrix_iii(5, 6, 1, 4),
            [
                [1, 4], [1, 5], [2, 5], [2, 4], [2, 3], [1, 3],
                [0, 3], [0, 4], [0, 5], [3, 5], [3, 4], [3, 3], 
                [3, 2], [2, 2], [1, 2], [0, 2], [4, 5], [4, 4], 
                [4, 3], [4, 2], [4, 1], [3, 1], [2, 1], [1, 1],
                [0, 1], [4, 0], [3, 0], [2, 0], [1, 0], [0, 0]
            ]
        );
    }

    #[test]
    fn test() {}
}
