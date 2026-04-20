
# Zentype — Full Feature Checklist

Complete list of every feature Zentype can have, organized by category with priority levels.

> 🔴 Required for Zenthra · 🟡 Important · 🟢 Nice to have · ⚪ Future

---

## 1. 🔤 Text Rendering

- [x] 🔴 Basic text rendering on GPU
- [x] 🔴 Instanced draw calls (thousands of glyphs, single draw call)
- [x] 🔴 Alpha-masked glyph rendering
- [x] 🔴 Per-glyph color
- [ ] 🔴 Per-glyph opacity
- [ ] 🟡 Color glyph rendering (emoji)
- [ ] 🟡 Gamma-correct blending
- [ ] 🟡 Subpixel antialiasing
- [ ] 🟢 SDF / MSDF rendering (crisp at any scale)

---

## 2. 🔡 Font System

- [ ] 🔴 Load font from file path
- [ ] 🔴 Load font from bytes
- [ ] 🔴 Multiple fonts in one render pass
- [x] 🔴 Font weight (thin, light, regular, medium, bold, black)
- [ ] 🔴 Font style (normal, italic, oblique)
- [x] 🔴 Font size (any float value)
- [ ] 🔴 Font fallback chains
- [ ] 🟡 Load system fonts
- [ ] 🟡 Font metrics (ascent, descent, line gap)
- [ ] 🟢 Variable fonts support

---

## 3. 📐 Layout & Shaping

- [ ] 🔴 Left to right text (LTR)
- [x] 🔴 Text wrapping (word, character, none)
- [x] 🔴 Text alignment (left, center, right, justified)
- [x] 🔴 Max width constraint
- [ ] 🔴 Multi-line layout
- [x] 🔴 Line height control
- [ ] 🔴 Letter spacing
- [ ] 🔴 Word spacing
- [ ] 🔴 Kerning
- [ ] 🔴 Ligatures
- [ ] 🟡 Inline layout (mix fonts/sizes on one line)
- [ ] 🟡 Right to left text (RTL)
- [ ] 🟡 Bidirectional text (BiDi)
- [ ] 🟢 Complex scripts (Arabic, Devanagari, Thai etc)

---

## 4. 🎨 Backgrounds & Highlights

- [x] 🔴 Per-text background color
- [x] 🔴 Full-width line highlight (editor style)
- [x] 🔴 Background padding control
- [x] 🔴 Multi-line background spanning (Gap-Free)
- [x] 🔴 Highlight ranges (Optical Gold Standard 1.1/1.4)
- [ ] 🔴 Custom highlight colors per range
- [ ] 🟡 Rounded background corners
- [ ] 🟡 Background opacity

### 2026-04-20

- ✅ Background Highlights — finalized 1.1/1.4 optical standard with gap-free multi-line support.
- ✅ Padding Control — added support for symmetrical padding in background highlights.
- ✅ Smart Metrics — integrated line-info in shaper for professional alignment.
- ✅ Multi-Line Spanning — implemented seamless, gap-free background blocks across text wrap points.

---

## 5. ✏️ Text Selection

- [ ] 🔴 Selection start and end positions
- [ ] 🔴 Selection highlight rendering
- [ ] 🔴 Multi-line selection
- [ ] 🔴 Selection color customization
- [ ] 🟡 Word selection (double click)
- [ ] 🟡 Line selection (triple click)
- [ ] 🟡 Select all
- [ ] 🟡 Selection with keyboard (shift + arrows)

---

## 6. 🖱️ Cursor

- [ ] 🔴 Cursor position tracking
- [ ] 🔴 I-beam cursor rendering
- [ ] 🔴 Blinking cursor animation
- [ ] 🔴 Cursor color customization
- [ ] 🟡 Block cursor rendering
- [ ] 🟡 Underscore cursor rendering
- [ ] 🟡 Cursor width customization
- [ ] 🟡 Cursor between glyphs (not just characters)

---

## 7. 🎯 Hit Testing

- [ ] 🔴 Click position → character index
- [ ] 🔴 Character index → pixel position
- [ ] 🔴 Line number from click
- [ ] 🟡 Word boundaries detection
- [ ] 🟡 Glyph bounding boxes
- [ ] 🟡 Line bounding boxes
- [ ] 🟢 Paragraph bounding boxes

---

## 8. ⌨️ Input & Editing

- [ ] 🔴 Insert character at cursor
- [ ] 🔴 Delete character (backspace, delete)
- [ ] 🔴 Insert newline
- [ ] 🟡 Undo / redo stack
- [ ] 🟡 Copy / paste support
- [ ] 🟡 IME support (Chinese, Japanese, Korean, Arabic)
- [ ] 🟢 Composition string preview (IME in-progress text)
- [ ] 🟢 Dead key support

---

## 9. 🌈 Syntax Highlighting

- [ ] 🟡 Token-based coloring
- [ ] 🟡 Custom theme support
- [ ] 🟡 Highlight current line
- [ ] 🟡 Bracket matching highlight
- [ ] 🟢 Language definitions (Rust, Python, JS etc)
- [ ] 🟢 Error / warning underlines
- [ ] 🟢 Inline diagnostics
- [ ] 🟢 Highlight matching words

---

## 10. 🖼️ Rich Text

- [ ] 🟡 Underline
- [ ] 🟡 Strikethrough
- [ ] 🟡 Mixed font sizes on one line
- [ ] 🟡 Mixed font weights on one line
- [ ] 🟡 Mixed colors on one line
- [ ] 🟡 Code spans (monospace inline)
- [ ] 🟢 Hyperlinks (clickable text regions)
- [ ] 🟢 Superscript / subscript
- [ ] 🟢 Inline images mixed with text
- [ ] 🟢 Blockquotes

---

## 11. ⚡ Performance

- [x] 🔴 Glyph atlas with LRU eviction
- [ ] 🔴 Auto-growing atlas texture
- [ ] 🔴 Bucketed atlas allocation by glyph size
- [x] 🔴 GPU buffer reuse across frames
- [ ] 🔴 Batch multiple text draws into one GPU call
- [ ] 🟡 Dirty region tracking (partial GPU uploads)
- [ ] 🟡 Zero allocation render path (hot path)
- [ ] 🟢 Frame budget tracking

---

## 12. 🔧 API Levels

- [ ] 🔴 Level 1 — full managed (`Zentype::new`)
- [ ] 🔴 Level 2 — semi managed (`TextRenderer`)
- [x] 🔴 Level 3 — raw primitives (`GlyphAtlas`, `ZentypePipeline`)
- [x] 🔴 Builder pattern (`TextRenderer::builder()`)
- [ ] 🔴 Escape hatches (drop down at runtime)
- [ ] 🔴 Bring your own font provider
- [x] `#RGB` (3 digits)
- [x] `#RGBA` (4 digits)
- [x] `#RRGGBB` (6 digits)
- [x] `#RRGGBBAA` (8 digits)
- [x] Standard Web Colors (Red, Blue, etc.)
- [x] Premium Palette (Teal, Indigo, etc.)
- [x] `with_alpha` utility
- [x] `to_f32_array` for GPU
- [x] `to_u32` for binary export
- [x] `as_attrs` for cosmic-text integration

---

## 13. 🧩 Integrations

- [x] 🔴 Standalone wgpu integration
- [ ] 🔴 Existing wgpu render graph integration
- [x] 🔴 Winit window integration
- [ ] 🔴 Zenthra UI Framework integration
- [ ] 🟡 Custom windowing system support
- [ ] 🟢 WebGPU / WASM target

---

## 14. 🛠️ Developer Experience

- [x] 🔴 `use zentype::prelude::*` for beginners
- [ ] 🔴 Panic-free API (return `Result` everywhere)
- [ ] 🔴 Descriptive error messages
- [ ] 🟡 Tracing / logging support
- [ ] 🟡 `cargo doc` with full examples
- [ ] 🟢 Debug overlay (atlas visualization, glyph bounds)
- [ ] 🟢 Hot reload fonts in debug mode

---

## 15. 🚩 Cargo Feature Flags

- [ ] 🔴 `default = ["managed", "cosmic"]`
- [ ] 🔴 `managed` — Level 1 Zentype struct
- [ ] 🔴 `renderer` — Level 2 TextRenderer
- [ ] 🔴 `cosmic` — cosmic-text font backend
- [ ] 🟡 `selection` — text selection + cursor
- [ ] 🟡 `hit-test` — click → character position
- [ ] 🟡 `editing` — insert, delete, undo, redo
- [ ] 🟡 `syntax` — syntax highlighting
- [ ] 🟡 `rich-text` — mixed styles, links
- [ ] 🟢 `ime` — input method engine
- [ ] 🟢 `emoji` — color emoji support
- [ ] 🟢 `sdf` — SDF glyph rendering
- [ ] 🟢 `wasm` — WebGPU / WASM target
- [ ] 🟢 `debug` — atlas overlay, glyph bounds
- [ ] 🟢 `full` — everything enabled
- [ ] 🟢 `harfbuzz` — alternative font backend

---

## Priority Summary

| Priority | Count | When |
|---|---|---|
| 🔴 Required for Zenthra | 47 items | Phase 1 & 2 |
| 🟡 Important | 28 items | Phase 3 & 4 |
| 🟢 Nice to have | 21 items | Phase 5 & 6 |

---

*Part of the Zentype UI Framework suite.*