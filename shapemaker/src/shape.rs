use crate::canvas::Canvas;
use crate::color::*;


#[derive(Debug)]
pub enum Object {
    Polygon(Anchor, Vec<Line>),
    Line(Anchor, Anchor),
    CurveOutward(Anchor, Anchor),
    CurveInward(Anchor, Anchor),
    SmallCircle(Anchor),
    Dot(Anchor),
    BigCircle(CenterAnchor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anchor(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CenterAnchor(pub i32, pub i32);

pub trait Coordinates {
    fn coords(&self, canvas: &Canvas) -> (f32, f32);
    fn center() -> Self;
}

impl Coordinates for Anchor {
    fn coords(&self, canvas: &Canvas) -> (f32, f32) {
        match self {
            Anchor(-1, -1) => (canvas.cell_size as f32 / 2.0, canvas.cell_size as f32 / 2.0),
            Anchor(i, j) => {
                let x = (i * canvas.cell_size as i32) as f32;
                let y = (j * canvas.cell_size as i32) as f32;
                (x, y)
            }
        }
    }

    fn center() -> Self {
        Anchor(-1, -1)
    }
}

impl Coordinates for CenterAnchor {
    fn coords(&self, canvas: &Canvas) -> (f32, f32) {
        match self {
            CenterAnchor(-1, -1) => ((canvas.cell_size / 2) as f32, (canvas.cell_size / 2) as f32),
            CenterAnchor(i, j) => {
                let x = *i as f32 * canvas.cell_size as f32 + canvas.cell_size as f32 / 2.0;
                let y = *j as f32 * canvas.cell_size as f32 + canvas.cell_size as f32 / 2.0;
                (x, y)
            }
        }
    }

    fn center() -> Self {
        CenterAnchor(-1, -1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Line(Anchor),
    InwardCurve(Anchor),
    OutwardCurve(Anchor),
}



#[derive(Debug)]
pub struct Shape {
    pub objects: Vec<(Object, Option<Fill>)>,
}

impl Shape {
    pub fn render(self, canvas: &Canvas) -> String {
        let canvas_width =
            canvas.cell_size * (canvas.grid_size.0 - 1) + 2 * canvas.canvas_outter_padding;
        let canvas_height =
            canvas.cell_size * (canvas.grid_size.1 - 1) + 2 * canvas.canvas_outter_padding;
        let default_color = Color::Black.to_string(&canvas.colormap);
        let background_color = canvas.colormap.background.clone();
        eprintln!("render: background_color({:?})", background_color);
        let mut svg = svg::Document::new().add(
            svg::node::element::Rectangle::new()
                .set("x", -(canvas.canvas_outter_padding as i32))
                .set("y", -(canvas.canvas_outter_padding as i32))
                .set("width", canvas_width)
                .set("height", canvas_height)
                .set("fill", background_color),
        );
        for (object, maybe_fill) in self.objects {
            let mut group = svg::node::element::Group::new();
            match object {
                Object::Polygon(start, lines) => {
                    eprintln!("render: polygon({:?}, {:?})", start, lines);
                    let mut path = svg::node::element::path::Data::new();
                    path = path.move_to(start.coords(&canvas));
                    for line in lines {
                        path = match line {
                            Line::Line(end) | Line::InwardCurve(end) | Line::OutwardCurve(end) => {
                                path.line_to(end.coords(&canvas))
                            }
                        };
                    }
                    path = path.close();
                    group = group
                        .add(svg::node::element::Path::new().set("d", path))
                        .set(
                            "style",
                            match maybe_fill {
                                // TODO
                                Some(Fill::Solid(color)) => {
                                    format!("fill: {};", color.to_string(&canvas.colormap))
                                }
                                _ => format!(
                                    "fill: none; stroke: {}; stroke-width: {}px;",
                                    default_color, canvas.empty_shape_stroke_width
                                ),
                            },
                        );
                }
                Object::Line(start, end) => {
                    eprintln!("render: line({:?}, {:?})", start, end);
                    group = group.add(
                        svg::node::element::Line::new()
                            .set("x1", start.coords(&canvas).0)
                            .set("y1", start.coords(&canvas).1)
                            .set("x2", end.coords(&canvas).0)
                            .set("y2", end.coords(&canvas).1)
                            .set(
                                "style",
                                match maybe_fill {
                                    // TODO
                                    Some(Fill::Solid(color)) => {
                                        format!(
                                            "fill: none; stroke: {}; stroke-width: 2px;",
                                            color.to_string(&canvas.colormap)
                                        )
                                    }
                                    _ => format!(
                                        "fill: none; stroke: {}; stroke-width: 2px;",
                                        default_color
                                    ),
                                },
                            ),
                    );
                }
                Object::CurveInward(start, end) | Object::CurveOutward(start, end) => {
                    let inward = if matches!(object, Object::CurveInward(_, _)) {
                        eprintln!("render: curve_inward({:?}, {:?})", start, end);
                        true
                    } else {
                        eprintln!("render: curve_outward({:?}, {:?})", start, end);
                        false
                    };

                    let (start_x, start_y) = start.coords(&canvas);
                    let (end_x, end_y) = end.coords(&canvas);

                    let midpoint = ((start_x + end_x) / 2.0, (start_y + end_y) / 2.0);
                    let start_from_midpoint = (start_x - midpoint.0, start_y - midpoint.1);
                    let end_from_midpoint = (end_x - midpoint.0, end_y - midpoint.1);
                    eprintln!("        midpoint: {:?}", midpoint);
                    eprintln!(
                        "        from midpoint: {:?} -> {:?}",
                        start_from_midpoint, end_from_midpoint
                    );
                    let control = {
                        let relative = (end_x - start_x, end_y - start_y);
                        eprintln!("        relative: {:?}", relative);
                        // diagonal line is going like this: \
                        if start_from_midpoint.0 * start_from_midpoint.1 > 0.0
                            && end_from_midpoint.0 * end_from_midpoint.1 > 0.0
                        {
                            eprintln!("        diagonal \\");
                            if inward {
                                (
                                    midpoint.0 + relative.0.abs() / 2.0,
                                    midpoint.1 - relative.1.abs() / 2.0,
                                )
                            } else {
                                (
                                    midpoint.0 - relative.0.abs() / 2.0,
                                    midpoint.1 + relative.1.abs() / 2.0,
                                )
                            }
                        // diagonal line is going like this: /
                        } else if start_from_midpoint.0 * start_from_midpoint.1 < 0.0
                            && end_from_midpoint.0 * end_from_midpoint.1 < 0.0
                        {
                            eprintln!("        diagonal /");
                            if inward {
                                (
                                    midpoint.0 - relative.0.abs() / 2.0,
                                    midpoint.1 - relative.1.abs() / 2.0,
                                )
                            } else {
                                (
                                    midpoint.0 + relative.0.abs() / 2.0,
                                    midpoint.1 + relative.1.abs() / 2.0,
                                )
                            }
                        // line is horizontal
                        } else if start_y == end_y {
                            eprintln!("        horizontal");
                            (
                                midpoint.0,
                                midpoint.1
                                    + (if inward { -1.0 } else { 1.0 }) * relative.0.abs() / 2.0,
                            )
                        // line is vertical
                        } else if start_x == end_x {
                            eprintln!("        vertical");
                            (
                                midpoint.0
                                    + (if inward { -1.0 } else { 1.0 }) * relative.1.abs() / 2.0,
                                midpoint.1,
                            )
                        } else {
                            unreachable!()
                        }
                    };
                    eprintln!("        control: {:?}", control);
                    group = group.add(
                        svg::node::element::Path::new()
                            .set(
                                "d",
                                svg::node::element::path::Data::new()
                                    .move_to(start.coords(&canvas))
                                    .quadratic_curve_to((control, end.coords(&canvas))),
                            )
                            .set(
                                "style",
                                match maybe_fill {
                                    // TODO
                                    Some(Fill::Solid(color)) => {
                                        format!(
                                            "fill: none; stroke: {}; stroke-width: {}px;",
                                            color.to_string(&canvas.colormap),
                                            canvas.line_width
                                        )
                                    }
                                    _ => format!(
                                        "fill: none; stroke: {}; stroke-width: {}px;",
                                        default_color, canvas.line_width
                                    ),
                                },
                            ),
                    );
                }
                Object::SmallCircle(center) => {
                    eprintln!("render: small_circle({:?})", center);
                    group = group.add(
                        svg::node::element::Circle::new()
                            .set("cx", center.coords(&canvas).0)
                            .set("cy", center.coords(&canvas).1)
                            .set("r", canvas.small_circle_radius)
                            .set(
                                "style",
                                match maybe_fill {
                                    // TODO
                                    Some(Fill::Solid(color)) => {
                                        format!("fill: {};", color.to_string(&canvas.colormap))
                                    }
                                    _ => format!(
                                        "fill: none; stroke: {}; stroke-width: {}px;",
                                        default_color, canvas.empty_shape_stroke_width
                                    ),
                                },
                            ),
                    );
                }
                Object::Dot(center) => {
                    eprintln!("render: dot({:?})", center);
                    group = group.add(
                        svg::node::element::Circle::new()
                            .set("cx", center.coords(&canvas).0)
                            .set("cy", center.coords(&canvas).1)
                            .set("r", canvas.dot_radius)
                            .set(
                                "style",
                                match maybe_fill {
                                    // TODO
                                    Some(Fill::Solid(color)) => {
                                        format!("fill: {};", color.to_string(&canvas.colormap))
                                    }
                                    _ => format!(
                                        "fill: none; stroke: {}; stroke-width: {}px;",
                                        default_color, canvas.empty_shape_stroke_width
                                    ),
                                },
                            ),
                    );
                }
                Object::BigCircle(center) => {
                    eprintln!("render: big_circle({:?})", center);
                    group = group.add(
                        svg::node::element::Circle::new()
                            .set("cx", center.coords(&canvas).0)
                            .set("cy", center.coords(&canvas).1)
                            .set("r", canvas.cell_size / 2)
                            .set(
                                "style",
                                match maybe_fill {
                                    // TODO
                                    Some(Fill::Solid(color)) => {
                                        format!("fill: {};", color.to_string(&canvas.colormap))
                                    }
                                    _ => format!(
                                        "fill: none; stroke: {}; stroke-width: 0.5px;",
                                        default_color
                                    ),
                                },
                            ),
                    );
                }
            }
            eprintln!("        fill: {:?}", &maybe_fill);
            svg = svg.add(group);
        }
        // render a dotted grid
        if canvas.render_grid {
            for i in 0..canvas.grid_size.0 as i32 {
                for j in 0..canvas.grid_size.1 as i32 {
                    let (x, y) = Anchor(i, j).coords(&canvas);
                    svg = svg.add(
                        svg::node::element::Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", canvas.line_width / 4.0)
                            .set("fill", "#000"),
                    );

                    // if i < canvas.grid_size.0 as i32 - 1 && j < canvas.grid_size.1 as i32 - 1 {
                    //     let (x, y) = CenterAnchor(i, j).coords(&canvas);
                    //     svg = svg.add(
                    //         svg::node::element::Circle::new()
                    //             .set("cx", x)
                    //             .set("cy", y)
                    //             .set("r", canvas.line_width / 4.0)
                    //             .set("fill", "#fff"),
                    //     );
                    // }
                }
            }
        }
        svg.set(
            "viewBox",
            format!(
                "{0} {0} {1} {2}",
                -(canvas.canvas_outter_padding as i32),
                canvas_width,
                canvas_height
            ),
        )
        .set("width", canvas_width)
        .set("height", canvas_height)
        .to_string()
    }
}
