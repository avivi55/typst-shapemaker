
use wasm_minimal_protocol::*;
initiate_protocol!();

use ciborium::{de::from_reader, ser::into_writer};

use shapemaker::{
    getrandom_custom::{set_random_seed_from_string},
    canvas::*,
    color_mapping::*,
    Args
};

#[wasm_func]
pub fn svg(
    grid_size: &[u8],
    cell_size: &[u8],
    canvas_padding: &[u8],
    line_width: &[u8],
    small_circle_radius: &[u8],
    dot_radius: &[u8],
    empty_shape_stroke: &[u8],
    render_grid: &[u8],
    objects_count: &[u8],
    polygon_vertices: &[u8],
    seed: &[u8],
    mapping: &[u8],
) -> Vec<u8> {

    set_random_seed_from_string(&String::from_utf8_lossy(seed));
    
    // Helper function to parse &[u8] to various types
    fn parse_bytes_to_string(bytes: &[u8]) -> Result<String, std::str::Utf8Error> {
        std::str::from_utf8(bytes).map(|s| s.to_string())
    }
    
    fn parse_bytes_to_usize(bytes: &[u8]) -> Result<usize, Box<dyn std::error::Error>> {
        let s = std::str::from_utf8(bytes)?;
        Ok(s.parse::<usize>()?)
    }
    
    fn parse_bytes_to_f32(bytes: &[u8]) -> Result<f32, Box<dyn std::error::Error>> {
        let s = std::str::from_utf8(bytes)?;
        Ok(s.parse::<f32>()?)
    }

    fn parse_bytes_to_bool(bytes: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let s = std::str::from_utf8(bytes)?;
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" => Ok(true),
            "false" | "0" | "no" => Ok(false),
            _ => Err(format!("Invalid boolean value: {}", s).into()),
        }
    }
    
    let parsed_grid_size = parse_bytes_to_string(grid_size).unwrap_or_default();
    let parsed_cell_size = parse_bytes_to_usize(cell_size).ok();
    let parsed_canvas_padding = parse_bytes_to_usize(canvas_padding).ok();
    let parsed_line_width = parse_bytes_to_f32(line_width).ok();
    let parsed_small_circle_radius = parse_bytes_to_f32(small_circle_radius).ok();
    let parsed_dot_radius = parse_bytes_to_f32(dot_radius).ok();
    let parsed_empty_shape_stroke = parse_bytes_to_f32(empty_shape_stroke).ok();
    let parsed_render_grid = parse_bytes_to_bool(render_grid).unwrap_or(false);
    let parsed_objects_count = parse_bytes_to_string(objects_count).unwrap_or_default();
    let parsed_polygon_vertices = parse_bytes_to_string(polygon_vertices).unwrap_or_default();

    let args = Args {
        flag_version: false,
        flag_color: vec![],
        flag_colors: None,
        flag_grid_size: if parsed_grid_size.is_empty() { None } else { Some(parsed_grid_size) },
        flag_cell_size: parsed_cell_size,
        flag_canvas_padding: parsed_canvas_padding,
        flag_line_width: parsed_line_width,
        flag_small_circle_radius: parsed_small_circle_radius,
        flag_dot_radius: parsed_dot_radius,
        flag_empty_shape_stroke: parsed_empty_shape_stroke,
        flag_render_grid: parsed_render_grid,
        flag_objects_count: if parsed_objects_count.is_empty() { None } else { Some(parsed_objects_count) },
        flag_polygon_vertices: if parsed_polygon_vertices.is_empty() { None } else { Some(parsed_polygon_vertices) },
    };

    let colormap: ColorMapping = from_reader(mapping).unwrap();

    let mut canvas = Canvas::default_settings();
    set_canvas_settings_from_args(&args, &mut canvas);
    canvas.colormap = colormap.clone();
    
    canvas
        .random_shape()
        .render(&canvas)
        .into_bytes()
}


pub fn set_canvas_settings_from_args(args: &Args, canvas: &mut Canvas) {
    if let Some(dimensions) = &args.flag_grid_size {
        let mut split = dimensions.split('x');
        let width = split.next().unwrap().parse::<usize>().unwrap();
        let height = split.next().unwrap().parse::<usize>().unwrap();
        canvas.grid_size = (width, height);
    }
    if let Some(cell_size) = args.flag_cell_size {
        canvas.cell_size = cell_size;
    }
    if let Some(canvas_padding) = args.flag_canvas_padding {
        canvas.canvas_outter_padding = canvas_padding;
    }
    if let Some(line_width) = args.flag_line_width {
        canvas.line_width = line_width;
    }
    if let Some(small_circle_radius) = args.flag_small_circle_radius {
        canvas.small_circle_radius = small_circle_radius;
    }
    if let Some(dot_radius) = args.flag_dot_radius {
        canvas.dot_radius = dot_radius;
    }
    if let Some(empty_shape_stroke) = args.flag_empty_shape_stroke {
        canvas.empty_shape_stroke_width = empty_shape_stroke;
    }
    if let Some(objects_count) = &args.flag_objects_count {
        let mut split = objects_count.split("..");
        let min = split.next().unwrap().parse::<usize>().unwrap();
        let max = split.next().unwrap().parse::<usize>().unwrap();
        // +1 because the range is exclusive, using ..= raises a type error
        canvas.objects_count_range = min..(max + 1);
    }
    if let Some(polygon_vertices) = &args.flag_polygon_vertices {
        let mut split = polygon_vertices.split("..");
        let min = split.next().unwrap().parse::<usize>().unwrap();
        let max = split.next().unwrap().parse::<usize>().unwrap();
        canvas.polygon_vertices_range = min..(max + 1);
    }
}