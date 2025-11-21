use std::time::Instant;

use glam::Vec2;

/* Submodules */
mod occup_map;

#[test]
fn simple_compare() {
    let ref_map = noob_slam_gen::gen_map_1();
    let input_map = noob_slam_gen::gen_map1_snip1();

    // Tests
    println!("> [TEST] Simple comparision - DP-Map Len: {}", ref_map.len());

    for radius in (10..=100).step_by(10) {
        let inst = Instant::now();
        let score = noob_slam_lib::vecmap_compare_2d(&ref_map, &input_map, radius as f32, Vec2::ZERO);

        println!("| - Radius: {} - Score: {} - Time: {}s", radius, score, inst.elapsed().as_secs_f32());
    }
}