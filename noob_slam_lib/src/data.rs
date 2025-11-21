use glam::Vec2;

#[derive(Clone, Debug)]
pub struct DataPoint2 {
    pub pos : Vec2,
    /// Accuracy factor, recommended between 1-5
    pub f_acc : f32
}
pub struct VectorDPMap2 {
    pub dp_list : Vec<DataPoint2>,
    pub pos_min : Vec2,
    pub pos_max : Vec2
}

impl VectorDPMap2 {
    pub fn from_vec(dp_list : Vec<DataPoint2>) -> Self {
        let mut pos_min = Vec2::ZERO;
        let mut pos_max = Vec2::ZERO;

        for dp in &dp_list {
            pos_min.x = pos_min.x.min(dp.pos.x);
            pos_min.x = pos_min.x.max(dp.pos.x);
            pos_max.y = pos_max.y.min(dp.pos.y);
            pos_max.y = pos_max.y.max(dp.pos.y);
        }

        Self {
            dp_list,
            pos_min,
            pos_max
        }
    }  

    pub fn dim(&self) -> Vec2 {
        self.pos_max - self.pos_min
    }
}

pub fn vecmap_compare_2d(ref_map : &VectorDPMap2, input_map : &VectorDPMap2, dp_radius : f32, shift : Vec2) -> f32 {
    let mut score = 0.0;

    for p_ref in &ref_map.dp_list {
        for p_in in &input_map.dp_list {
            let d_pos = (p_in.pos - p_ref.pos) + shift;
            let d_dist = d_pos.length();
            // Distance considering accuracy factors
            let acc_d = dp_radius * p_ref.f_acc * p_in.f_acc;

            if d_dist < acc_d {
                score += (1.0 - d_dist / acc_d) / p_ref.f_acc / p_in.f_acc;
            }
        }
    }

    score
}

pub fn vecmap_correlate_2d(ref_map : &VectorDPMap2, input_map : &VectorDPMap2, grid_size : f32) {

}