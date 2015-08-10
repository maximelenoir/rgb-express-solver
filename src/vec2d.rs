use std::ops;
use std::fmt;
use std::clone;
use std::slice;
use std::default::Default;
use std::result::Result;

pub struct Vec2D<T> {
    pub width: usize,
    pub height: usize,
    pub undl: Vec<T>,
}

impl<T: Default + Clone> Vec2D<T> {
    pub fn new(width: usize, height: usize) -> Vec2D<T> {
        Vec2D {
            width: width,
            height: height,
            undl: vec![T::default(); width * height],
        }
    }

    pub fn iter(&self) -> slice::Iter<T> {
        self.undl.iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec2D<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for (r, row) in self.undl.chunks(self.width).enumerate() {
            if r > 0 {
                try!(write!(fmt, "\n"));
            }
            try!(write!(fmt, "{:?}", row));
        }
        Result::Ok(())
    }
}

impl<T> ops::Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &T {
        let idx = y * self.width + x;
        self.undl.get(idx).unwrap()
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        let idx = y * self.width + x;
        self.undl.get_mut(idx).unwrap()
    }
}

impl<T> clone::Clone for Vec2D<T> where T: Clone {
    fn clone(&self) -> Self {
        Vec2D {
            width: self.width,
            height: self.height,
            undl: self.undl.to_vec(),
        }
    }
}
