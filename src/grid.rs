use std::convert::TryFrom;
use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
    io::{BufRead, BufReader, Read},
    iter::{repeat, FromIterator},
    str::FromStr,
};

#[derive(Debug)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub w: usize,
    pub h: usize,
}

impl<T> Grid<T>
where
    T: Clone + Default + TryFrom<u8>,
{
    pub fn new(w: usize, h: usize) -> Self {
        Self::new_with(w, h, Default::default())
    }

    pub fn new_with(w: usize, h: usize, val: T) -> Self {
        Self {
            cells: Vec::from_iter(repeat(val).take(w * h)),
            w,
            h,
        }
    }

    pub fn from_reader<R: Read>(r: R) -> Result<Self, T::Error> {
        let cells = BufReader::new(r)
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.bytes().map(T::try_from).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;
        let h = cells.len();
        let w = cells.first().map_or(0, |c| c.len());

        Ok(Self {
            cells: cells.into_iter().flatten().collect(),
            w,
            h,
        })
    }

    pub fn from_map(points: HashMap<Point, T>) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for pt in points.keys() {
            if pt.x < min_x {
                min_x = pt.x;
            }
            if pt.x > max_x {
                max_x = pt.x;
            }
            if pt.y < min_y {
                min_y = pt.y;
            }
            if pt.y > max_y {
                max_y = pt.y;
            }
        }
        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;
        let x_offset = -min_x;
        let y_offset = -min_y;

        let mut grid = Self::new(w as usize, h as usize);

        for (pt, cell) in points {
            let x = (pt.x + x_offset) as usize;
            let y = (pt.y + y_offset) as usize;
            grid.set((x, y), cell);
        }

        grid
    }

    pub fn set(&mut self, c: impl Coord, value: T) {
        if let Some(e) = self.cells.get_mut(c.x() + c.y() * self.w) {
            *e = value;
        }
    }

    pub fn get(&self, c: impl Coord) -> Option<&T> {
        self.cells.get(c.x() + c.y() * self.w)
    }
}

impl<T> FromStr for Grid<T>
where
    T: TryFrom<u8>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|l| l.bytes().map(T::try_from).collect::<Result<Vec<_>, _>>())
            .collect::<Result<Vec<_>, _>>()?;

        let h = cells.len();
        let w = cells.first().map_or(0, |c| c.len());

        Ok(Grid {
            cells: cells.into_iter().flatten().collect(),
            w,
            h,
        })
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in self.cells.chunks(self.w) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn coords(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug)]
pub struct GridPoint {
    x: usize,
    y: usize,
}

impl Coord for GridPoint {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl Coord for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn coords(&self) -> (usize, usize) {
        *self
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}
