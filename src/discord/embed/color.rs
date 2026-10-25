//! Color

use crate::discord::error::DiscordError;
use serde::Serialize;

/// Color
#[derive(Serialize)]
pub struct Color(u32);

impl Color {
  /// Black
  pub const BLACK: Self = Self(0x000000);
  /// White
  pub const WHITE: Self = Self(0xFFFFFF);
  /// Red
  pub const RED: Self = Self(0xFF0000);
  /// Green
  pub const GREEN: Self = Self(0x00FF00);
  /// Blue
  pub const BLUE: Self = Self(0x0000FF);
  /// Yellow
  pub const YELLOW: Self = Self(0xFFFF00);
  /// Purple
  pub const PURPLE: Self = Self(0xFF00FF);
  /// Cyan
  pub const CYAN: Self = Self(0x00FFFF);
  /// Magenta
  pub const MAGENTA: Self = Self(0xFF00FF);
  /// Orange
  pub const ORANGE: Self = Self(0xFFA500);

  /// create new color
  /// # Errors
  /// Returns an error if:
  /// - `value` is greater than 0xFFFFFF.
  pub fn new(value: u32) -> Result<Self, DiscordError> {
    if value > 0xFFFFFF {
      return Err(DiscordError::InvalidColor {
        value,
        max: 0xFFFFFF,
      });
    }

    Ok(Self(value))
  }

  /// create new color
  /// # Returns
  /// The decimal number of the color.
  /// # Examples
  /// ```rust
  /// let color = Color::new(0xFF, 0x26, 0x00);
  ///
  /// assert_eq!(color.build(), 0xFF2600);
  /// ```
  pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
    Self(((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
  }
}
