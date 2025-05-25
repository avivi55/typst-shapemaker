#let shapemaker = plugin("typst_shapemaker.wasm")
#let shape_index = counter("shape_index")
#let seed = state("shape_seed", "")

#let color_themes = (
  palenight: (
    black: "#676E95",
    white: "#ffffff",
    red: "#ff5572",
    green: "#a9c77d",
    blue: "#82AAFF",
    yellow: "#FFCB6B",
    orange: "#FFCB6B",
    purple: "#C792EA",
    brown: "#ff5572",
    pink: "#C792EA",
    gray: "#ffffff",
    cyan: "#89DDFF",
    background: "#eeeeee"
  )
)

#let get_compilation_seed() = {
  let headings = query(heading)
  let figures = query(figure)
  let equations = query(math.equation)
  
  // Create entropy from document structure
  let base_entropy = headings.len() * 31 + figures.len() * 37 + equations.len() * 41 + 12 * 1
  
  let today = datetime.today()
  let date_entropy = today.year() * 365 + today.ordinal()
  
  return calc.rem(base_entropy * 1009 + date_entropy, 100000)
}

#let shape(
  width_ratio: 1,
  cell_size: 50,
  canvas_padding: 10,
  line_width: 3,
  small_circle_radius: 15,
  dot_radius: 9,
  empty_shape_stroke: 1,
  render_grid: "false",
  objects_count: "3..4",
  polygon_vertices: "3..5",
  _seed: none,
  color_theme: color_themes.palenight,
  image_options: ()
) = context {
  shape_index.step()

  let seed = {calc.rem(get_compilation_seed() * 7919 + shape_index.get().at(0) * 43227, 1000)}

  let width
  let height

  if (width_ratio < 1) {
    width = 3
    height = int(3 * (1/width_ratio))
  } else {
    width = 3 * width_ratio
    height = 3
  }


  image( format: "svg",
    shapemaker.svg(
      bytes(str(width) + "x" + str(height)),
      bytes(str(cell_size)),
      bytes(str(canvas_padding)),
      bytes(str(line_width)),
      bytes(str(small_circle_radius)),
      bytes(str(dot_radius)),
      bytes(str(empty_shape_stroke)),
      bytes(str(render_grid)),
      bytes(str(objects_count)),
      bytes(str(polygon_vertices)),
      bytes(str(seed) + state("shape_seed").final()),
      cbor.encode(color_theme)
    ),
    ..image_options
  )
}

#let shape_strip(number: 1, ..options) = {
  let shapes = range(number)
  shapes = shapes.map(s => shape(..options))

  grid(
    columns: number,
    ..shapes.map(shape => [ #shape ]),
  )
}

#let generate-palette(base-color) = {
  let rgb_base = rgb(base-color)
  (
    black: rgb_base.darken(30%),
    white: white,
    red: rgb_base.rotate(180deg), // Complementary
    green: rgb_base.rotate(30deg), // Analogous
    blue: rgb_base,
    yellow: rgb_base.rotate(120deg), // Triadic
    orange: rgb_base.rotate(150deg), // Split-Complementary
    purple: rgb_base.rotate(-30deg), // Analogous
    brown: rgb_base.rotate(-120deg), // Triadic
    pink: rgb_base.rotate(-150deg), // Split-Complementary
    gray: rgb_base.desaturate(100%), // Desaturated base
    cyan: rgb_base.lighten(20%),
    background: rgb_base.lighten(95%) // Light background
  ).pairs()
    .map(((k, v)) => (k, v.to-hex()))
    .to-dict()
}
