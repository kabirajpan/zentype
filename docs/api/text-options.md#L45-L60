# TextWrap API

`TextWrap` determines how the engine handles text that exceeds the container width.

## Enum Variants

| Variant | Description |
| :--- | :--- |
| `None` | No wrapping. Text will continue on a single line and may be clipped. |
| `Word` | (Default) Wraps at whole word boundaries. |
| `Character` | Wraps at the nearest character (useful for CJK languages). |

## Usage

```rust
let options = TextOptions::new()
    .wrap(TextWrap::None); // Force single-line headers
```

## Internal Mapping

`TextWrap` maps directly to `cosmic_text::Wrap`.
