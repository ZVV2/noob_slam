use core::ops::{Index, IndexMut, Range};
use std::collections::HashMap;

pub struct RectChunkedMap<T, const C : usize> {
    dict : HashMap<(i64, i64), [[T; C]; C]>,
    /// Number of chunks in `(negative X, positive X)` direction
    _x_chunks : (i64, i64),
    /// Number of chunks in `(negative Y, positive Y)` direction
    _y_chunks : (i64, i64),

    // Iteration position when using a `Copy` type
    __iter_pos : (i64, i64)
}

impl<T : Default + Copy, const C : usize> RectChunkedMap<T, C> {
    pub fn with_chunks_centered(size : (usize, usize)) -> Self {
        let mut dict = HashMap::with_capacity(size.0 * size.1);

        let _x_chunks = (
            -((size.0 / 2) as i64), (size.0 / 2 + size.0 % 2) as i64,
        );
        let _y_chunks = (
            -((size.1 / 2) as i64), (size.1 / 2 + size.1 % 2) as i64
        );

        for i_x in _x_chunks.0 .. _x_chunks.1 {
            for i_y in _y_chunks.0 .. _y_chunks.1 {
                dict.insert((i_x, i_y), [[T::default(); C]; C]);
            }
        }

        Self {
            dict, 
            _x_chunks, 
            _y_chunks, 
            __iter_pos: (0, 0)
        }
    }

    pub fn with_chunks(chunks_x : (i64, i64), chunks_y : (i64, i64)) -> Self {
        // Guards
        if chunks_x.0 > 0 {
            panic!("Negative size dimension cannot be positive! (Got {} as X-negative)", chunks_x.0);
        }

        if chunks_y.0 > 0 {
            panic!("Negative size dimension cannot be positive! (Got {} as Y-negative)", chunks_y.0);
        }

        if chunks_y.1 < 0 {
            panic!("Positive size dimension cannot be negative! (Got {} as X-positive", chunks_x.1);
        }

        if chunks_y.1 < 0 {
            panic!("Positive size dimension cannot be negative! (Got {} as Y-positive", chunks_y.1);
        }

        let mut dict = HashMap::with_capacity(((chunks_x.1 - chunks_x.0) * (chunks_y.1 - chunks_y.0)) as usize);

        for i_x in chunks_x.0 .. chunks_x.1 {
            for i_y in chunks_y.0 .. chunks_y.1 {
                dict.insert((i_x, i_y), [[T::default(); C]; C]);
            }
        }

        Self {
            dict, 
            _x_chunks: chunks_x, 
            _y_chunks: chunks_y,
            __iter_pos: (0, 0)
        }
    }

    pub fn with_size_centered(size : (usize, usize)) -> Self {
        Self::with_chunks_centered(((size.0 - 1) / C + 1, (size.1 - 1) / C + 1))
    }

    pub fn with_size(size_x : (i64, i64), size_y : (i64, i64)) -> Self {
        Self::with_chunks(
            (size_x.0.div_euclid(C as i64), (size_x.1 - 1) / C as i64 + 1),
            (size_y.0.div_euclid(C as i64), (size_y.1 - 1) / C as i64 + 1)
        )
    }
}

impl<T, const C : usize> RectChunkedMap<T, C> {
    /* Indexing */ 
        pub fn get(&self, index : &(i64, i64)) -> Option<&T> {
            self.dict.get(&(index.0.div_euclid(C as i64), index.1.div_euclid(C as i64)))
                // Indexing cannnot fail within a chunk
                .map(|s| &s[index.0.rem_euclid(C as i64) as usize][index.1.rem_euclid(C as i64) as usize])
        }

        pub fn get_mut(&mut self, index : &(i64, i64)) -> Option<&mut T> {
            self.dict.get_mut(&(index.0.div_euclid(C as i64), index.1.div_euclid(C as i64)))
                // Indexing cannnot fail within a chunk
                .map(|s| &mut s[index.0.rem_euclid(C as i64) as usize][index.1.rem_euclid(C as i64) as usize])
        }

        /// Returns the chunk of the given index
        /// - Note that this function is not specific to a specific instant
        pub fn chunk_of(index : (i64, i64)) -> (i64, i64) {
            (index.0.div_euclid(C as i64), index.1.div_euclid(C as i64))
        }

        /// Returns the array index for the given index
        pub fn array_index_of(index : (i64, i64)) -> (usize, usize) {
            (index.0.rem_euclid(C as i64) as usize, index.1.rem_euclid(C as i64) as usize)
        }
        
        pub fn to_chunked_index(index : (i64, i64)) -> ((i64, i64), (usize, usize)) {
            (Self::chunk_of(index), Self::array_index_of(index))
        }

        /// Checks whether the given index is currently in bounds
        pub fn in_bounds(&self, index : &(i64, i64)) -> bool {
            (index.0 < (self._x_chunks.1 * (C as i64))) && (index.0 >= (self._x_chunks.0 * (C as i64))) &&
            (index.1 < (self._y_chunks.1 * (C as i64))) && (index.1 >= (self._y_chunks.0 * (C as i64)))
        }

        pub fn len_chunks(&self) -> usize {
            ((self._x_chunks.1 - self._x_chunks.0) * (self._y_chunks.1 - self._y_chunks.0)) as usize
        }

        pub fn len(&self) -> usize {
            self.len_chunks() * C * C
        }
    /* */

    /* Dimensions */
        /// Returns the size of the rectangular map for each size `(x, y)` in chunks
        pub fn dim_chunks(&self) -> (usize, usize) {
            (
                (self._x_chunks.1 - self._x_chunks.0) as usize,
                (self._y_chunks.1 - self._y_chunks.0) as usize
            )
        }

        /// Returns the size of the rectangular map for each size `(x, y)` in values (`chunks * C`)
        pub fn dim(&self) -> (usize, usize) {
            (
                (self._x_chunks.1 - self._x_chunks.0) as usize * C,
                (self._y_chunks.1 - self._y_chunks.0) as usize * C
            )
        }

        pub fn x_chunks(&self) -> (i64, i64) {
            self._x_chunks
        }

        pub fn y_chunks(&self) -> (i64, i64) {
            self._y_chunks
        }
        
        pub fn x_dim(&self) -> (i64, i64) {
            (self._x_chunks.0 * (C as i64), self._x_chunks.1 * (C as i64))
        }

        pub fn y_dim(&self) -> (i64, i64) { 
            (self._y_chunks.0 * (C as i64), self._y_chunks.1 * (C as i64))
        }

        // Ranges
        pub fn x_range(&self) -> Range<i64> {
            (self._x_chunks.0 * (C as i64)) .. (self._x_chunks.1 * (C as i64))
        }

        pub fn y_range(&self) -> Range<i64> {
            (self._y_chunks.0 * (C as i64)) .. (self._y_chunks.1 * (C as i64))
        }
    /**/
}

/* Indexing */
impl<T, const C : usize> Index<(i64, i64)> for RectChunkedMap<T, C> {
    type Output = T;

    fn index(&self, index: (i64, i64)) -> &Self::Output {
        self.get(&index)
            .expect(format!("Index {:?} is not present in the chunked map! (X: {:?}, Y: {:?})", index, self.x_dim(), self.y_dim()).as_str())
    }
}

impl<T, const C : usize> IndexMut<(i64, i64)> for RectChunkedMap<T, C> {
    fn index_mut(&mut self, index: (i64, i64)) -> &mut Self::Output {
        self.get_mut(&index)
            .expect(format!("Index {:?} is not present in the chunked map!", index).as_str())
    }
}

impl<T, const C : usize> Index<(usize, usize)> for RectChunkedMap<T, C> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.index(
            (index.0 as i64 + self._x_chunks.0 * (C as i64), index.1 as i64 + self._y_chunks.0 * (C as i64))
        )
    }
}

impl<T, const C : usize> IndexMut<(usize, usize)> for RectChunkedMap<T, C> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.index_mut(
            (index.0 as i64 + self._x_chunks.0 * (C as i64), index.1 as i64 + self._y_chunks.0 * (C as i64))
        )
    }
}

/* Iteration */
impl<T : Copy, const C : usize> Iterator for RectChunkedMap<T, C> {
    type Item = ((i64, i64), T);

    fn next(&mut self) -> Option<Self::Item> {
        // Reposition iterator
        if self.__iter_pos.0 > self._x_chunks.1 {
            self.__iter_pos.0 = self._x_chunks.0;
            self.__iter_pos.1 += 1;
        }

        if self.__iter_pos.1 > self._y_chunks.1 {
            self.__iter_pos.1 = self._y_chunks.0;
            return None;
        }

        // Reindexing as ownership is required
        let val = self.dict[&(self.__iter_pos.0.div_euclid(C as i64), self.__iter_pos.1.div_euclid(C as i64))]
                [self.__iter_pos.0.rem_euclid(C as i64) as usize][self.__iter_pos.1.rem_euclid(C as i64) as usize];
        let ret_val = (self.__iter_pos, val);

        self.__iter_pos.0 += 1;

        Some(ret_val)
    }
}


/* Other */
impl<T : Clone, const C : usize> Clone for RectChunkedMap<T, C> {
    fn clone(&self) -> Self {
        Self {
            dict: self.dict.clone(),
            _x_chunks: self._x_chunks.clone(),
            _y_chunks: self._x_chunks.clone(),
            __iter_pos: self.__iter_pos.clone(),
        }
    }
}

impl<T : Default, const C : usize> Default for RectChunkedMap<T, C> {
    fn default() -> Self {
        Self {
            dict: Default::default(),
            _x_chunks: Default::default(),
            _y_chunks: Default::default(),
            __iter_pos: Default::default()
        }
    }
}