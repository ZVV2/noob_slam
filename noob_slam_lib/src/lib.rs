#[derive(Clone, Debug)]
pub struct SlamMapSettings {
    pub tile_size : f32,

    /// Orientation value for how much "weight" a datapoint adds to the grid
    pub dp_weight : f32,
    /// Base datapoint radius
    pub dp_radius : f32,

    /// Defines a base size in chunks using `u32`
    pub base_size : [usize; 2] 
}

impl Default for SlamMapSettings {
    fn default() -> Self {
        Self {
            tile_size: 10.0,

            dp_weight: 10.0,
            dp_radius: 25.0,

            base_size: [100, 100]
        }
    }
}

impl SlamMapSettings {
    pub fn tile_area(&self) -> f32 {
        self.tile_size * self.tile_size
    }

    pub fn map_size(&self) -> [f32; 2] {
        [ 
            self.base_size[0] as f32 * self.tile_size, 
            self.base_size[1] as f32 * self.tile_size 
        ]
    }
}

#[derive(Clone, Default, Debug)]
pub struct SlamTile {
    pub prop : f32
}

#[derive(Clone)]
pub struct SlamMap {
    pub settings : SlamMapSettings,
    pub origin : [i32;2],
    pub tile_map : Vec<Vec<SlamTile>>,

    pub dp_list : Vec<DataPoint>,
    pub highest_prop : f32
}

impl SlamMap {
    pub fn from_settings(settings : SlamMapSettings) -> Self {
        Self {
            tile_map: vec![vec![SlamTile::default(); settings.base_size[1]]; settings.base_size[0]],
            origin: [(settings.base_size[0]/2) as i32, (settings.base_size[1]/2) as i32],       // Set origin in the middle of the map
            dp_list : Vec::new(),
            highest_prop: 0.0,

            settings
        }
    }

    pub fn tile_at_pos(&self, x : f32, y : f32) -> Option<(&SlamTile, usize, usize)> {
        let index_x = (x / self.settings.tile_size).round() as i32 + self.origin[0];
        let index_y = (y / self.settings.tile_size).round() as i32 + self.origin[1];

        if (0 <= index_x) && (index_x < self.tile_map.len() as i32) {
            if (0 <= index_y) && (index_y < self.tile_map[index_x as usize].len() as i32) {
                return Some((
                    &self.tile_map[index_x as usize][index_y as usize],
                    index_x as usize,
                    index_y as usize
                ));
            }
        }

        None
    }

    pub fn apply_datapoint(&mut self, dp : DataPoint) {
        if let Some((_tile, index_x, index_y)) = self.tile_at_pos(dp.pos[0], dp.pos[1]) {
            let delta = self.settings.dp_weight;
            let dp_radius = self.settings.dp_radius * dp.f_acc;
            let dp_idx_radius = (dp_radius / self.settings.tile_size).round() as usize;

            // Relative delta => Delta divided by a "cone"
            let delta_r = delta / (dp_radius * dp_radius * 1.0 * core::f32::consts::PI / 3.0);

            // Creating safe indecies to prevent out of bounds
            let min_idx_x = index_x.checked_sub(dp_idx_radius).unwrap_or(0);
            let max_idx_x = (index_x + dp_idx_radius).min(self.tile_map.len() - 1);

            let min_idx_y = index_y.checked_sub(dp_idx_radius).unwrap_or(0);
            let max_idx_y = (index_y + dp_idx_radius).min(self.tile_map[0].len() - 1);

            for idx_x in min_idx_x .. max_idx_x {
                for idx_y in min_idx_y .. max_idx_y {
                    let distance = (
                        ((idx_x as f32 - self.origin[0] as f32 + 0.5) * self.settings.tile_size - dp.pos[0]).powi(2) +
                        ((idx_y as f32 - self.origin[1] as f32 + 0.5) * self.settings.tile_size - dp.pos[1]).powi(2)
                    ).sqrt();

                    let dist_fac = 1.0 - distance / dp_radius;

                    // Check if the datapoint is in range
                    if dist_fac > 0.0 {
                        self.tile_map[idx_x][idx_y].prop += delta_r * dist_fac * self.settings.tile_area();
                        // Update highest probability
                        self.highest_prop = self.highest_prop.max(self.tile_map[idx_x][idx_y].prop);        
                    }
                }
            }

            self.dp_list.push(dp);
        }
    }

    pub fn apply_datapoint_list(&mut self, dp_list : Vec<DataPoint>) {
        for dp in dp_list {
            self.apply_datapoint(dp);
        }
    }
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub pos : [f32; 2],
    /// Accuracy factor, recommended between 1-5
    pub f_acc : f32
}

#[derive(Clone, Debug)]
pub struct PlotSettings {
    pub tile_pixel_width : u32
}