//! A theme-aware SVG icon widget.
//!
//! Iced's built-in [`Svg`] widget ignores the surrounding
//! [`renderer::Style`] (specifically `text_color`), so icons rendered with it
//! do not change color when the surrounding widget does (e.g. a button's
//! hover/disabled state). This module provides [`ThemedIcon`], a small widget
//! that draws an SVG itself and applies the incoming `style.text_color` as the
//! color filter, making icons behave like text glyphs.

use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::svg;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget};
use iced::mouse;
use iced::{Element, Length, Rectangle, Size};

/// An icon rendered as an SVG that inherits the surrounding text color.
///
/// Unlike [`iced::widget::Svg`], this widget reads the [`renderer::Style`]
/// passed down by the parent (e.g. a button) and uses its `text_color` as the
/// SVG color filter, so the icon recolors on hover/disabled just like text.
pub struct ThemedIcon<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    handle: svg::Handle,
    size: f32,
    _phantom: std::marker::PhantomData<&'a (Message, Theme, Renderer)>,
}

impl<'a, Message, Theme, Renderer> ThemedIcon<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + svg::Renderer + 'a,
    Theme: 'a,
    Message: 'a,
{
    /// Creates a new [`ThemedIcon`] from an SVG handle at the given size.
    pub fn new(handle: svg::Handle, size: f32) -> Self {
        Self {
            handle,
            size,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ThemedIcon<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + svg::Renderer + 'a,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        renderer.draw_svg(
            svg::Svg {
                handle: self.handle.clone(),
                color: Some(style.text_color),
                rotation: iced::Radians(0.0),
                opacity: 1.0,
            },
            bounds,
            *viewport,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<ThemedIcon<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + svg::Renderer + 'a,
{
    fn from(icon: ThemedIcon<'a, Message, Theme, Renderer>) -> Self {
        Element::new(icon)
    }
}

/// Builds a theme-aware icon element from raw SVG bytes at the given size.
///
/// The icon inherits the surrounding `text_color`, so it recolors on hover and
/// disabled states like text glyphs. Accepts any bytes that can be turned into
/// a `'static` SVG handle (a `&'static [u8]`, an owned `Vec<u8>`, etc.).
///
/// The returned element is `'static` (the handle owns its bytes), so it can be
/// used anywhere an `Element<'_, Message>` is expected without forcing
/// `Message: 'static`.
///
/// # Example
///
/// ```no_run
/// let icon: iced::Element<'static, ()> =
///     lucide_iced::themed_icon(lucide_iced::bytes::HEART, 16.0);
/// ```
pub fn themed_icon<'a, Message: 'a>(
    bytes: impl Into<std::borrow::Cow<'static, [u8]>>,
    size: f32,
) -> Element<'a, Message> {
    let handle = svg::Handle::from_memory(bytes);
    ThemedIcon::new(handle, size).into()
}
