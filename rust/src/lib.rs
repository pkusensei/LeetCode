mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn robot_sim(commands: &[i32], obstacles: &[[i32; 2]]) -> i32 {
    let mut rob = Robot::new();
    commands
        .iter()
        .fold(0, |acc, &num| rob.walk(num, obstacles, acc))
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    dir: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    fn new() -> Self {
        Self {
            dir: Direction::North,
            x: 0,
            y: 0,
        }
    }

    fn dist(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }

    fn walk(&mut self, mut num: i32, obstacles: &[[i32; 2]], mut dist: i32) -> i32 {
        match num {
            -2 => self.dir = self.dir.turn_left(),
            -1 => self.dir = self.dir.turn_right(),
            1..=9 => {
                while num > 0 {
                    num -= 1;
                    let (x, y) = match self.dir {
                        Direction::North => (self.x, self.y + 1),
                        Direction::West => (self.x - 1, self.y),
                        Direction::South => (self.x, self.y - 1),
                        Direction::East => (self.x + 1, self.y),
                    };
                    if obstacles.contains(&[x, y]) {
                        return dist;
                    } else {
                        (self.x, self.y) = (x, y);
                        dist = dist.max(self.dist())
                    }
                }
            }
            _ => (),
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(robot_sim(&[4, -1, 3], &[]), 25);
        debug_assert_eq!(robot_sim(&[4, -1, 4, -2, 4], &[[2, 4]]), 65);
        debug_assert_eq!(robot_sim(&[6, -1, -1, 6], &[]), 36);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
