use serde::Deserialize;

pub mod color_mapping;
pub mod canvas;
pub mod shape;
pub mod color;
pub mod getrandom_custom;


#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_version: bool,
    pub flag_color: Vec<String>,
    pub flag_colors: Option<String>,
    pub flag_grid_size: Option<String>,
    pub flag_cell_size: Option<usize>,
    pub flag_canvas_padding: Option<usize>,
    pub flag_line_width: Option<f32>,
    pub flag_small_circle_radius: Option<f32>,
    pub flag_dot_radius: Option<f32>,
    pub flag_empty_shape_stroke: Option<f32>,
    pub flag_render_grid: bool,
    pub flag_objects_count: Option<String>,
    pub flag_polygon_vertices: Option<String>,
}
