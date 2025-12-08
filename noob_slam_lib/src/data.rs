use glam::Vec2;
use ndarray::Array2;

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
        let mut pos_min = Vec2::MAX;
        let mut pos_max = Vec2::MIN;

        for dp in &dp_list {
            pos_min.x = pos_min.x.min(dp.pos.x);
            pos_max.x = pos_max.x.max(dp.pos.x);
            pos_min.y = pos_min.y.min(dp.pos.y);
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

/* Score functions */
    pub fn score_lim_2d(p_ref : &DataPoint2, p_in : &DataPoint2, dp_radius : f32, shift : Vec2) -> f32 {
        let d_pos = (p_in.pos - p_ref.pos) + shift;
        let d_dist = d_pos.length();
        // Distance considering accuracy factors
        let acc_d = dp_radius * p_ref.f_acc * p_in.f_acc;

        if d_dist < acc_d {
            (1.0 - d_dist / acc_d) / p_ref.f_acc / p_in.f_acc
        } else {
            0.0
        }
    }

    pub fn score_unlim_2d(p_ref : &DataPoint2, p_in : &DataPoint2, dp_radius : f32, shift : Vec2) -> f32 {
        let d_pos = (p_in.pos - p_ref.pos) + shift;
        let d_dist = d_pos.length();

        1.0 / (p_ref.f_acc * p_in.f_acc + d_dist / dp_radius)
    }
/**/

pub fn vecmap_score_2d<S>(ref_map : &VectorDPMap2, input_map : &VectorDPMap2, dp_radius : f32, shift : Vec2, s_f : S) -> f32 
where
    S : Fn(&DataPoint2, &DataPoint2, f32, Vec2) -> f32
{
    let mut score = 0.0;

    for p_ref in &ref_map.dp_list {
        for p_in in &input_map.dp_list {
            score += s_f(p_ref, p_in, dp_radius, shift);
        }
    }

    score
}

pub fn vecmap_derivative_2d<S>(ref_map : &VectorDPMap2, input_map : &VectorDPMap2, dp_radius : f32, shift : Vec2, delta : f32, s_f : S) -> (f32, Vec2) 
where
    S : Fn(&DataPoint2, &DataPoint2, f32, Vec2) -> f32
{
    let s_0 = vecmap_score_2d(ref_map, input_map, dp_radius, shift, &s_f); 
    let s_x = vecmap_score_2d(ref_map, input_map, dp_radius, shift + Vec2::new(delta, 0.0), &s_f);
    let s_y = vecmap_score_2d(ref_map, input_map, dp_radius, shift + Vec2::new(0.0, delta), &s_f);

    ( 
        s_0,
        Vec2::new(
            (s_x - s_0) / delta,
            (s_y - s_0) / delta
        )
    )
}

pub fn vecmap_score_map_2d<S>(ref_map : &VectorDPMap2, input_map : &VectorDPMap2, dp_radius : f32, grid_size : f32, s_f : S) 
    -> (f32, Vec2, Array2<f32>, Vec2) 
where
    S : Fn(&DataPoint2, &DataPoint2, f32, Vec2) -> f32
{
    let ref_dim = ref_map.dim();
    let input_dim = input_map.dim();

    let delta_dim = ref_dim - input_dim;   

    if delta_dim.x < 0.0 {
        panic!("Reference map is smaller than input map! (X-Direction)");
    }

    if delta_dim.y < 0.0 {
        panic!("Reference map is smaller than input map! (Y-Direction");
    }

    let steps_xy = delta_dim / grid_size;
    let mut delta_max = 0.0;
    let mut shift_at_max = Vec2::ZERO;

    let base_shift = ref_map.pos_min - input_map.pos_min;

    let dim = (steps_xy.x.ceil() as usize, steps_xy.y.ceil() as usize);

    let mut arr : Array2<f32> = Array2::zeros(dim);

    for i_x in 0 .. dim.0 {
        for i_y in 0 .. dim.1 {
            let shift = base_shift + Vec2::new( 
                (i_x as f32) * grid_size,
                (i_y as f32) * grid_size
            );

            let delta = vecmap_score_2d(ref_map, input_map, dp_radius, shift, &s_f);

            arr[(i_x, i_y)] = delta;

            if delta > delta_max {
                delta_max = delta;
                shift_at_max = shift;
            }
        }
    }

    (delta_max, shift_at_max, arr, base_shift)
}

pub fn vecmap_newton_iterate_2d<S>(
    ref_map : &VectorDPMap2, input_map : &VectorDPMap2, dp_radius : f32, mut shift_0 : Vec2, delta_min : f32, f_shift : f32, s_f : S
) -> (f32, Vec2, u32) 
where
    S : Fn(&DataPoint2, &DataPoint2, f32, Vec2) -> f32
{
    let mut i = 0;

    loop {
        let (score, delta_shift) = vecmap_derivative_2d(ref_map, input_map, dp_radius, shift_0, 0.1, &s_f);

        i += 1;

        if delta_shift.length() < (delta_min / f_shift) {
            return (score, shift_0, i)
        }

        shift_0 += delta_shift * f_shift;
    }
}