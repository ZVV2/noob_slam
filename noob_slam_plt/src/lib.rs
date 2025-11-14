use noob_slam_lib::*;

use plotters::prelude::*;

#[derive(Clone, Debug)]
pub struct PlotSettings {
    pub tile_pixel_width : u32
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            tile_pixel_width: 1
        }
    }
}

pub fn quick_plot_single(map : &OccupMap, path : &str, settings : PlotSettings) -> Result<(), Box<dyn std::error::Error>> {
    // Dimensions of the output image
    let width = settings.tile_pixel_width * map.tile_map.dim().0 as u32;
    let height = settings.tile_pixel_width * map.tile_map.dim().1 as u32;

    // Number of rows and columns in the grid
    let (rows, cols) = map.tile_map.dim(); 

    // Create the drawing area (bitmap backend)
    let root  = BitMapBackend::new(path, (width, height)).into_drawing_area();
    // root.fill(&WHITE)?;

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
            let f = 255 - (map.tile_map[(rows - row - 1, col)].prop * 255.0) as u8;
            
            let color = RGBColor(f, f, f);

            // Draw the rectangle filled with this colour
            let rect = Rectangle::new(
                [(x0, y0), (x1, y1)],    
                color.filled()
            );
            root.draw(&rect)?;
        }
    }

    root.present()?;

    Ok(())
}