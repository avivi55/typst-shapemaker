use crate::color_mapping::ColorMapping;
use crate::shape::Shape;
use crate::shape::*;
use crate::color::*;
use std::ops::Range;
use rand::Rng;


#[derive(Debug, Clone)]
pub struct Canvas {
    pub grid_size: (usize, usize),
    pub cell_size: usize,
    pub objects_count_range: Range<usize>,
    pub polygon_vertices_range: Range<usize>,
    pub canvas_outter_padding: usize,
    pub line_width: f32,
    pub empty_shape_stroke_width: f32,
    pub small_circle_radius: f32,
    pub dot_radius: f32,
    pub render_grid: bool,
    pub colormap: ColorMapping,
}

impl Canvas {
    pub fn default_settings() -> Self {
        Self {
            grid_size: (3, 3),
            cell_size: 50,
            objects_count_range: 3..7,
            polygon_vertices_range: 2..7,
            canvas_outter_padding: 10,
            line_width: 2.0,
            empty_shape_stroke_width: 0.5,
            small_circle_radius: 5.0,
            dot_radius: 2.0,
            render_grid: false,
            colormap: ColorMapping::default(),
        }
    }
    pub fn random_shape(&self) -> Shape {
        let mut objects: Vec<(Object, Option<Fill>)> = vec![];
        let number_of_objects = rand::thread_rng().gen_range(self.objects_count_range.clone());
        for _ in 0..number_of_objects {
            let object = self.random_object();
            objects.push((
                object,
                if rand::thread_rng().gen_bool(0.5) {
                    Some(self.random_fill())
                } else {
                    None
                },
            ));
        }
        Shape { objects }
    }

    pub fn random_object(&self) -> Object {
        let start = self.random_anchor();
        match rand::thread_rng().gen_range(1..=7) {
            1 => self.random_polygon(),
            2 => Object::BigCircle(self.random_center_anchor()),
            3 => Object::SmallCircle(start),
            4 => Object::Dot(start),
            5 => Object::CurveInward(start, self.random_end_anchor(start)),
            6 => Object::CurveOutward(start, self.random_end_anchor(start)),
            7 => Object::Line(self.random_anchor(), self.random_anchor()),
            _ => unreachable!(),
        }
    }

    pub fn random_end_anchor(&self, start: Anchor) -> Anchor {
        // End anchors are always a square diagonal from the start anchor (for now)
        // that means taking steps of the form n * (one of (1, 1), (1, -1), (-1, 1), (-1, -1))
        // Except that the end anchor needs to stay in the bounds of the shape.

        // Determine all possible end anchors that are in a square diagonal from the start anchor
        let mut possible_end_anchors = vec![];
        let grid_width = self.grid_size.0 as i32;
        let grid_height = self.grid_size.1 as i32;

        for x in -grid_width..=grid_width {
            for y in -grid_height..=grid_height {
                let end_anchor = Anchor(start.0 + x, start.1 + y);

                if end_anchor == start {
                    continue;
                }

                // Check that the end anchor is in a square diagonal from the start anchor and that the end anchor is in bounds
                if x.abs() == y.abs()
                    && end_anchor.0.abs() < grid_width
                    && end_anchor.1.abs() < grid_height
                    && end_anchor.0 >= 0
                    && end_anchor.1 >= 0
                {
                    possible_end_anchors.push(end_anchor);
                }
            }
        }

        // Pick a random end anchor from the possible end anchors
        possible_end_anchors[rand::thread_rng().gen_range(0..possible_end_anchors.len())]
    }

    pub fn random_polygon(&self) -> Object {
        let number_of_anchors = rand::thread_rng().gen_range(self.polygon_vertices_range.clone());
        let start = self.random_anchor();
        let mut lines: Vec<Line> = vec![];
        for _ in 0..number_of_anchors {
            let next_anchor = self.random_anchor();
            lines.push(self.random_line(next_anchor));
        }
        Object::Polygon(start, lines)
    }

    pub fn random_line(&self, end: Anchor) -> Line {
        match rand::thread_rng().gen_range(1..=3) {
            1 => Line::Line(end),
            2 => Line::InwardCurve(end),
            3 => Line::OutwardCurve(end),
            _ => unreachable!(),
        }
    }

    pub fn random_anchor(&self) -> Anchor {
        if rand::thread_rng().gen_bool(1.0 / (self.grid_size.0 * self.grid_size.1) as f64) {
            // small change of getting center (-1, -1) even when grid size would not permit it (e.g. 4x4)
            Anchor(-1, -1)
        } else {
            Anchor(
                rand::thread_rng().gen_range(0..=self.grid_size.0 - 1) as i32,
                rand::thread_rng().gen_range(0..=self.grid_size.1 - 1) as i32,
            )
        }
    }

    pub fn random_center_anchor(&self) -> CenterAnchor {
        if rand::thread_rng()
            .gen_bool(1.0 / ((self.grid_size.0 as i32 - 1) * (self.grid_size.1 as i32 - 1)) as f64)
        {
            // small change of getting center (-1, -1) even when grid size would not permit it (e.g. 3x3)
            CenterAnchor(-1, -1)
        } else {
            CenterAnchor(
                rand::thread_rng().gen_range(0..=self.grid_size.0 - 2) as i32,
                rand::thread_rng().gen_range(0..=self.grid_size.1 - 2) as i32,
            )
        }
    }

    pub fn random_fill(&self) -> Fill {
        Fill::Solid(self.random_color())
        // match rand::thread_rng().gen_range(1..=3) {
        //     1 => Fill::Solid(random_color()),
        //     2 => Fill::Hatched,
        //     3 => Fill::Dotted,
        //     _ => unreachable!(),
        // }
    }

    pub fn random_color(&self) -> Color {
        match rand::thread_rng().gen_range(1..=12) {
            1 => Color::Black,
            2 => Color::White,
            3 => Color::Red,
            4 => Color::Green,
            5 => Color::Blue,
            6 => Color::Yellow,
            7 => Color::Orange,
            8 => Color::Purple,
            9 => Color::Brown,
            10 => Color::Pink,
            11 => Color::Gray,
            12 => Color::Cyan,
            _ => unreachable!(),
        }
    }
}
