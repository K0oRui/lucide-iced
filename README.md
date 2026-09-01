# lucide-iced

Compile-time tree-shaken [Lucide](https://lucide.dev) icons for the
[Iced](https://iced.rs) GUI framework.

Only the icons you reference end up in your binary. Each icon listed in
`build.toml` generates a type-safe function that returns an
`iced::widget::Svg` widget. No runtime file reads, no icon font required.

## Features

- Tree-shaken. Only referenced SVGs are compiled in via `include_bytes!`.
- Type-safe. Generated per-icon functions like `icon::heart()`.
- Theme-aware. `themed_icon` recolors on hover and disabled states like text.
- Mirroring. `mirror_svg` flips an icon horizontally.
- Custom icons. Render your own SVGs through the same code path with
  `svg_from_bytes`.
- Optional font. Bundle the Lucide TTF for text-glyph rendering (opt-in `font`
  feature).
- Auto-updating. A GitHub Action keeps the vendored icons in sync with upstream
  Lucide.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
lucide-iced = "0.1"
```

Then use the icons in your `view()`:

```rust
use iced::widget::svg;

fn view() -> iced::Element<'static, ()> {
    lucide_iced::icon::heart().into()
}
```

### Choosing which icons to include

The icons are chosen in the crate's `build.toml`:

```toml
icons = ["heart", "settings", "trash-2", "user"]
```

Each name must have a matching file at `icons/<name>.svg`. The build script
fails with a clear message if one is missing.

`build.toml` lives in the `lucide-iced` crate itself, not in your project. For
a published dependency the icon set is fixed at publish time. To pick your own
set, fork the crate and edit `build.toml`, or vendor it.

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

Render your own SVG through the library's rendering path. `svg_from_bytes`
accepts a `&'static [u8]` or an owned `Vec<u8>`:

```rust
let svg = lucide_iced::svg_from_bytes(b"<svg .../>".as_slice());
```

## Font rendering (optional)

Enable the `font` feature to bundle the Lucide TTF:

```toml
[dependencies]
lucide-iced = { version = "0.1", features = ["font"] }
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
`fonts/lucide.ttf`, flags any icons referenced in `build.toml` that were removed
upstream, and opens a pull request for review.

## Minimum supported Rust version

`rust-version` is `1.88`.

## License

ISC. The Lucide icon set is ISC (some icons MIT-derived from Feather). See
[LICENSE](LICENSE).
