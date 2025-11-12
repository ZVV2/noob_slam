use noob_slam_lib::*;
use noob_slam_gen::*;

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = SlamMap::from_settings(SlamMapSettings {
        base_size: [400, 400],
        ..Default::default()
    });

    let settings = PlotSettings {
        tile_pixel_width: 2
    };
    
    // let dp_list = gen_line([-400.0, 0.0], [200.0, 250.0], 25);
    // map.apply_datapoint_list(dp_list);

    map.apply_datapoint_list(
        gen_map_1()
    );

    // Dimensions of the output image
    let width = settings.tile_pixel_width * map.tile_map.len() as u32;
    let height = settings.tile_pixel_width * map.tile_map.len() as u32;

    // Number of rows and columns in the grid
    let cols = map.tile_map.len();
    let rows = map.tile_map[0].len();

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
            let r = map.tile_map[col][rows - row - 1].prop;

            // if map.tile_map[col][row].prop > 0.0 {
            //     println!("> [{} {}] {}", col, row, map.tile_map[col][row].prop);
            // }
            
            let g = 0.5f32;
            let b = 0.5f32;
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