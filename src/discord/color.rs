//! Color

/// Color
pub struct Color {
  r: u8,
  g: u8,
  b: u8,
}

impl Color {
  /// create new color
  /// # Arguments
  /// * `r` - red
  /// * `g` - green
  /// * `b` - blue
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  /// build
  /// # Returns
  /// The decimal number of the color.
  /// # Examples
  /// ```rust
  /// let color = Color::new(0xFF, 0x26, 0x00);
  ///
  /// assert_eq!(color.build(), 0xFF2600);
  /// ```
  pub fn build(&self) -> u32 {
    ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
  }
}
