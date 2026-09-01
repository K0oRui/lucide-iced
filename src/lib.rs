//! Compile-time tree-shaken [Lucide](https://lucide.dev) icons for the
//! [Iced](https://iced.rs) GUI framework.
//!
//! Only the icons listed in [`build.toml`](https://docs.rs/lucide-iced/latest/lucide_iced/#build-config)
//! are embedded into your binary. Each generates a type-safe function that returns
//! an [`iced::widget::Svg`] widget, plus a constant with the raw SVG bytes for
//! rendering through Iced's mesh path inside custom widgets.
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

/// The generated per-icon functions and raw SVG bytes.
///
/// [`icon`] contains one function per Lucide icon (kebab-case converted to
/// snake_case) that returns an [`iced::widget::Svg`] widget. [`bytes`] contains
/// the raw SVG bytes for each icon, for rendering through Iced's SVG mesh path
/// inside custom widgets.
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

/// The generated per-icon SVG widget functions.
pub use generated::icon;

/// The generated per-icon raw SVG bytes.
pub use generated::bytes;

mod themed;

pub use themed::{themed_icon, ThemedIcon};

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

/// Mirrors raw SVG bytes horizontally around the vertical center of the
/// `viewBox`, returning a new [`iced::widget::Svg`] widget.
///
/// Iced's [`Svg`] widget has no flip/scale-axis API, so this wraps the SVG's
/// inner content in a `<g transform="translate(W,0) scale(-1,1)">` group, where
/// `W` is the width from the `viewBox`. The result is a new SVG widget with the
/// same dimensions but mirrored.
///
/// Prefer [`mirror_bytes`] when you need the raw bytes (e.g. to feed a themed
/// icon widget) rather than a plain [`Svg`] widget.
///
/// # Example
///
/// ```no_run
/// let heart = lucide_iced::mirror_svg(lucide_iced::bytes::HEART);
/// ```
pub fn mirror_svg(bytes: &[u8]) -> Svg<'static> {
    let mirrored = mirror_bytes(bytes);
    Svg::new(Handle::from_memory(mirrored))
}

/// Mirrors raw SVG bytes horizontally around the vertical center of the
/// `viewBox`.
///
/// Returns a new owned byte buffer. The input is not modified.
pub fn mirror_bytes(bytes: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(bytes);
    let view_box = extract_view_box(&text);
    let width = view_box.map(|(w, _)| w).unwrap_or(24.0);

    let inner_start = text.find('>').map(|i| i + 1).unwrap_or(0);
    let inner_end = text.rfind('<').unwrap_or(text.len());

    let mut out = String::with_capacity(text.len() + 64);
    out.push_str(&text[..inner_start]);
    out.push_str(&format!(
        "<g transform=\"translate({width},0) scale(-1,1)\">"
    ));
    out.push_str(&text[inner_start..inner_end]);
    out.push_str("</g>");
    out.push_str(&text[inner_end..]);
    out.into_bytes()
}

/// Extracts the `(width, height)` from an SVG `viewBox="x y w h"` attribute.
fn extract_view_box(text: &str) -> Option<(f32, f32)> {
    let attr = text.find("viewBox")?;
    let rest = &text[attr..];
    let open = rest.find('"')? + 1;
    let close = rest[open..].find('"')? + open;
    let values: Vec<f32> = rest[open..close]
        .split_whitespace()
        .filter_map(|v| v.parse().ok())
        .collect();
    match values.as_slice() {
        [_, _, w, h] => Some((*w, *h)),
        _ => None,
    }
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
    fn generated_bytes_are_valid_svg() {
        for bytes in [
            crate::bytes::HEART,
            crate::bytes::SETTINGS,
            crate::bytes::TRASH_2,
            crate::bytes::USER,
            crate::bytes::SEARCH,
            crate::bytes::STAR,
        ] {
            let text = String::from_utf8_lossy(bytes);
            assert!(text.contains("<svg"), "expected SVG markup, got: {text}");
        }
    }

    #[test]
    fn mirror_bytes_wraps_in_group() {
        let mirrored = crate::mirror_bytes(crate::bytes::HEART);
        let text = String::from_utf8_lossy(&mirrored);
        assert!(text.contains("<g transform=\"translate(24,0) scale(-1,1)\">"));
        assert!(text.contains("</g>"));
    }

    #[test]
    fn mirror_svg_constructs() {
        let _ = crate::mirror_svg(crate::bytes::HEART);
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
