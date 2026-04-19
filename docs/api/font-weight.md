# FontWeight API

`FontWeight` is an enum that defines the thickness of the glyphs. It is compatible with standard OpenType weight values.

## Enum Variants

| Variant | Value |
| :--- | :--- |
| `Thin` | 100 |
| `Light` | 300 |
| `Normal` | 400 |
| `Medium` | 500 |
| `SemiBold` | 600 |
| `Bold` | 700 |
| `ExtraBold` | 800 |
| `Black` | 900 |

## Usage

```rust
let options = TextOptions::new()
    .font_weight(FontWeight::Bold);
```

## Conversions

`FontWeight` can be converted directly into `cosmic_text::Weight`.

```rust
let cosmic_weight: cosmic_text::Weight = FontWeight::Bold.into();
```
