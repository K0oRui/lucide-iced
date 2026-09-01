# lucide-iced

Compile-time tree-shaken [Lucide](https://lucide.dev) icons for the
[Iced](https://iced.rs) GUI framework.

Only the icons you actually use are embedded into your binary. Each icon listed in
`build.toml` generates a type-safe function that returns an `iced::widget::Svg`
widget. No runtime file reads, no icon font required.

## Features

- Tree-shaken. Only referenced SVGs are compiled in via `include_bytes!`.
- Type-safe. Generated per-icon functions like `icon::heart()`.
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

Edit `build.toml` in your crate root to choose which icons to include:

```toml
icons = ["heart", "settings", "trash-2", "user"]
```

Then use them in your `view()`:

```rust
use iced::widget::svg;

fn view() -> iced::Element<'static, ()> {
    lucide_iced::icon::heart().into()
}
```

Each icon name in `build.toml` must have a matching file at `icons/<name>.svg`. The
build script fails with a clear message if one is missing.

## Raw SVG bytes

Every icon also exposes its raw SVG bytes as a constant, for rendering through
Iced's SVG mesh path inside custom widgets (checkbox checks, undo/redo buttons,
window controls, and so on):

```rust
let handle = iced::advanced::svg::Handle::from_memory(lucide_iced::bytes::CHECK);
```

The constants are named after their icon in `SCREAMING_SNAKE_CASE` (for example
`bytes::UNDO_2`, `bytes::REDO_2`, `bytes::COPY`).

## Custom icons

Render your own SVG through the library's rendering path:

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

The font is a single binary file and cannot be tree-shaken. It is only embedded when
the `font` feature is enabled.

## Auto-updating icons

The included GitHub Action (`.github/workflows/update-icons.yml`) runs daily and on
manual dispatch. It fetches the latest Lucide release, syncs `icons/` and
`fonts/lucide.ttf`, flags any icons referenced in `build.toml` that were removed
upstream, and opens a pull request for review.

## License

ISC. The Lucide icon set is ISC (some icons MIT-derived from Feather). See
[LICENSE](LICENSE).
