//! Compile-time tree-shaken [Lucide](https://lucide.dev) icons for the
//! [Iced](https://iced.rs) GUI framework.
//!
//! Only the icons listed in [`build.toml`](https://docs.rs/lucide-iced/latest/lucide_iced/#build-config)
//! are embedded into your binary. Each generates a type-safe function that returns
//! an [`iced::widget::Svg`] widget.
//!
//! # Build config
//!
//! Edit `build.toml` in the crate root to choose which icons to include:
//!
//! ```toml
//! icons = ["heart", "settings", "trash-2"]
//! ```
//!
//! # Usage
//!
//! ```no_run
//! use iced::widget::svg;
//!
//! fn view() -> iced::Element<'static, ()> {
//!     lucide_iced::icon::heart().into()
//! }
//! ```
//!
//! # Custom icons
//!
//! To render your own SVG through the same code path, use [`svg_from_bytes`]:
//!
//! ```no_run
//! let svg = lucide_iced::svg_from_bytes(b"<svg .../>".as_slice());
//! ```
//!
//! # Font rendering
//!
//! Enable the `font` feature to bundle the Lucide TTF and render icons as text
//! glyphs. Register the font with Iced, then use the glyph codepoints.

#![forbid(unsafe_code)]

use iced::advanced::svg::Handle;
use iced::widget::Svg;

/// The generated per-icon functions.
///
/// Each function is named after its Lucide icon (kebab-case converted to
/// snake_case) and returns an [`iced::widget::Svg`] widget.
pub mod icon {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

/// Builds an [`iced::widget::Svg`] widget from arbitrary SVG bytes.
///
/// This lets you render your own custom SVGs through the same rendering path as
/// the generated Lucide icons, without them being part of the generated set.
///
/// # Example
///
/// ```no_run
/// let svg = lucide_iced::svg_from_bytes(b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>".as_slice());
/// ```
pub fn svg_from_bytes(bytes: &'static [u8]) -> Svg<'static> {
    Svg::new(Handle::from_memory(bytes))
}

/// The Lucide icon font bytes, available when the `font` feature is enabled.
///
/// Register it with Iced to render icons as text glyphs:
///
/// ```no_run
/// # #[cfg(feature = "font")]
/// let settings = iced::Settings {
///     fonts: vec![lucide_iced::LUCIDE_FONT_BYTES.into()],
///     ..Default::default()
/// };
/// ```
#[cfg(feature = "font")]
pub const LUCIDE_FONT_BYTES: &[u8] = include_bytes!("../fonts/lucide.ttf");

#[cfg(test)]
mod tests {
    #[test]
    fn generated_icons_construct() {
        let _ = crate::icon::heart();
        let _ = crate::icon::settings();
        let _ = crate::icon::trash_2();
        let _ = crate::icon::user();
        let _ = crate::icon::search();
        let _ = crate::icon::star();
    }

    #[test]
    fn svg_from_bytes_constructs() {
        let svg = crate::svg_from_bytes(
            b"<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><circle cx='12' cy='12' r='10'/></svg>"
                .as_slice(),
        );
        let _ = svg;
    }
}
