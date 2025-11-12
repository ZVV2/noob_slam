#[derive(Clone, Debug)]
pub struct MapSettings {
    pub tile_size : f32,
    pub chunk_size : usize,

    /// Defines a base size in chunks using `u32`
    pub base_size : [u32; 2] 
}

impl MapSettings {
    pub fn chunk_size(&self) -> f32 {
        (self.chunk_size as f32) * self.tile_size
    }
}

#[derive(Clone, Debug, Default)]
pub struct Tile {
    pub prop : f32
}

#[derive(Clone)]
pub struct Chunk {
    pub size : usize,
    pub tile_map : Vec<Vec<Tile>>,

    pub dp_list : Vec<DataPoint>
}

impl Chunk {
    pub fn with_size(size : usize) -> Self {
        Self {
            size,
            tile_map: vec![vec![Tile::default(); size]; size],
            dp_list: Vec::new()
        }
    }   
}

#[derive(Clone)]
pub struct Map {
    pub settings : MapSettings,
    pub origin : [i32;2],
    pub chunk_map : Vec<Vec<Chunk>>
}

impl Map {
    pub fn from_settings(settings : MapSettings) -> Self {
        Self {
            chunk_map: vec![vec![Chunk::with_size(settings.chunk_size)]],

            // Set origin in the middle of the map
            origin: [(settings.base_size[0]/2) as i32, (settings.base_size[1]/2) as i32],

            settings
        }
    }

    pub fn chunk_at_pos(&self, x : f32, y : f32) -> Option<&Chunk> {
        let index_x = (x / self.settings.chunk_size()).floor() as i32 + self.origin[0];
        let index_y = (y / self.settings.chunk_size()).floor() as i32 + self.origin[1];

        if (0 <= index_x) && (index_x < self.chunk_map.len() as i32) {
            if (0 <= index_y) && (index_y < self.chunk_map[index_x as usize].len() as i32) {
                return Some(
                    &self.chunk_map[index_x as usize][index_y as usize]
                );
            }
        }

        None
    }

    // pub fn apply_datapoint(&self, x : f32)
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub pos : [f32; 2],
    /// Accuracy factor, recommended between 1-5
    pub f_acc : f32
}