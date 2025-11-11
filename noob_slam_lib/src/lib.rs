#[derive(Clone, Debug)]
pub struct MapSettings {
    pub tile_size : f32,
    pub chunk_size : usize,

    /// Defines a base size in chunks using `u64`
    pub base_size : [u64; 2] 
}

#[derive(Clone, Debug, Default)]
pub struct Tile {
    pub prop : f32
}

#[derive(Clone)]
pub struct Chunk {
    pub size : usize,
    pub tile_map : Vec<Vec<Tile>>
}

impl Chunk {
    pub fn with_size(size : usize) -> Self {
        Self {
            size,
            tile_map: vec![vec![Tile::default(); size]; size]
        }
    }   
}

#[derive(Clone)]
pub struct Map {
    pub chunk_map : Vec<Vec<Chunk>>
}

impl Map {
    pub fn from_settings(settings : MapSettings) -> Self {
        Self {
            chunk_map: vec![]
        }
    }
}