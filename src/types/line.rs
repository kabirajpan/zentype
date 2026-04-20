/// Information about a single line of shaped text.
/// Used for rendering backgrounds and handling selection/click events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineInfo {
    /// Relative X offset within the container.
    pub x: f32,
    /// Vertical position (baseline) of the line.
    pub y: f32,
    /// Visual width of the text in this line.
    pub width: f32,
}
