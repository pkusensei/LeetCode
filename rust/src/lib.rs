mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_robot_bounded(instructions: &str) -> bool {
    let mut rob = Robot::new();
    let start = rob;
    for b in instructions.bytes() {
        rob.step(b);
    }
    (rob.x == start.x && rob.y == start.y) || rob.dir != start.dir
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i16,
    y: i16,
    dir: Direction,
}

impl Robot {
    const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::North,
        }
    }

    fn step(&mut self, b: u8) {
        match b {
            b'L' => self.dir = self.dir.turn_left(),
            b'R' => self.dir = self.dir.turn_right(),
            b'G' => match self.dir {
                Direction::North => self.y += 1,
                Direction::West => self.x -= 1,
                Direction::South => self.y -= 1,
                Direction::East => self.x += 1,
            },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_robot_bounded("GGLLGG"));
        debug_assert!(!is_robot_bounded("GG"));
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
