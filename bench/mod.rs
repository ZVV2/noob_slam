use std::fs;
use std::time::Instant;

use glam::Vec2;
use noob_slam_lib::VectorDPMap2;

/* Submodules */
mod occup_map;

#[test]
fn vecmap_score_2d() {
    let ref_map = VectorDPMap2::from_vec(
        noob_slam_gen::gen_map_1()
    );
    let input_map = VectorDPMap2::from_vec(
        noob_slam_gen::gen_map1_snip1()
    );

    // Tests
    println!("> [TEST] Vecmap score - DP-Map Len: {}", ref_map.dp_list.len());

    for radius in (10..=100).step_by(10) {
        let inst = Instant::now();
        let score = noob_slam_lib::vecmap_score_2d(
            &ref_map, &input_map, radius as f32, Vec2::ZERO, noob_slam_lib::score_lim_2d
        );

        println!("| - Radius: {} - Score: {} - Time: {}s", radius, score, inst.elapsed().as_secs_f32());
    }
}

#[test]
fn vecmap_score_map_2d() {
    let ref_map = VectorDPMap2::from_vec(
        noob_slam_gen::gen_map_1()
    );
    let input_map = VectorDPMap2::from_vec(
        noob_slam_gen::gen_map1_snip1()
    );

    // Create folder
    fs::create_dir_all("data/4_vecmap_score_map").unwrap();

    // Tests
    println!("> [TEST] Vecmap score map (limited) - DP-Map Len: {}", ref_map.dp_list.len());

    let grid_size = 20.0;

    let inst = Instant::now();
    let (delta_max, shift_at_max, score_map, base_shift) = noob_slam_lib::vecmap_score_map_2d(
        &ref_map, &input_map, 10.0, grid_size, noob_slam_lib::score_lim_2d
    );

    println!("| - Score: {} - Shift: {} - Time: {}s", delta_max, shift_at_max, inst.elapsed().as_secs_f32());

    for i in 0 .. 10 {
        noob_slam_plt::vecmap_plt_score_map(
            delta_max, &score_map, base_shift, grid_size, 
            format!("data/4_vecmap_score_map/4_vecmap_score_map_lim_{}.svg", i).as_str(), 
            0.8, i as f64 / 3.0
        );
    }

    println!("> [TEST] Vecmap score map (unlimited) - DP-Map Len: {}", ref_map.dp_list.len());

    let grid_size = 20.0;

    let inst = Instant::now();
    let (delta_max, shift_at_max, score_map, base_shift) = noob_slam_lib::vecmap_score_map_2d(
        &ref_map, &input_map, 10.0, grid_size, noob_slam_lib::score_unlim_2d
    );

    println!("| - Score: {} - Shift: {} - Time: {}s", delta_max, shift_at_max, inst.elapsed().as_secs_f32());

    for i in 0 .. 10 {
        noob_slam_plt::vecmap_plt_score_map(
            delta_max, &score_map, base_shift, grid_size, 
            format!("data/4_vecmap_score_map/4_vecmap_score_map_unlim_{}.svg", i).as_str(), 
            0.8, i as f64 / 3.0
        );
    }
}