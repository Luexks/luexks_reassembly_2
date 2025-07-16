pub struct PortPosition;

impl PortPosition {
    pub const CURRENT_VERT: f32 = 0.0;
    pub const NEXT_VERT: f32 = 1.0;
    pub const CENTER: f32 = 0.5;
}

pub const PORT_COUNT_DECISION_TOLERANCE: f32 = 0.001;

pub const VERTEX_ORIENTATION_MULTIPLIERS: [(f32, f32); 4] =
    [(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)];
pub const VERTEX_DIAGONAL_ORIENTATION_MULTIPLIERS: [(f32, f32); 4] =
    [(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, -1.0)];

pub const SHAPES_NAME: &str = "shapes.lua";
