# Zenthra — What It Needs From Zentype

Every feature Zenthra UI Framework requires from Zentype, mapped to the exact component that needs it.

---

## 🔴 Must Have Before Zenthra Can Start

These are blocking. Zenthra cannot render a single widget without these.

- [x] Basic text rendering on GPU
- [x] Instanced draw calls
- [x] Per-glyph color
- [ ] Font loading from file and bytes
- [x] Font weight and style
- [x] Font size control
- [ ] Font fallback chains
- [x] Text wrapping (word, character, none)
- [x] Text alignment (left, center, right)
- [x] Max width constraint
- [ ] Multi-line layout
- [x] Line height control
- [x] Per-text background color
- [x] Background padding
- [x] Full-width line highlight
- [x] Glyph atlas with LRU eviction
- [x] GPU buffer reuse across frames
- [ ] Level 1 managed API (`Zentype::new`)
- [ ] Level 2 semi managed (`TextRenderer`)
- [x] Standalone wgpu integration
- [x] Winit integration
- [x] `use zentype::prelude::*`
- [ ] Panic-free API (`Result` everywhere)

---

## 🟡 Must Have Before Zenthra Ships Input Components

These are needed for `TextInput`, `TextArea`, and `CodeEditor`.

- [ ] Cursor position tracking
- [ ] I-beam cursor rendering
- [ ] Blinking cursor animation
- [ ] Cursor color customization
- [ ] Selection start and end positions
- [ ] Selection highlight rendering
- [ ] Multi-line selection
- [ ] Click position → character index
- [ ] Character index → pixel position
- [ ] Line number from click
- [ ] Insert character at cursor
- [ ] Delete character (backspace, delete)
- [ ] Insert newline
- [ ] Word boundaries detection
- [ ] Word selection (double click)
- [ ] Line selection (triple click)
- [ ] Select all
- [ ] Copy / paste support

---

## 🟡 Must Have Before Zenthra Ships CodeEditor

These are needed specifically for the `CodeEditor` component.

- [ ] Token-based syntax coloring
- [ ] Custom theme support
- [ ] Highlight current line
- [ ] Bracket matching highlight
- [ ] Undo / redo stack
- [ ] Highlight ranges with custom colors
- [ ] Line number from click
- [ ] Error / warning underlines

---

## 🟡 Must Have Before Zenthra Ships RichText

These are needed for the `RichText` and `Label` components.

- [ ] Underline
- [ ] Strikethrough
- [ ] Mixed font sizes on one line
- [ ] Mixed font weights on one line
- [ ] Mixed colors on one line
- [ ] Code spans (monospace inline)
- [ ] Hyperlinks (clickable text regions)

---

## 🟢 Nice to Have for Zenthra v1.0

Not blocking but make Zenthra production-ready.

- [ ] IME support (Chinese, Japanese, Korean, Arabic)
- [ ] RTL text support
- [ ] Bidirectional text (BiDi)
- [ ] Emoji support (color glyphs)
- [ ] Subpixel antialiasing
- [ ] Debug overlay (atlas, glyph bounds)
- [ ] Tracing / logging support

---

## How Features Map to Zenthra Components

| Zenthra Component | Zentype Features Required |
|---|---|
| `Label` | rendering, font system, alignment, wrapping |
| `Button` | rendering, font system, alignment |
| `TextInput` | rendering, cursor, selection, hit-test, editing |
| `TextArea` | rendering, cursor, selection, hit-test, editing, multi-line |
| `CodeEditor` | all of TextArea + syntax highlighting + line highlight |
| `RichText` | rendering + rich text (underline, mixed styles, links) |
| `Tooltip` | rendering, wrapping, background |
| `Dropdown` | rendering, alignment, font system |
| `Menu` | rendering, alignment, highlight |
| `Badge` | rendering, background, font weight |

---

## Build Order for Zenthra

```
Step 1 — Label, Button, Tooltip, Badge
         needs: rendering, fonts, layout, backgrounds

Step 2 — TextInput, Dropdown, Menu
         needs: cursor, selection, hit-test, editing

Step 3 — TextArea
         needs: multi-line editing, undo/redo

Step 4 — CodeEditor
         needs: syntax highlighting, bracket matching

Step 5 — RichText
         needs: mixed styles, links, inline images

Step 6 — Internationalization
         needs: RTL, BiDi, IME, complex scripts
```

---

## Cargo Features Zenthra Will Use

```toml
[dependencies]
zentype = { version = "0.1", features = [
    "managed",      # Level 1 API
    "renderer",     # Level 2 API
    "selection",    # text selection
    "hit-test",     # click handling
    "editing",      # insert, delete, undo
    "syntax",       # code editor
    "rich-text",    # rich text component
] }
```

---

*Zenthra is built on top of Zentype. Every UI component traces back to one or more features in this list.*