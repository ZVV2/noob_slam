use chunked_map::RectChunkedMap;
use glam::{Mat2, Vec2};
#[derive(Clone, Debug)]
pub struct OccupMapSettings {
    pub tile_size : f32,

    /// Orientation value for how much "weight" a datapoint adds to the grid
    pub dp_weight : f32,
    /// Base datapoint radius
    pub dp_radius : f32
}

impl Default for OccupMapSettings {
    fn default() -> Self {
        Self {
            tile_size: 10.0,

            dp_weight: 10.0,
            dp_radius: 25.0
        }
    }
}

impl OccupMapSettings {
    pub fn tile_area(&self) -> f32 {
        self.tile_size * self.tile_size
    }
}

#[derive(Clone, Default, Debug, Copy)]
pub struct OccupTile {
    pub prop : f32
}

#[derive(Clone)]
pub struct OccupMap<const C : usize> {
    pub tile_map : RectChunkedMap<OccupTile, C>,
    pub settings : OccupMapSettings,

    pub dp_list : Vec<DataPoint>,
    pub highest_prop : f32
}

impl<const C : usize> OccupMap<C> {
    pub fn from_settings(base_size : (usize, usize), settings : OccupMapSettings) -> Self {
        Self {
            tile_map: RectChunkedMap::with_size_centered(base_size), 
            dp_list : Vec::new(),
            highest_prop: 0.0,

            settings
        }
    }

    pub fn tile_index(&self, x : f32, y : f32) -> (i64, i64) {
        ((x / self.settings.tile_size).floor() as i64, (y / self.settings.tile_size).floor() as i64)
    }

    // pub fn tile_at_pos(&self, x : f32, y : f32) -> Option<((i64, i64), &OccupTile)> {
    //     let index = ((x / self.settings.tile_size).floor() as i64, (y / self.settings.tile_size).floor() as i64);
    //     self.tile_map.get(&index).map(|val| (index, val))
    // }

    // pub fn tile_at_pos_mut(&mut self, x : f32, y : f32) -> Option<((i64, i64), &mut OccupTile)> {
    //     let index = ((x / self.settings.tile_size).floor() as i64, (y / self.settings.tile_size).floor() as i64);
    //     self.tile_map.get_mut(&index).map(|val| (index, val))
    // }

    pub fn apply_datapoint(&mut self, dp : DataPoint) {
        let index = self.tile_index(dp.pos[0], dp.pos[1]);

        if self.tile_map.in_bounds(&index) {
            let delta = self.settings.dp_weight;
            let dp_radius = self.settings.dp_radius * dp.f_acc;
            let dp_idx_radius = (dp_radius / self.settings.tile_size).round() as i64;

            let (min_x, max_x) = self.tile_map.x_dim();
            let (min_y, max_y) = self.tile_map.y_dim();

            // Relative delta => Delta divided by a "cone"
            let delta_r = delta / (dp_radius * dp_radius * 1.0 * core::f32::consts::PI / 3.0);

            // Creating safe indecies to prevent out of bounds
            let min_idx_x = (index.0 - dp_idx_radius).max(min_x);
            let max_idx_x = (index.0 + dp_idx_radius).min(max_x);

            let min_idx_y = (index.1 - dp_idx_radius).max(min_y);
            let max_idx_y = (index.1 + dp_idx_radius).min(max_y);

            // TODO: Add map size extension

            for idx_x in min_idx_x .. max_idx_x {
                for idx_y in min_idx_y .. max_idx_y {
                    let distance = (
                        ((idx_x as f32 + 0.5) * self.settings.tile_size - dp.pos[0]).powi(2) +
                        ((idx_y as f32 + 0.5) * self.settings.tile_size - dp.pos[1]).powi(2)
                    ).sqrt();

                    let dist_fac = 1.0 - distance / dp_radius;

                    // Check if the datapoint is in range
                    if dist_fac > 0.0 {
                        self.tile_map[(idx_x, idx_y)].prop += delta_r * dist_fac * self.settings.tile_area();
                        // Update highest probability
                        self.highest_prop = self.highest_prop.max(self.tile_map[(idx_x, idx_y)].prop);        
                    }
                }
            }

            self.dp_list.push(dp);
        }

        // TODO: Add automatic size extension
    }

    pub fn apply_datapoint_list(&mut self, dp_list : Vec<DataPoint>) {
        for dp in dp_list {
            self.apply_datapoint(dp);
        }
    }

    pub fn size(&self) -> (f32, f32) {
        let (x, y) = self.tile_map.dim();
        (x as f32 * self.settings.tile_size, y as f32 * self.settings.tile_size)
    }

    /* Modifications */
        pub fn sample_down_i(&self, factor : usize) -> Self {
            if factor == 0 {
                panic!("Down-sampling factor of 1 or smaller is not valid!");
            } else if factor == 1 {
                return self.clone();
            }

            let fac_i = factor as i64;
            let mut new_settings = self.settings.clone();
            new_settings.tile_size *= fac_i as f32;

            let x_dim_old = self.tile_map.x_dim();
            let y_dim_old = self.tile_map.y_dim();

            let mut new_tile_map: RectChunkedMap<OccupTile, C> = RectChunkedMap::with_size(
                (x_dim_old.0 / fac_i, x_dim_old.1 / fac_i), (y_dim_old.0 / fac_i, y_dim_old.1 / fac_i)
            );
            let mut highest_prop : f32 = 0.0;

            for i_x in new_tile_map.x_range() {
                for i_y in new_tile_map.y_range() {
                    let mut prop_sum = 0.0;

                    for n_x in 0 .. fac_i {
                        for n_y in 0 .. fac_i {
                            prop_sum += self.tile_map.get(&(i_x*fac_i + n_x, i_y*fac_i + n_y))
                                .map(|val| *val).unwrap_or_default().prop;
                        }
                    }

                    new_tile_map[(i_x, i_y)].prop = prop_sum / (fac_i * fac_i) as f32;

                    highest_prop = highest_prop.max(new_tile_map[(i_x, i_y)].prop);
                }
            }
            
            Self {
                tile_map: new_tile_map,
                highest_prop,
                dp_list: self.dp_list.clone(),
                settings: new_settings
            }
        }

        /// Angle in radians
        /// - Does not transfer datapoints (TODO)
        pub fn rotate(&self, angle : f32) -> Self {
            let x_dim = self.tile_map.x_dim();
            let y_dim = self.tile_map.y_dim();

            let rot_matr = Mat2::from_angle(angle);

            let size_vectors = [
                rot_matr * Vec2::new(x_dim.1 as f32, y_dim.1 as f32),
                rot_matr * Vec2::new(x_dim.0 as f32, y_dim.1 as f32),
                rot_matr * Vec2::new(x_dim.0 as f32, y_dim.0 as f32),
                rot_matr * Vec2::new(x_dim.1 as f32, y_dim.0 as f32)
            ];

            let mut x_min : f32 = 0.0;
            let mut x_max : f32 = 0.0;
            let mut y_min : f32 = 0.0;
            let mut y_max : f32 = 0.0;

            for v in size_vectors {
                x_min = x_min.min(v.x);
                x_max = x_max.max(v.x);
                y_min = y_min.min(v.y);
                y_max = y_max.max(v.y);
            }

            let mut new_tile_map: RectChunkedMap<OccupTile, C> = RectChunkedMap::with_chunks(
                (x_min.floor() as i64, x_max.ceil() as i64), 
                (y_min.floor() as i64, y_max.ceil() as i64)
            );
            
            for t_x in self.tile_map.x_range() {
                for t_y in self.tile_map.y_range() {
                    let old_tile = self.tile_map.get(&(t_x, t_y)).map(|val| *val).unwrap_or_default();

                    let new_tile_pos = rot_matr * Vec2::new(
                        (t_x as f32 + 0.5) * self.settings.tile_size, 
                        (t_y as f32 + 0.5) * self.settings.tile_size
                    );

                    if let Some(new_tile) = new_tile_map.get_mut(&self.tile_index(new_tile_pos.x, new_tile_pos.y)) {
                        new_tile.prop = old_tile.prop;
                    }
                }
            }

            Self {
                tile_map: new_tile_map,
                dp_list: Vec::new(),
                highest_prop: self.highest_prop,
                settings: self.settings.clone()
            }
        }
    /**/
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub pos : [f32; 2],
    /// Accuracy factor, recommended between 1-5
    pub f_acc : f32
}

/// Expects the same tile size!
pub fn simple_correlation_2d<const C : usize>(input_map : &OccupMap<C>, ref_map : &OccupMap<C>, tile_grid : usize) -> (f32, usize, usize) {
    let (input_map_w, input_map_h) = input_map.tile_map.dim();
    let (ref_map_w, ref_map_h) = ref_map.tile_map.dim();

    if input_map_w > ref_map_w {
        panic!("Input map is wider than reference map!");
    }

    if input_map_h > ref_map_h {
        panic!("Input map is higher than reference map!");
    }

    let x_span = ref_map_w - input_map_w; 
    let y_span = ref_map_h - input_map_h;

    let x_iter = x_span / tile_grid;
    let y_iter = y_span / tile_grid;

    // Tracking variables
    let mut delta_min = f32::INFINITY;
    let mut t_x_min = 0;
    let mut t_y_min = 0;

    for t_x in 0..=x_iter {
        for t_y in 0..=y_iter {
            // Each whole map iteration to see where it lies best
            // t_x and t_y describe the iter progress in the TILE_GRID, to get the amount of tiles in, multiply by `tile_grid`

            // Iter tracking variables
            let mut delta = 0.0;

            for i_x in 0..input_map_w {
                for i_y in 0..input_map_h {
                    let im_tile = &input_map.tile_map[(i_x, i_y)];
                    let rm_tile = &ref_map.tile_map[(t_x*tile_grid + i_x, t_y*tile_grid + i_y)];
                    
                    // TODO: Add proper threshold
                    if im_tile.prop > 0.05 {
                        delta += (im_tile.prop - rm_tile.prop).abs() + (1.0 - im_tile.prop * rm_tile.prop);
                    }
                }
            }

            // Check tracking
            if delta < delta_min {
                delta_min = delta;
                t_x_min = t_x;
                t_y_min = t_y;
            }
        }
    }

    (delta_min, t_x_min, t_y_min)
}

// /// - tile_grid -> How many tiles should be grouped together (length of tile-square)  
// /// - angle_grid -> How many times the 90° are split up
// pub fn correlation_trans_rot_2d(input_map : &OccupMap, ref_map : &OccupMap, tile_grid : usize, angle_grid : usize) -> (f32, f32, usize, usize) {
//     let (input_map_w, input_map_h) = input_map.tile_map.dim();
//     let (ref_map_w, ref_map_h) = ref_map.tile_map.dim();

//     if input_map_w > ref_map_w {
//         panic!("Input map is wider than reference map!");
//     }

//     if input_map_h > ref_map_h {
//         panic!("Input map is higher than reference map!");
//     }

//     for angle in (0..360).step_by(90/angle_grid) {

//     }

//     ()
// }