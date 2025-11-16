use std::time::Instant;

use noob_slam_lib::{OccupMap, OccupMapSettings, simple_correlation_2d};
use noob_slam_plt::{PlotSettings, quick_plot_dual, quick_plot_single};

/// This test performs some downsampling and looks at the results generated
#[test]
fn sample_down() {
    let dp_list = noob_slam_gen::gen_map_1();
    let mut map = OccupMap::from_settings(OccupMapSettings {
        base_size: (400, 400),
        ..Default::default()
    });

    map.apply_datapoint_list(dp_list);

    // Tests
    println!("> [TEST] Sampling down maps - Size: {}", map.tile_map.len());

    quick_plot_single(&map, "data/1_sample_down/1_sample_down_f1.png", PlotSettings::default()).unwrap();

    for factor in 2 ..=10 {
        let inst = Instant::now();
        let new_map = map.sample_down_i(factor);

        println!("| - (Factor {}): Size: {} - {}s", factor, new_map.tile_map.len(), inst.elapsed().as_secs_f32());

        quick_plot_single(&new_map, format!("data/1_sample_down/1_sample_down_f{}.png", factor).as_str(), PlotSettings::default()).unwrap();
    }
}

#[test]
fn correlation_tile_grid_vs_sample_down() {
    let mut ref_map = OccupMap::from_settings(OccupMapSettings {
        base_size: (400, 400),
        ..Default::default()
    });

    ref_map.apply_datapoint_list(
        noob_slam_gen::gen_map_1()
    );

    // Snippet 1
    println!("> [TEST] Correlation comparison - Map snippet 1");

    let mut input_map = OccupMap::from_settings(OccupMapSettings {
        base_size: (200, 200),
        ..Default::default()
    });

    input_map.apply_datapoint_list(
        noob_slam_gen::gen_map_snip1()
    );

    for factor in (4..=10).step_by(2) {
        // Down-Sample
        let inst_ds = Instant::now();
        let new_ref_map = ref_map.sample_down_i(factor);
        let new_input_map = input_map.sample_down_i(factor);

        let (delta_ds, x_ds, y_ds) = simple_correlation_2d(&new_input_map, &new_ref_map, 1);

        let dur_ds = inst_ds.elapsed();

        // Tile-Grid
        let inst_tg = Instant::now();
        let (delta_tg, x_tg, y_tg) = simple_correlation_2d(&input_map, &ref_map, factor);
        let dur_tg = inst_tg.elapsed();

        println!("| - (Factor {})", factor);
        println!("| | -> DS: {} - X: {} - Y: {} - Time: {}s", delta_ds, x_ds, y_ds, dur_ds.as_secs_f32());
        println!("| | -> TG: {} - X: {} - Y: {} - Time: {}s", delta_tg, x_tg, y_tg, dur_tg.as_secs_f32());

        quick_plot_dual(&new_ref_map, &new_input_map, x_ds, y_ds, format!("data/2_correlation/2_correlation_snip1_ds_f{}.png", factor).as_str(), PlotSettings::default()).unwrap();
    }

    // Snippet 2 - More imperfections
    println!("> [TEST] Correlation comparison - Map snippet 2");

    let mut input_map = OccupMap::from_settings(OccupMapSettings {
        base_size: (188, 195),
        ..Default::default()
    });

    input_map.apply_datapoint_list(
        noob_slam_gen::gen_map_snip2()
    );

    for factor in (4..=10).step_by(2) {
        // Down-Sample
        let inst_ds = Instant::now();
        let new_ref_map = ref_map.sample_down_i(factor);
        let new_input_map = input_map.sample_down_i(factor);

        let (delta_ds, x_ds, y_ds) = simple_correlation_2d(&new_input_map, &new_ref_map, 1);

        let dur_ds = inst_ds.elapsed();

        // Tile-Grid
        let inst_tg = Instant::now();
        let (delta_tg, x_tg, y_tg) = simple_correlation_2d(&input_map, &ref_map, factor);
        let dur_tg = inst_tg.elapsed();

        println!("| - (Factor {})", factor);
        println!("| | -> DS: {} - X: {} - Y: {} - Time: {}s", delta_ds, x_ds, y_ds, dur_ds.as_secs_f32());
        println!("| | -> TG: {} - X: {} - Y: {} - Time: {}s", delta_tg, x_tg, y_tg, dur_tg.as_secs_f32());

        quick_plot_dual(&new_ref_map, &new_input_map, x_ds, y_ds, format!("data/2_correlation/2_correlation_snip2_ds_f{}.png", factor).as_str(), PlotSettings::default()).unwrap();
    }
}

#[test]
fn rotation() {
    let mut map = OccupMap::from_settings(OccupMapSettings {
        base_size: (425, 375),
        ..Default::default()
    });

    map.apply_datapoint_list(
        noob_slam_gen::gen_map_1()
    );

    // Tests
    println!("> [TEST] Rotating maps - Size: {}", map.tile_map.len());

    for factor in 1..=9 {
        let angle = factor * 10;
        let inst = Instant::now();
        let new_map = map.sample_down_i(factor).rotate((angle as f32).to_radians());

        println!("| - (Angle {}° - F{}): {}x{} - {}s", angle, factor, new_map.tile_map.dim().0, new_map.tile_map.dim().1, inst.elapsed().as_secs_f32());

        quick_plot_single(&new_map, format!("data/3_rotate/3_rotate_f{}_ang{}.png", factor, angle).as_str(), PlotSettings::default()).unwrap();
    }
}