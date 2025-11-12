use noob_slam_lib::DataPoint;

pub fn gen_line_test(start : [f32; 2], end : [f32; 2], n_points : usize) -> Vec<DataPoint> {
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