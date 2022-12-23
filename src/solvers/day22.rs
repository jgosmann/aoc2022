use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Mul,
};

use super::base::AocSolver;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn delta(&self) -> (isize, isize) {
        match self {
            Facing::Down => (1, 0),
            Facing::Up => (-1, 0),
            Facing::Left => (0, -1),
            Facing::Right => (0, 1),
        }
    }

    fn encode(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rotation {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Forward(usize),
    Rotation(Rotation),
}

struct MovesParser<'a> {
    input: &'a str,
}

impl<'a> MovesParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for MovesParser<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.as_bytes().first().copied() {
            None => None,
            Some(rotation) if rotation == b'R' || rotation == b'L' => {
                self.input = &self.input[1..];
                Some(Move::Rotation(match rotation {
                    b'R' => Rotation::Right,
                    b'L' => Rotation::Left,
                    _ => unreachable!(),
                }))
            }
            Some(_) => {
                let mut i: usize = 0;
                while let Some(c) = self.input.as_bytes().get(i) {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    i += 1;
                }
                let result = self.input[..i].parse().ok().map(Move::Forward);
                self.input = &self.input[i..];
                result
            }
        }
    }
}

type Vec3 = (isize, isize, isize);

#[derive(Debug, PartialEq, Eq)]
struct Mat3x3(Vec3, Vec3, Vec3);

impl Mat3x3 {
    fn inv(&self) -> Self {
        let d0 = self.1 .1 * self.2 .2 - self.2 .1 * self.1 .2;
        let d1 = -self.0 .1 * self.2 .2 + self.2 .1 * self.0 .2;
        let d2 = self.0 .1 * self.1 .2 - self.1 .1 * self.0 .2;
        let div = self.0 .0 * d0 + self.1 .0 * d1 + self.2 .0 * d2;
        Self(
            (d0 / div, d1 / div, d2 / div),
            (
                (self.2 .0 * self.1 .2 - self.1 .0 * self.2 .2) / div,
                (self.0 .0 * self.2 .2 - self.2 .0 * self.0 .2) / div,
                (self.1 .0 * self.0 .2 - self.0 .0 * self.1 .2) / div,
            ),
            (
                (self.1 .0 * self.2 .1 - self.2 .0 * self.1 .1) / div,
                (self.2 .0 * self.0 .1 - self.0 .0 * self.2 .1) / div,
                (self.0 .0 * self.1 .1 - self.1 .0 * self.0 .1) / div,
            ),
        )
    }
}

impl Mul for &Mat3x3 {
    type Output = Mat3x3;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat3x3(
            (
                (self.0 .0 * rhs.0 .0 + self.1 .0 * rhs.0 .1 + self.2 .0 * rhs.0 .2),
                (self.0 .1 * rhs.0 .0 + self.1 .1 * rhs.0 .1 + self.2 .1 * rhs.0 .2),
                (self.0 .2 * rhs.0 .0 + self.1 .2 * rhs.0 .1 + self.2 .2 * rhs.0 .2),
            ),
            (
                (self.0 .0 * rhs.1 .0 + self.1 .0 * rhs.1 .1 + self.2 .0 * rhs.1 .2),
                (self.0 .1 * rhs.1 .0 + self.1 .1 * rhs.1 .1 + self.2 .1 * rhs.1 .2),
                (self.0 .2 * rhs.1 .0 + self.1 .2 * rhs.1 .1 + self.2 .2 * rhs.1 .2),
            ),
            (
                (self.0 .0 * rhs.2 .0 + self.1 .0 * rhs.2 .1 + self.2 .0 * rhs.2 .2),
                (self.0 .1 * rhs.2 .0 + self.1 .1 * rhs.2 .1 + self.2 .1 * rhs.2 .2),
                (self.0 .2 * rhs.2 .0 + self.1 .2 * rhs.2 .1 + self.2 .2 * rhs.2 .2),
            ),
        )
    }
}

impl Mul<&Mat3x3> for Mat3x3 {
    type Output = Mat3x3;

    fn mul(self, rhs: &Mat3x3) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Mat3x3> for &Mat3x3 {
    type Output = Mat3x3;

    fn mul(self, rhs: Mat3x3) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Mat3x3> for Mat3x3 {
    type Output = Mat3x3;

    fn mul(self, rhs: Mat3x3) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Vec3> for &Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        (
            self.0 .0 * rhs.0 + self.1 .0 * rhs.1 + self.2 .0 * rhs.2,
            self.0 .1 * rhs.0 + self.1 .1 * rhs.1 + self.2 .1 * rhs.2,
            self.0 .2 * rhs.0 + self.1 .2 * rhs.1 + self.2 .2 * rhs.2,
        )
    }
}

impl Mul<&Vec3> for Mat3x3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Vec3> for &Mat3x3 {
    type Output = Vec3;

    #[allow(clippy::op_ref)]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self * &rhs
    }
}

impl Mul<Vec3> for Mat3x3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        &self * rhs
    }
}

static FOLD_DOWN: &Mat3x3 = &Mat3x3((0, 0, -1), (0, 1, 0), (1, 0, 0));
static FOLD_UP: &Mat3x3 = &Mat3x3((0, 0, 1), (0, 1, 0), (-1, 0, 0));
static FOLD_RIGHT: &Mat3x3 = &Mat3x3((1, 0, 0), (0, 0, -1), (0, 1, 0));
static FOLD_LEFT: &Mat3x3 = &Mat3x3((1, 0, 0), (0, 0, 1), (0, -1, 0));

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum CubeFace {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

impl TryFrom<Vec3> for CubeFace {
    type Error = anyhow::Error;

    fn try_from(value: Vec3) -> Result<Self, Self::Error> {
        match value {
            (0, 0, 1) => Ok(CubeFace::Top),
            (0, 0, -1) => Ok(CubeFace::Bottom),
            (1, 0, 0) => Ok(CubeFace::Front),
            (-1, 0, 0) => Ok(CubeFace::Back),
            (0, -1, 0) => Ok(CubeFace::Left),
            (0, 1, 0) => Ok(CubeFace::Right),
            _ => Err(anyhow::anyhow!("not a valid normal")),
        }
    }
}

struct Map<'a> {
    map: Vec<&'a [u8]>,
    width: usize,
}

impl<'a> Map<'a> {
    pub fn new(map: Vec<&'a [u8]>) -> Self {
        Self {
            width: map.iter().map(|row| row.len()).max().unwrap(),
            map,
        }
    }

    pub fn start(&self) -> (usize, usize) {
        (0, self.map[0].iter().position(|&c| c == b'.').unwrap())
    }

    pub fn is_free(&self, pos: (usize, usize)) -> bool {
        self.map[pos.0][pos.1] != b'#'
    }

    pub fn next_in_direction(&self, pos: (usize, usize), facing: Facing) -> (usize, usize) {
        let mut next = self.step_in_facing(pos, facing);
        while next.1 >= self.map[next.0].len() || self.map[next.0][next.1] == b' ' {
            next = self.step_in_facing(next, facing);
        }
        next
    }

    fn step_in_facing(&self, pos: (usize, usize), facing: Facing) -> (usize, usize) {
        let delta = facing.delta();
        let mut next = (pos.0 as isize + delta.0, pos.1 as isize + delta.1);

        if next.0 < 0 {
            next.0 = self.map.len() as isize - 1;
        }
        if next.0 >= self.map.len() as isize {
            next.0 = 0;
        }
        if next.1 < 0 {
            next.1 = self.width as isize - 1;
        }
        if next.1 >= self.width as isize {
            next.1 = 0;
        }

        (next.0 as usize, next.1 as usize)
    }

    fn edge_length(&self) -> usize {
        let (long, short) = if self.width > self.map.len() {
            (self.width, self.map.len())
        } else {
            (self.map.len(), self.width)
        };
        if long / 4 == short / 3 {
            long / 4
        } else if long / 5 == short / 2 {
            long / 5
        } else {
            panic!("not a cube unfolding?");
        }
    }
}

struct Tracer<'a, 'b> {
    map: &'a Map<'b>,
    pos: (usize, usize),
    facing: Facing,
}

impl<'a, 'b> Tracer<'a, 'b> {
    pub fn new(map: &'a Map<'b>) -> Self {
        Self {
            map,
            pos: map.start(),
            facing: Facing::Right,
        }
    }

    pub fn execute_move(&mut self, mv: Move) {
        match mv {
            Move::Forward(steps) => {
                for _ in 0..steps {
                    let next = self.map.next_in_direction(self.pos, self.facing);
                    if self.map.is_free(next) {
                        self.pos = next;
                    } else {
                        break;
                    }
                }
            }
            Move::Rotation(Rotation::Left) => {
                self.facing = match self.facing {
                    Facing::Down => Facing::Right,
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Right => Facing::Up,
                };
            }
            Move::Rotation(Rotation::Right) => {
                self.facing = match self.facing {
                    Facing::Down => Facing::Left,
                    Facing::Up => Facing::Right,
                    Facing::Left => Facing::Up,
                    Facing::Right => Facing::Down,
                }
            }
        }
    }

    pub fn password(&self) -> usize {
        1000 * (self.pos.0 + 1) + 4 * (self.pos.1 + 1) + self.facing.encode()
    }
}

struct CubeTracer<'a, 'b> {
    map: &'a Map<'b>,
    edge_length: usize,
    faces: HashMap<CubeFace, (Mat3x3, (usize, usize))>,
    facing: Facing,
    face: CubeFace,
    pos: (isize, isize),
}

impl<'a, 'b> CubeTracer<'a, 'b> {
    pub fn from_map(map: &'a Map<'b>) -> Self {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut faces: HashMap<CubeFace, (Mat3x3, (usize, usize))> = HashMap::new();
        let edge_length = map.edge_length();

        let mut queue: VecDeque<(Mat3x3, (usize, usize))> = VecDeque::new();
        visited.insert(map.start());
        queue.push_back((Mat3x3((1, 0, 0), (0, 1, 0), (0, 0, 1)), map.start()));
        while let Some((orientation, origin)) = queue.pop_front() {
            let check_pos = (origin.0 + edge_length, origin.1);
            if !visited.contains(&check_pos)
                && check_pos.0 < map.map.len()
                && check_pos.1 < map.map[check_pos.0].len()
                && map.map[check_pos.0][check_pos.1] != b' '
            {
                visited.insert(check_pos);
                queue.push_back(((FOLD_UP * orientation.inv()).inv(), check_pos));
            }
            if origin.0 >= edge_length {
                let check_pos = (origin.0 - edge_length, origin.1);
                if !visited.contains(&check_pos)
                    && check_pos.1 < map.map[check_pos.0].len()
                    && map.map[check_pos.0][check_pos.1] != b' '
                {
                    visited.insert(check_pos);
                    queue.push_back(((FOLD_DOWN * orientation.inv()).inv(), check_pos));
                }
            }

            let check_pos = (origin.0, origin.1 + edge_length);
            if !visited.contains(&check_pos)
                && check_pos.0 < map.map.len()
                && check_pos.1 < map.map[check_pos.0].len()
                && map.map[check_pos.0][check_pos.1] != b' '
            {
                visited.insert(check_pos);
                queue.push_back(((FOLD_LEFT * orientation.inv()).inv(), check_pos));
            }
            if origin.1 >= edge_length {
                let check_pos = (origin.0, origin.1 - edge_length);
                if !visited.contains(&check_pos) && map.map[check_pos.0][check_pos.1] != b' ' {
                    visited.insert(check_pos);
                    queue.push_back(((FOLD_RIGHT * orientation.inv()).inv(), check_pos));
                }
            }

            faces.insert(
                CubeFace::try_from(orientation.2).unwrap(),
                (orientation, origin),
            );
        }

        Self {
            edge_length,
            map,
            faces,
            face: CubeFace::Top,
            facing: Facing::Right,
            pos: (0, 0),
        }
    }

    fn normalize_pos(
        &self,
        facing: Facing,
        (face, pos): (CubeFace, (isize, isize)),
    ) -> (Facing, CubeFace, (isize, isize)) {
        let el = self.edge_length as isize;
        assert!((0 <= pos.0 && pos.0 < el) || (0 <= pos.1 && pos.1 < el));
        if 0 <= pos.0 && pos.0 < el && 0 <= pos.1 && pos.1 < el {
            return (facing, face, pos);
        }
        let folding = if pos.0 < 0 {
            FOLD_DOWN
        } else if pos.0 >= el {
            FOLD_UP
        } else if pos.1 < 0 {
            FOLD_RIGHT
        } else if pos.1 >= el {
            FOLD_LEFT
        } else {
            unreachable!()
        };

        let folded = (folding * self.faces.get(&face).unwrap().0.inv()).inv();
        let new_face = CubeFace::try_from(folded.2).unwrap();
        let target_orientation = &self.faces.get(&new_face).unwrap().0;
        let rotation = target_orientation.inv() * folded;
        let new_pos = &rotation * (pos.0 + el, pos.1 + el, 0);

        let new_facing = rotation * (facing.delta().0, facing.delta().1, 0);
        let new_facing = match new_facing {
            (1, 0, 0) => Facing::Down,
            (-1, 0, 0) => Facing::Up,
            (0, 1, 0) => Facing::Right,
            (0, -1, 0) => Facing::Left,
            _ => panic!("invalid facing"),
        };

        (
            new_facing,
            new_face,
            (self.to_range(new_pos.0), self.to_range(new_pos.1)),
        )
    }

    fn is_free(&self, (face, pos): (CubeFace, (isize, isize))) -> bool {
        let origin = self.faces.get(&face).unwrap().1;
        self.map.map[origin.0 + pos.0 as usize][origin.1 + pos.1 as usize] != b'#'
    }

    fn to_range(&self, mut value: isize) -> isize {
        if value < 0 {
            value -= 1;
        }
        while value < 0 {
            value += self.edge_length as isize;
        }
        while value >= self.edge_length as isize {
            value -= self.edge_length as isize;
        }
        value
    }

    pub fn execute_move(&mut self, mv: Move) {
        match mv {
            Move::Forward(steps) => {
                for _ in 0..steps {
                    let delta = self.facing.delta();
                    let new_pos = (self.pos.0 + delta.0, self.pos.1 + delta.1);
                    let (new_facing, face, new_pos) =
                        self.normalize_pos(self.facing, (self.face, new_pos));
                    if self.is_free((face, new_pos)) {
                        self.facing = new_facing;
                        self.face = face;
                        self.pos = new_pos;
                    } else {
                        break;
                    }
                }
            }
            Move::Rotation(Rotation::Left) => {
                self.facing = match self.facing {
                    Facing::Down => Facing::Right,
                    Facing::Up => Facing::Left,
                    Facing::Left => Facing::Down,
                    Facing::Right => Facing::Up,
                };
            }
            Move::Rotation(Rotation::Right) => {
                self.facing = match self.facing {
                    Facing::Down => Facing::Left,
                    Facing::Up => Facing::Right,
                    Facing::Left => Facing::Up,
                    Facing::Right => Facing::Down,
                }
            }
        }
    }

    pub fn password(&self) -> usize {
        let map_pos = self.map_pos();
        1000 * (map_pos.0 + 1) + 4 * (map_pos.1 + 1) + self.facing.encode()
    }

    pub fn map_pos(&self) -> (usize, usize) {
        let origin = self.faces.get(&self.face).unwrap().1;
        (
            origin.0 + self.pos.0 as usize,
            origin.1 + self.pos.1 as usize,
        )
    }
}

pub struct Solver<'a> {
    map: Map<'a>,
    path: Vec<Move>,
}

impl<'a> AocSolver<'a, usize, usize> for Solver<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut map: Vec<&[u8]> = vec![];
        let mut path = vec![];
        let mut is_map_complete = false;
        for line in input.split('\n') {
            if line.trim().is_empty() {
                is_map_complete = true;
            } else if is_map_complete {
                path = MovesParser::new(line).collect()
            } else {
                map.push(line.trim_end().as_bytes())
            }
        }

        let map = Map::new(map);
        Ok(Self { map, path })
    }

    fn solve_part1(&self) -> anyhow::Result<usize> {
        let mut tracer = Tracer::new(&self.map);
        for mv in &self.path {
            tracer.execute_move(*mv);
        }
        Ok(tracer.password())
    }

    fn solve_part2(&self) -> anyhow::Result<Option<usize>> {
        let mut tracer = CubeTracer::from_map(&self.map);
        for mv in &self.path {
            tracer.execute_move(*mv);
        }
        Ok(Some(tracer.password()))
    }
}

#[cfg(test)]
mod tests {
    use crate::solvers::test::test_example_input;

    use super::*;

    static IDENTITY: Mat3x3 = Mat3x3((1, 0, 0), (0, 1, 0), (0, 0, 1));

    #[test]
    fn test_folding_inverses() {
        assert_eq!(FOLD_DOWN * FOLD_UP, IDENTITY);
        assert_eq!(FOLD_UP * FOLD_DOWN, IDENTITY);
        assert_eq!(FOLD_LEFT * FOLD_RIGHT, IDENTITY);
        assert_eq!(FOLD_RIGHT * FOLD_LEFT, IDENTITY);
    }

    #[test]
    fn test_mat_inv() {
        assert_eq!(FOLD_DOWN.inv(), *FOLD_UP);
        assert_eq!(FOLD_UP.inv(), *FOLD_DOWN);
        assert_eq!(FOLD_LEFT.inv(), *FOLD_RIGHT);
        assert_eq!(FOLD_RIGHT.inv(), *FOLD_LEFT);
    }

    #[test]
    fn test_example() {
        let input = include_str!("examples/day22");
        test_example_input::<Solver, _, _>(input, 6032, Some(5031));
    }
}
