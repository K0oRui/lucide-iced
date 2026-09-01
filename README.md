# lucide-iced

[![crates.io](https://img.shields.io/crates/v/lucide-iced)](https://crates.io/crates/lucide-iced)

Compile-time tree-shaken [Lucide](https://lucide.dev) icons for the
[Iced](https://iced.rs) GUI framework.

Every Lucide icon is available as a type-safe function that returns an
`iced::widget::Svg` widget. Only the icons you actually reference are compiled
into your binary (dead-code elimination strips the rest), so there is no runtime
file read and no icon font required.

## Features

- Tree-shaken. Only referenced SVGs are compiled in via `include_bytes!`.
- Type-safe. Generated per-icon functions like `icon::heart()`.
- Theme-aware. `themed_icon` recolors on hover and disabled states like text.
- Mirroring. `mirror_svg` flips an icon horizontally.
- Custom icons. Drop SVGs in a folder and they are auto-registered as
  `custom_icons::icon::*()` functions, or render a single SVG at runtime with
  `svg_from_bytes`.
- Optional font. Bundle the Lucide TTF for text-glyph rendering (opt-in `font`
  feature).
- Auto-updating. A GitHub Action keeps the vendored icons in sync with upstream
  Lucide.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
lucide-iced = "0.3"
```

Then use the icons in your `view()`:

```rust
use iced::widget::svg;

fn view() -> iced::Element<'static, ()> {
    lucide_iced::icon::heart().into()
}
```

### Which icons are available

Every Lucide icon is available. The full set is generated from the `icons/`
folder at build time, and only the icons you reference are compiled into your
binary. To find an icon name, browse the [Lucide icon list](https://lucide.dev/icons)
or the `icons/` folder in this crate.

### Icon names that collide with Rust keywords

A few Lucide icons are Rust keywords after conversion: `box`, `move`, and
`type`. They are generated as raw identifiers, so call them with an `r#`
prefix:

```rust
lucide_iced::icon::r#box();
lucide_iced::icon::r#move();
lucide_iced::icon::r#type();
```

The matching byte constants stay plain: `bytes::BOX`, `bytes::MOVE`,
`bytes::TYPE`.

## Raw SVG bytes

Every icon also exposes its raw SVG bytes as a constant, for rendering through
Iced's SVG mesh path inside custom widgets (checkbox checks, undo/redo buttons,
window controls, and so on):

```rust
let handle = iced::advanced::svg::Handle::from_memory(lucide_iced::bytes::CHECK);
```

The constants are named after their icon in `SCREAMING_SNAKE_CASE` (for example
`bytes::UNDO_2`, `bytes::REDO_2`, `bytes::COPY`).

## Theme-aware icons

Iced's built-in `Svg` widget ignores the surrounding `text_color`, so an icon
inside a button does not change color on hover or when disabled. `themed_icon`
draws the SVG itself and applies the incoming `text_color` as the color filter,
so it behaves like a text glyph:

```rust
let icon: iced::Element<'static, ()> =
    lucide_iced::themed_icon(lucide_iced::bytes::HEART, 16.0);
```

The second argument is the icon size in pixels. It accepts any bytes that can
become a `'static` SVG handle, so you can pass an owned `Vec<u8>` as well.

For rotation or opacity, build a `ThemedIcon` directly and chain the builder
methods. Rotation is in radians and is useful for animating an icon by advancing
the angle each frame from your `update` loop:

```rust
let handle = iced::advanced::svg::Handle::from_memory(lucide_iced::bytes::HEART);
let icon: iced::Element<'static, ()> =
    lucide_iced::ThemedIcon::new(handle, 16.0)
        .rotation(iced::Radians(0.5))
        .opacity(0.8)
        .into();
```

## Mirroring icons

Iced's `Svg` widget has no flip or scale-axis API. `mirror_svg` wraps the
SVG's inner content in a `<g transform="translate(W,0) scale(-1,1)">` group to
mirror it horizontally around the vertical center of the `viewBox`:

```rust
let heart = lucide_iced::mirror_svg(lucide_iced::bytes::HEART);
```

Use `mirror_bytes` when you need the raw mirrored bytes (for example to feed a
themed icon widget) rather than a plain `Svg` widget:

```rust
let mirrored = lucide_iced::mirror_bytes(lucide_iced::bytes::HEART);
```

## Custom icons

There are two ways to use your own SVGs.

### Auto-register a folder of icons

Enable the `build` feature and add `lucide-iced` as a build-dependency:

```toml
[dependencies]
lucide-iced = "0.3"

[build-dependencies]
lucide-iced = { version = "0.3", features = ["build"] }
```

In your `build.rs`, point at a folder of SVGs:

```rust
fn main() {
    lucide_iced::build::register_icons("icons/custom");
}
```

In your crate, include the generated module:

```rust
pub mod custom_icons {
    include!(env!("LUCIDE_ICED_CUSTOM_ICONS"));
}
```

Every `.svg` in the folder becomes a `custom_icons::icon::<name>()` function and
a `custom_icons::bytes::<NAME>` constant, generated the same way as the built-in
Lucide icons. Drop a file in, rebuild, and it is registered. The folder is
watched, so edits trigger a rebuild.

Two SVGs that map to the same identifier (for example `foo-bar.svg` and
`foo_bar.svg`) fail the build with a clear error. Icon names that collide with
Rust keywords (`box`, `move`, `type`) are generated as raw identifiers, so call
them with an `r#` prefix.

### Render a single SVG at runtime

`svg_from_bytes` builds a widget from arbitrary SVG bytes without touching the
build system. It accepts a `&'static [u8]` or an owned `Vec<u8>`:

```rust
let svg = lucide_iced::svg_from_bytes(b"<svg .../>".as_slice());
```

## Font rendering (optional)

Enable the `font` feature to bundle the Lucide TTF:

```toml
[dependencies]
lucide-iced = { version = "0.3", features = ["font"] }
```

Register the font with Iced and render icons as text glyphs:

```rust
let settings = iced::Settings {
    fonts: vec![lucide_iced::LUCIDE_FONT_BYTES.into()],
    ..Default::default()
};
```

The font is a single binary file and cannot be tree-shaken. It is only embedded
when the `font` feature is enabled, and it adds about 850 KB to your binary.

## Auto-updating icons

The included GitHub Action (`.github/workflows/update-icons.yml`) runs daily and
on manual dispatch. It fetches the latest Lucide release, syncs `icons/` and
`fonts/lucide.ttf`, and opens a pull request for review.

## Minimum supported Rust version

`rust-version` is `1.88`.

## License

ISC. The Lucide icon set is ISC (some icons MIT-derived from Feather). See
[LICENSE](LICENSE).
