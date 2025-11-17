use core::ops::{Index, IndexMut, Range};
use std::collections::HashMap;

/* 
pub struct ChunkedMap<T, const C : usize> {
    dict : HashMap<(i64, i64), [[T; C]; C]>
}

impl<T, const C : usize> ChunkedMap<T, C> {
    pub fn get(&self, index : &(i64, i64)) -> Option<&T> {
        self.dict.get(index)
            // Indexing cannnot fail within a chunk
            .map(|s| &s[(index.0 % (C as i64)) as usize][(index.1 % (C as i64)) as usize])
    }

    pub fn get_mut(&mut self, index : &(i64, i64)) -> Option<&mut T> {
        self.dict.get_mut(index)
            // Indexing cannnot fail within a chunk
            .map(|s| &mut s[(index.0 % (C as i64)) as usize][(index.1 % (C as i64)) as usize])
    }
}

impl<T, const C : usize> Index<(i64, i64)> for ChunkedMap<T, C> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        self.get(&index)
            .expect(format!("Index {:?} is not present in the chunked map!", index).as_str())
    }
}

impl<T, const C : usize> IndexMut<(i64, i64)> for ChunkedMap<T, C> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        self.get_mut(&index)
            .expect(format!("Index {:?} is not present in the chunked map!", index).as_str())
    }
}
*/

pub struct RectChunkedMap<T, const C : usize> {
    dict : HashMap<(i64, i64), [[T; C]; C]>,
    _x_dim : (i64, i64),
    _y_dim : (i64, i64)
}

impl<T, const C : usize> RectChunkedMap<T, C> {
    pub fn new() {

    }

    // Indexing
        pub fn get(&self, index : &(i64, i64)) -> Option<&T> {
            self.dict.get(index)
                // Indexing cannnot fail within a chunk
                .map(|s| &s[(index.0 % (C as i64)) as usize][(index.1 % (C as i64)) as usize])
        }

        pub fn get_mut(&mut self, index : &(i64, i64)) -> Option<&mut T> {
            self.dict.get_mut(index)
                // Indexing cannnot fail within a chunk
                .map(|s| &mut s[(index.0 % (C as i64)) as usize][(index.1 % (C as i64)) as usize])
        }
    // 

    /* Dimensions */
        /// Returns the size of the rectangular map for each size `(x, y)`
        pub fn dim(&self) -> (usize, usize) {
            (
                (self._x_dim.1 - self._x_dim.0) as usize,
                (self._y_dim.1 - self._y_dim.0) as usize
            )
        }

        pub fn x_dim(&self) -> (i64, i64) {
            self._x_dim
        }

        pub fn y_dim(&self) -> (i64, i64) {
            self._y_dim
        }

        // Ranges
        pub fn x_range(&self) -> Range<i64> {
            self._x_dim.0 .. self._x_dim.1
        }

        pub fn y_range(&self) -> Range<i64> {
            self._y_dim.0 .. self._y_dim.1
        }
    /**/
}

impl<T, const C : usize> Index<(i64, i64)> for RectChunkedMap<T, C> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        self.get(&index)
            .expect(format!("Index {:?} is not present in the chunked map!", index).as_str())
    }
}

impl<T, const C : usize> IndexMut<(i64, i64)> for RectChunkedMap<T, C> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        self.get_mut(&index)
            .expect(format!("Index {:?} is not present in the chunked map!", index).as_str())
    }
}