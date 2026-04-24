
# Zentype — Code Editor Feature Checklist

Everything Zentype must handle to power a professional code editor.

> ✅ Belongs in Zentype · ❌ Does not belong in Zentype

---

## ✅ Typography

- [x] Font size (affects rasterization)
- [x] Font weight (bold keywords need different glyph)
- [x] Font style (italic comments need different glyph)
- [x] Font family (monospace font for code)
- [ ] Font fallback chains (missing glyph → try next font)
- [ ] Ligatures (`->` `!=` `=>` as single glyphs)
- [ ] Letter spacing
- [x] Line height
- [x] Padding support
- [x] Horizontal & Vertical alignment
- [ ] Tab width (affects glyph positioning)


---

## ✅ Colors

- [x] Text color (per-glyph GPU data)
- [ ] Syntax colors (per-token color, rendered per-glyph)
- [ ] Cursor color
- [ ] Selection color
- [x] Highlight color (current line)
- [ ] Bracket match color
- [ ] Error underline color
- [ ] Warning underline color


---

## ✅ Highlights & Backgrounds

- [x] Current line highlight (full-width bar in shader)
- [ ] Selection highlight (rendered in shader)
- [ ] Bracket match highlight (highlight range with custom color)
- [ ] Multi-range highlight (multiple selections)
- [x] Highlight opacity control
- [x] Full-width background spanning entire viewport


---

## ✅ Underlines & Decorations

- [ ] Error underline (red squiggle under glyph)
- [ ] Warning underline (yellow squiggle under glyph)
- [ ] Info underline (blue underline)
- [ ] Strikethrough
- [ ] Regular underline
- [ ] Underline color customization

---

## ✅ Cursor

- [ ] Cursor position tracking
- [ ] I-beam cursor rendering
- [ ] Block cursor rendering
- [ ] Underscore cursor rendering
- [ ] Blinking cursor animation
- [ ] Cursor color customization
- [ ] Cursor width customization
- [ ] Cursor between glyphs (not just characters)
- [ ] Multiple cursors (multi-cursor editing)

---

## ✅ Selection

- [ ] Selection start and end positions
- [ ] Selection highlight rendering
- [ ] Multi-line selection
- [ ] Multiple selection ranges
- [ ] Selection color customization
- [ ] Word selection (double click)
- [ ] Line selection (triple click)
- [ ] Select all
- [ ] Column / block selection

---

## ✅ Hit Testing

- [x] Click position → character index
- [x] Character index → pixel position
- [ ] Line number from click
- [ ] Word boundaries detection
- [ ] Glyph bounding boxes
- [ ] Line bounding boxes

---

## ✅ Layout

- [x] Multi-line layout
- [ ] Visible line culling (only render what's on screen)
- [x] Max width constraint
- [x] Text wrapping (word, character, none)
- [ ] Soft wrap indicators
- [ ] Tab rendering (expand tabs to spaces visually)
- [ ] Whitespace rendering (dots for spaces, arrows for tabs)


---

## ✅ Special Rendering

- [ ] Line numbers (rendered as text on GPU)
- [ ] Emoji support (color glyphs in comments/strings)
- [ ] Whitespace characters (visible dots and arrows)
- [ ] Indent guides (vertical lines showing indentation)
- [x] Glyph atlas management (GPU texture cache)
- [x] Instanced draw calls (all glyphs in one draw call)
- [x] GPU buffer reuse across frames


---

## ❌ Does NOT Belong in Zentype

These belong in the editor logic layer or the app above.

- [ ] ❌ Scroll position (editor/app decides this)
- [ ] ❌ Keyboard input handling
- [ ] ❌ File loading and saving
- [ ] ❌ Undo / redo stack
- [ ] ❌ Find & replace logic
- [ ] ❌ Syntax parsing (parser gives tokens, Zentype colors them)
- [ ] ❌ Window management
- [ ] ❌ Tab management
- [ ] ❌ Buffer management
- [ ] ❌ Language server protocol (LSP)
- [ ] ❌ Git diff logic (Zentype can render the diff colors, not compute them)

---

## Priority Order for Code Editor

```
Phase 1 → typography, colors, line highlight, glyph atlas
Phase 2 → cursor, selection, hit testing
Phase 3 → underlines, decorations, whitespace rendering
Phase 4 → multiple cursors, column selection, indent guides
Phase 5 → ligatures, emoji, soft wrap indicators
```

---

## Summary

| Category | Total Items | Belongs in Zentype |
|---|---|---|
| Typography | 9 | ✅ All |
| Colors | 8 | ✅ All |
| Highlights | 6 | ✅ All |
| Underlines | 6 | ✅ All |
| Cursor | 9 | ✅ All |
| Selection | 9 | ✅ All |
| Hit Testing | 6 | ✅ All |
| Layout | 7 | ✅ All |
| Special Rendering | 8 | ✅ All |
| Editor Logic | 11 | ❌ None |

---

*Part of the Zentype — Code Editor integration guide.*