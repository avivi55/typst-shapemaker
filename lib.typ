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
  ),
  snazzy-light: (
    black: "#565869",
    white: "#FFFFFF",
    red: "#FF5C57",
    green: "#2DAE58",
    blue: "#09A1ED",
    yellow: "#F5B900",
    orange: "#CF9C00",
    purple: "#F767BB",
    brown: "#FFAEAC",
    pink: "#FF94D2",
    gray: "#FAFBF9",
    cyan: "#13BBB7",
    background: "#eeeeee"
  )
)

/// [INTERNAL] Get a semi unique seed for the document
/// 
/// -> int: The seed from the document layout
#let get_compilation_seed() = {
  let headings = query(heading)
  let figures = query(figure)
  let equations = query(math.equation)
  
  let base_entropy = headings.len() * 31 + figures.len() * 37 + equations.len() * 41 + 12 * 1
  
  let today = datetime.today()
  let date_entropy = today.year() * 365 + today.ordinal()
  
  return calc.rem(base_entropy * 1009 + date_entropy, 100000)
}

/// Generates a """""random""""" shape with the given parameters
///
/// - width_ratio (int): a stupid parameter, you can control the ratio of the width in relation to the height
/// - cell_size (int): The canvas cell size
/// - canvas_padding (int): The canvas padding
/// - line_width (int): 
/// - small_circle_radius (int): 
/// - dot_radius (int): 
/// - empty_shape_stroke (int): 
/// - render_grid (bool): ?
/// - objects_count (int): 
/// - polygon_vertices (int): 
/// - _seed (int | none): The final seed to pass down to the pulgin
/// - color_theme (dict): The palette to pass down to the plugin (color mapping)
/// - image_options (dict): Other options for the generated image
/// -> image
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

/// Self explanatory
///
/// - number (int): The number of shapes
/// - options (dict): The shape options to pass down
/// -> A grid of shapes
#let shape_strip(number: 1, ..options) = {
  let shapes = range(number)
  shapes = shapes.map(s => shape(..options))

  grid(
    columns: number,
    ..shapes.map(shape => [ #shape ]),
  )
}


/// A 12 color palette generator from a base color.
///
/// - base-color (color): the base color for generating the palette
/// -> dictionnary: The 💅 palette 💅
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
