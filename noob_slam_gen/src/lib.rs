use noob_slam_lib::DataPoint;

pub fn gen_line(start : [f32; 2], end : [f32; 2], n_points : usize) -> Vec<DataPoint> {
    let f_r = 2.5;
    let mut point_list = Vec::new();

    let x_step = (end[0] - start[0]) / n_points as f32;
    let y_step = (end[1] - start[1]) / n_points as f32;

    for i in 0 ..=n_points {
        let x_num : f32 = rand::random_range(-1.0 .. 1.0);
        let y_num : f32 = rand::random_range(-1.0 .. 1.0);

        point_list.push(
            DataPoint {
                pos: [
                    start[0] + (i as f32)*x_step + x_num *f_r*10.0,
                    start[1] + (i as f32)*y_step + y_num *f_r*10.0
                ],
                f_acc: 1.0 + f_r*(x_num*x_num + y_num*y_num).sqrt()
            }
        );
    }

    point_list
}

pub fn gen_map<const C : usize>(point_list : [[f32; 2]; C], n_point_list : [usize; C]) -> Vec<DataPoint> {
    let mut dp_list = Vec::new();

    for i in 0 .. (C-1) {
        dp_list.append(&mut gen_line(point_list[i], point_list[i+1], n_point_list[i]));
    }

    dp_list
}

pub const MAP1_P : [[f32; 2]; 7] = [ 
    [-1500.0, -1000.0],
    [-1500.0, 1000.0],
    [-250.0, 1000.0],
    [-250.0, 750.0],
    [250.0, 750.0],
    [250.0, 1000.0],
    [1250.0, 1000.0]
];

pub const MAP1_N : [usize; 7] = [
    50, 40, 15, 20, 10, 30, 0
];

pub fn gen_map_1() -> Vec<DataPoint> {
    gen_map(MAP1_P, MAP1_N)
}

pub const MAP1_SNIP1_P : [[f32; 2]; 4] = [ 
    [-750.0, 1000.0],
    [-250.0, 1000.0],
    [-250.0, 750.0],
    [50.0, 750.0],
];

pub const MAP1_SNIP1_N : [usize; 4] = [
   60, 25, 40, 0
];

pub fn gen_map_snip1() -> Vec<DataPoint> {
    gen_map(MAP1_SNIP1_P, MAP1_SNIP1_N)
}