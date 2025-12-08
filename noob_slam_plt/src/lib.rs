use glam::Vec2;
use ndarray::Array2;
use noob_slam_lib::*;

use plotters::prelude::*;

#[derive(Clone, Debug)]
pub struct PlotSettings {
    pub tile_pixel_width : usize
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            tile_pixel_width: 1
        }
    }
}

pub fn occup_plt_single(map : &OccupMap, path : &str, settings : PlotSettings) -> Result<(), Box<dyn std::error::Error>> {
    // Number of rows and columns in the grid
    let (cols, rows) = map.tile_map.dim(); 

    // Dimensions of the output image
    let width = settings.tile_pixel_width * cols;
    let height = settings.tile_pixel_width * rows;

    // Create the drawing area (bitmap backend)
    let root  = BitMapBackend::new(path, (width as u32, height as u32)).into_drawing_area();
    // root.fill(&WHITE)?;

    // Iterate through each cell and draw a colored rectangle
    for row in 0..rows {
        for col in 0..cols {
            // Choose color based on row
            let f = 255 - (map.tile_map[(col, row)].prop * 255.0).min(255.0) as u8;
            
            let color = RGBColor(f, f, f);

            // Draw the rectangle filled with this colour
            let rect = Rectangle::new(
                [
                    ((col * settings.tile_pixel_width) as i32, ((rows - row) * settings.tile_pixel_width) as i32), 
                    (((col + 1) * settings.tile_pixel_width) as i32, ((rows - row - 1) * settings.tile_pixel_width) as i32)
                ],    
                color.filled()
            );
            root.draw(&rect)?;
        }
    }

    root.present()?;

    Ok(())
}

pub fn occup_plt_dual(ref_map : &OccupMap, input_map : &OccupMap, offset_x : usize, offset_y : usize, path : &str, settings : PlotSettings) -> Result<(), Box<dyn std::error::Error>> {
    // Number of rows and columns in the grid
    let (cols, rows) = ref_map.tile_map.dim(); 

    // Dimensions of the output image
    let width = settings.tile_pixel_width * cols;
    let height = settings.tile_pixel_width * rows;

    // Create the drawing area (bitmap backend)
    let root  = BitMapBackend::new(path, (width as u32, height as u32)).into_drawing_area();
    // root.fill(&WHITE)?;

    // Iterate through each cell and draw a colored rectangle
    for row in 0..rows {
        for col in 0..cols {
            // Choose color based on row
            let r = ref_map.tile_map[(col, rows - row - 1)].prop;
            let mut b = 0.0;

            if offset_x <= col {
                let im_idx_x = col - offset_x;

                if im_idx_x < input_map.tile_map.dim().0 {
                    if row >= (rows - (input_map.tile_map.dim().1 + offset_y)) {
                        let im_idx_y = row - (rows - (input_map.tile_map.dim().1 + offset_y));

                        if im_idx_y < input_map.tile_map.dim().1 {
                            b = input_map.tile_map[(im_idx_x, input_map.tile_map.dim().1 - im_idx_y - 1)].prop;
                        }
                    }
                }
            }
            
            let color = RGBColor((r * 255.0).min(255.0) as u8, 127, (b * 255.0).min(255.0) as u8);

            // Draw the rectangle filled with this colour
            let rect = Rectangle::new(
                [
                    ((col * settings.tile_pixel_width) as i32, (row * settings.tile_pixel_width) as i32), 
                    (((col + 1) * settings.tile_pixel_width) as i32, ((row + 1) * settings.tile_pixel_width) as i32)
                ],    
                color.filled()
            );
            root.draw(&rect)?;
        }
    }

    root.present()?;

    Ok(())
}

pub fn vecmap_plt_score_map(delta_max : f32, score_map : &Array2<f32>, base_shift : Vec2, grid_size : f32, path : &str, pitch : f64, yaw : f64) 
{
    // Create diagramm
    let area = SVGBackend::new(path, 
        ( score_map.dim().0 as u32 * grid_size.round() as u32, score_map.dim().1 as u32 * grid_size.round() as u32)
    ).into_drawing_area();

    let mut chart = ChartBuilder::on(&area)
        .build_cartesian_3d(
            base_shift.x..(base_shift.x + grid_size*(score_map.dim().0 as f32)),
            0.0..delta_max,
            base_shift.y..(base_shift.y + grid_size*(score_map.dim().1 as f32)),
        ).unwrap();

    chart.with_projection(|mut pb| {
        pb.pitch = pitch;
        pb.yaw = yaw;
        pb.scale = 0.8;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw().unwrap();

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (0..score_map.dim().0).map(|f| f as f32 * grid_size + base_shift.x),
                (0..score_map.dim().1).map(|f| f as f32 * grid_size + base_shift.y),
                |x, y| score_map[(
                    ((x - base_shift.x) / grid_size).round() as usize,
                    ((y - base_shift.y) / grid_size).round() as usize
                )],
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v / delta_max)).into())
        ).unwrap();

    area.present().unwrap();
}