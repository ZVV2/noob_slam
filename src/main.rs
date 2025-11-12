use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dimensions of the output image
    let width = 500;
    let height = 500;

    // Number of rows and columns in the grid
    let rows = 10;
    let cols = 10;

    // Create the drawing area (bitmap backend)
    let root = BitMapBackend::new("grid.png", (width, height)).into_drawing_area();
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

            // Choose a color based on row/col (for example)
            // here I just use a simple gradient
            let r = (row as f64 / (rows - 1) as f64) as f32;
            let g = (col as f64 / (cols - 1) as f64) as f32;
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
    println!("Result saved to grid.png");

    Ok(())
}