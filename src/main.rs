use std::time::Instant;

use noob_slam_lib::*;
use noob_slam_gen::*;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ref_map = OccupMap::from_settings(OccupMapSettings {
        base_size: [400, 400],
        ..Default::default()
    });

    let settings = PlotSettings {
        tile_pixel_width: 2
    };

    ref_map.apply_datapoint_list(
        gen_map_1()
    );

    let mut input_map = OccupMap::from_settings(OccupMapSettings {
        base_size: [200, 200],
        ..Default::default()
    });

    input_map.apply_datapoint_list(
        gen_map_snip1()
    );

    let tile_grid = 5;

    let (delta_min, min_x, min_y) = simple_correlation_2d(&input_map, &ref_map, tile_grid);
    println!("{} - X: {} - Y: {}", delta_min, min_x, min_y);

    for i in 1 ..=10 {
        let inst = Instant::now();

        let (delta_min, min_x, min_y) = simple_correlation_2d(&input_map, &ref_map, i*5);
        println!("[{}] Delta: {} - X: {} - Y: {}", i*5, delta_min, min_x, min_y);
        println!(" => Time: {}s", inst.elapsed().as_secs_f32());
    }

    // Dimensions of the output image
    let width = settings.tile_pixel_width * ref_map.tile_map.len() as u32;
    let height = settings.tile_pixel_width * ref_map.tile_map.len() as u32;

    // Number of rows and columns in the grid
    let cols = ref_map.tile_map.len();
    let rows = ref_map.tile_map[0].len();

    // Create the drawing area (bitmap backend)
    let root = BitMapBackend::new("map.png", (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    // Compute size of each cell
    let cell_width = width as f64 / cols as f64;
    let cell_height = height as f64 / rows as f64;

    // Iterate through each cell and draw a colored rectangle
    for row in 0..rows {
        for col in 0..cols {
            // compute top-left and bottom-right corners
            let x0 = (col as f64 * cell_width) as i32;
            let y0 = (row as f64 * cell_height) as i32;
            let x1 = ((col + 1) as f64 * cell_width) as i32;
            let y1 = ((row + 1) as f64 * cell_height) as i32;

            // Choose color based on row
            let r = ref_map.tile_map[col][rows - row - 1].prop;
            let g = 0.5f32;
            let mut b = 0.0;

            if (min_x * tile_grid) <= col {
                let im_idx_x = col - min_x * tile_grid;

                if im_idx_x < input_map.size_tiles().0 {

                    if (min_y * tile_grid) <= row {
                        let im_idx_y = row - min_y * tile_grid;

                        if im_idx_y < input_map.size_tiles().1 {
                            b = input_map.tile_map[im_idx_x][input_map.size_tiles().1 - im_idx_y - 1].prop;
                        }
                    }
                }
            }
            
            let colour = RGBColor(
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
            );

            // Draw the rectangle filled with this colour
            let rect = Rectangle::new(
                [(x0, y0), (x1, y1)],    
                colour.filled()
            );
            root.draw(&rect)?;
        }
    }

    // Save/output
    root.present()?;
    println!("Result saved to map.png");

    Ok(())
}