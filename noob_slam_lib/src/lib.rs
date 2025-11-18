use glam::{Mat2, Vec2};
use ndarray::Array2;

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

#[derive(Clone, Default, Debug)]
pub struct OccupTile {
    pub prop : f32
}

#[derive(Clone)]
pub struct OccupMap {
    pub settings : OccupMapSettings,
    pub origin : (usize, usize),
    /// Usually it's (row, col) for indexing, but as we are creating a map here, we will be using (x, y)
    pub tile_map : Array2<OccupTile>,

    pub dp_list : Vec<DataPoint>,
    pub highest_prop : f32
}

impl OccupMap {
    pub fn from_settings(base_size : (usize, usize), settings : OccupMapSettings) -> Self {
        Self {
            tile_map: Array2::from_elem(base_size, OccupTile::default()),
            origin: (base_size.0/2, base_size.1/2),       // Set origin in the middle of the map
            dp_list : Vec::new(),
            highest_prop: 0.0,

            settings
        }
    }

    pub fn tile_index(&self, pos : (f32, f32)) -> (usize, usize) {
        (
            (pos.0 / self.settings.tile_size).round() as usize + self.origin.0,
            (pos.1 / self.settings.tile_size).round() as usize + self.origin.1
        )
    }

    pub fn tile_index_checked(&self, pos : (f32, f32)) -> Option<(usize, usize)> {
        let x = (pos.0 / self.settings.tile_size).round() as i64 + self.origin.0 as i64;
        let y = (pos.1 / self.settings.tile_size).round() as i64 + self.origin.1 as i64;

        if (0 <= x) && (x < self.tile_map.dim().0 as i64) {
            if (0 <= y) && (y < self.tile_map.dim().1 as i64) {
                return Some((x as usize, y as usize));
            }
        }

        None
    }

    pub fn tile_at_pos(&self, pos : (f32, f32)) -> Option<((usize, usize), &OccupTile)> {
        self.tile_index_checked(pos).map(|idx| (idx, &self.tile_map[idx]))
    }

    pub fn tile_at_pos_mut(&mut self, pos : (f32, f32)) -> Option<((usize, usize), &mut OccupTile)> {
        self.tile_index_checked(pos).map(|idx| (idx, &mut self.tile_map[idx]))
    }

    pub fn apply_datapoint(&mut self, dp : DataPoint) {
        if let Some((index_x, index_y)) = self.tile_index_checked(dp.pos) {
            let delta = self.settings.dp_weight;
            let dp_radius = self.settings.dp_radius * dp.f_acc;
            let dp_idx_radius = (dp_radius / self.settings.tile_size).round() as usize;

            // Relative delta => Delta divided by a "cone"
            let delta_r = delta / (dp_radius * dp_radius * 1.0 * core::f32::consts::PI / 3.0);

            // Creating safe indecies to prevent out of bounds
            let min_idx_x = index_x.checked_sub(dp_idx_radius).unwrap_or(0);
            let max_idx_x = (index_x + dp_idx_radius).min(self.tile_map.dim().0 - 1);

            let min_idx_y = index_y.checked_sub(dp_idx_radius).unwrap_or(0);
            let max_idx_y = (index_y + dp_idx_radius).min(self.tile_map.dim().1 - 1);

            for idx_x in min_idx_x .. max_idx_x {
                for idx_y in min_idx_y .. max_idx_y {
                    let distance = (
                        ((idx_x as f32 - self.origin.0 as f32 + 0.5) * self.settings.tile_size - dp.pos.0).powi(2) +
                        ((idx_y as f32 - self.origin.1 as f32 + 0.5) * self.settings.tile_size - dp.pos.1).powi(2)
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

            let mut new_settings = self.settings.clone();
            new_settings.tile_size *= factor as f32;

            let new_cols = self.tile_map.dim().0 / factor;
            let new_rows = self.tile_map.dim().1 / factor;

            let mut new_tile_map = Array2::from_elem((new_cols, new_rows), OccupTile::default());
            let mut highest_prop : f32 = 0.0;

            for i_x in 0 .. new_cols {
                for i_y in 0 .. new_rows {
                    let mut prop_sum = 0.0;

                    for n_x in 0 .. factor {
                        for n_y in 0 .. factor {
                            prop_sum += self.tile_map[(i_x*factor + n_x, i_y*factor + n_y)].prop;
                        }
                    }

                    new_tile_map[(i_x, i_y)].prop = prop_sum / (factor * factor) as f32;

                    highest_prop = highest_prop.max(new_tile_map[(i_x, i_y)].prop);
                }
            }
            
            Self {
                tile_map: new_tile_map,
                origin: (self.origin.0/factor, self.origin.1/factor),
                highest_prop,
                dp_list: self.dp_list.clone(),
                settings: new_settings
            }
        }

        /// Angle in radians
        pub fn rotate(&self, angle : f32) -> Self {
            let rot_matr = Mat2::from_angle(angle);

            let (tile_x, tile_y) = self.tile_map.dim();
            let (width, height) = self.size();

            let width_rot = rot_matr * Vec2::new(width, 0.0);
            let height_rot = rot_matr * Vec2::new(0.0, height);

            let new_width = width_rot.x.abs() + height_rot.x.abs();
            let new_height = width_rot.y.abs() + height_rot.y.abs();

            let mut new_map = OccupMap::from_settings((
                (new_width / self.settings.tile_size).ceil() as usize, 
                (new_height / self.settings.tile_size).ceil() as usize,
            ), self.settings.clone());
            
            for t_x in 0 .. tile_x {
                for t_y in 0 .. tile_y {
                    let old_tile = &self.tile_map[(t_x, t_y)];
                    let new_tile_pos = rot_matr * Vec2::new(
                        (t_x as f32 - self.origin.0 as f32 + 0.5) * self.settings.tile_size, 
                        (t_y as f32 - self.origin.1 as f32 + 0.5) * self.settings.tile_size
                    );

                    if let Some((_, new_tile)) = new_map.tile_at_pos_mut((new_tile_pos.x, new_tile_pos.y)) {
                        new_tile.prop = old_tile.prop;
                    }
                }
            }

            new_map
        }

        pub fn expand(&mut self, x_neg : usize, x_pos : usize, y_neg : usize, y_pos : usize) {
            let (x, y) = self.tile_map.dim();
            let mut new_tile_map = Array2::from_elem(
                (x_neg + x_pos + x, y_neg + y_pos + y), 
                OccupTile::default()
            );

            new_tile_map.slice_mut(ndarray::s![x_neg..x_neg+x, y_neg..y_neg+y]).assign(&self.tile_map);

            self.tile_map = new_tile_map;
        }
    /**/
}

#[derive(Clone, Debug)]
pub struct DataPoint {
    pub pos : (f32, f32),
    /// Accuracy factor, recommended between 1-5
    pub f_acc : f32
}

/// Expects the same tile size!
pub fn simple_correlation_2d(input_map : &OccupMap, ref_map : &OccupMap, tile_grid : usize) -> (f32, (usize, usize)) {
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

    // Input map size in tiles
    let (im_sizet_x, im_sizet_y) = input_map.tile_map.dim(); 

    for t_x in 0..=x_iter {
        for t_y in 0..=y_iter {
            // Each whole map iteration to see where it lies best
            // t_x and t_y describe the iter progress in the TILE_GRID, to get the amount of tiles in, multiply by `tile_grid`

            // Iter tracking variables
            let mut delta = 0.0;

            for i_x in 0..im_sizet_x {
                for i_y in 0..im_sizet_y {
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

    (delta_min, (t_x_min, t_y_min))
}

/// - tile_grid -> How many tiles should be grouped together (length of tile-square)  
/// - angle_grid -> How many times the 90° are split up
pub fn correlation_trans_rot_2d(input_map : &OccupMap, ref_map : &OccupMap, tile_grid : usize, angle_grid : usize) -> (f32, f32, (usize, usize)) {
    let (input_map_w, input_map_h) = input_map.tile_map.dim();
    let (ref_map_w, ref_map_h) = ref_map.tile_map.dim();

    if input_map_w > ref_map_w {
        panic!("Input map is wider than reference map!");
    }

    if input_map_h > ref_map_h {
        panic!("Input map is higher than reference map!");
    }

    let mut delta_min = f32::INFINITY;
    let mut c_min = (0, 0);
    let mut angle_min = 0.0;

    for angle in (0..360).step_by(90/angle_grid) {
        let rot_map = input_map.rotate((angle as f32).to_radians());

        let (delta, c) = simple_correlation_2d(&rot_map, ref_map, tile_grid);

        if delta < delta_min {
            delta_min = delta;
            c_min = c;
            angle_min = (angle as f32).to_radians();
        }
    }

    (delta_min, angle_min, c_min)
}