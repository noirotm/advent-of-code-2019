use crate::{grid::Grid, solver::Solver};
use num::Integer;
use std::{
    collections::HashMap,
    convert::TryFrom,
    f64::consts::PI,
    fmt::{Display, Error, Formatter},
    io::Read,
};

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<Elem>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        Grid::from_reader(r).unwrap()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (_, s) = find_best_location(input);
        s
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let (pt, _) = find_best_location(input);
        let mut field = input.clone();
        let v = find_all_vaporized_from_point(&mut field, &pt);

        let pt = v.get(199).unwrap();
        pt.x * 100 + pt.y
    }
}

fn find_best_location(grid: &Grid<Elem>) -> (Point, usize) {
    let mut visibles = vec![];
    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.get((x, y)) == Some(&Elem::Empty) {
                continue;
            }

            let pt = Point { x, y };
            let visible = find_visible_from_point(grid, &pt).len();
            visibles.push((pt.clone(), visible));
        }
    }

    visibles.into_iter().max_by_key(|(_, l)| *l).unwrap()
}

fn find_visible_from_point(grid: &Grid<Elem>, origin: &Point) -> Vec<(Point, Vector2D)> {
    let mut closest_points: HashMap<Vector2D, Point> = HashMap::new();

    for y in 0..grid.h {
        for x in 0..grid.w {
            if grid.get((x, y)) == Some(&Elem::Empty) {
                continue;
            }

            let point = Point { x, y };

            if *origin == point {
                continue;
            }

            let angle = origin.vector(&point);

            if let Some(closest) = closest_points.get(&angle).cloned() {
                let closest_dist = origin.distance(&closest);
                let current_dist = origin.distance(&point);

                if current_dist < closest_dist {
                    closest_points.insert(angle, point);
                }
            } else {
                closest_points.insert(angle, point.clone());
            }
        }
    }

    closest_points
        .iter()
        .map(|(a, pt)| (pt.clone(), a.clone()))
        .collect()
}

fn find_all_vaporized_from_point(grid: &mut Grid<Elem>, origin: &Point) -> Vec<Point> {
    let mut all_vap = vec![];
    loop {
        let mut visible_points = find_visible_from_point(&grid, origin);
        visible_points.sort_by(|(_, a), (_, b)| a.degrees().partial_cmp(&b.degrees()).unwrap());

        for (p, _) in visible_points.iter() {
            grid.set((p.x, p.y), Elem::Empty);
            all_vap.push(p.clone());
        }

        if visible_points.is_empty() {
            break;
        }
    }

    all_vap
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Vector2D {
    dx: isize,
    dy: isize,
}

impl Vector2D {
    fn new(dx: isize, dy: isize) -> Self {
        let gcd = dx.gcd(&dy);
        Self {
            dx: dx / gcd,
            dy: dy / gcd,
        }
    }

    fn degrees(&self) -> f64 {
        let angle = (self.dy as f64).atan2(self.dx as f64);
        let d = 180.0 * angle / PI + 90.0; // rotate by 90° as origin is up instead of right
        if d < 0.0 {
            d + 360.0
        } else {
            d
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Elem {
    Empty,
    Asteroid,
}

impl TryFrom<u8> for Elem {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Elem::Empty),
            b'#' => Ok(Elem::Asteroid),
            v => Err(format!("Invalid cell: {}", v)),
        }
    }
}

impl Default for Elem {
    fn default() -> Self {
        Elem::Empty
    }
}

impl Display for Elem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match self {
                Elem::Empty => '.',
                Elem::Asteroid => '#',
            }
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    fn vector(&self, other: &Point) -> Vector2D {
        Vector2D::new(
            other.x as isize - self.x as isize,
            other.y as isize - self.y as isize,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_vectors() {
        let p1 = Point { x: 5, y: 9 };
        let p2 = Point { x: 5, y: 2 };
        assert!(p1.vector(&p2).degrees() - 0.0 < 1e-10);

        let p1 = Point { x: 5, y: 9 };
        let p2 = Point { x: 10, y: 9 };
        assert!(p1.vector(&p2).degrees() - 90.0 < 1e-10);

        let p1 = Point { x: 5, y: 9 };
        let p2 = Point { x: 0, y: 9 };
        assert!(p1.vector(&p2).degrees() - 270.0 < 1e-10);
    }

    #[test]
    fn test_vector() {
        let a = Vector2D { dx: 0, dy: -1 };
        assert!(a.degrees() < 1e-10);

        let a = Vector2D { dx: 1, dy: 0 };
        assert!(a.degrees() - 90.0 < 1e-10);

        let a = Vector2D { dx: 0, dy: 1 };
        assert!(a.degrees() - 180.0 < 1e-10);

        let a = Vector2D { dx: -1, dy: 0 };
        assert!(a.degrees() - 270.0 < 1e-10);
    }
}
