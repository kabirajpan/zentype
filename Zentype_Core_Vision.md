# Zentype Core Vision

Zentype is a high-performance, modular, and **interactive** text rendering engine for Rust, built on `wgpu`, `cosmic-text`, and `swash`. 

It is designed to be the "Interactive Brain" behind the **Zenthra UI Framework**.

---

## 🎯 What is Zentype?

Zentype fills the **"Missing Middle"** of the Rust text ecosystem. 

Most text libraries are either "just renderers" (like Glyphon) that don't understand cursors, or "full universes" (like GPUI/Zed) that you can't use standalone. Zentype is a standalone crate that provides both **Elite Rendering** and **Interactive Logic**.

---

## ✨ Key Pillars

### 1. Optical Precision (The Gold Standard)
We don't just center text; we use a **1.1x Ascent / 1.4x Height** ratio. This creates a "Top-Biased" vertical balance that feels premium and spacious, exactly like high-end code editors and design tools.

### 2. Solid-Block Backgrounds
Unlike other engines that draw separate rectangles for highlights, Zentype integrates backgrounds **directly into the text shader**. This ensures:
- **Zero extra draw calls.**
- **Perfect alignment** that never "jitters" or gaps.
- **Gap-Free multi-line spanning** for professional-grade text selection.

### 3. Stateless but Capable
Zentype provides all the **capabilities** for editing (Hit-Testing, Cursor Math, Selection Mapping) without forcing a specific **state model** on you. This makes it perfect as a foundational layer for any UI framework.

---

## 🏗️ Zentype + Zenthra: How it Works

Zentype is the **Logic & Rendering Layer**, while Zenthra is the **State & Widget Layer**.

| Layer | Responsibility | Example |
|---|---|---|
| **Zenthra (UI)** | **State Ownership** | Remembers that `cursor_index = 42`. Handles the `on_click` event. |
| **Zentype (Engine)** | **Geometric Logic** | Calculates exactly which pixel `index 42` is at. Renders the blinking I-Beam. |

### The Workflow:
1. **User Clicks**: Zenthra receives a mouse event at `(100, 50)`.
2. **Hit Test**: Zenthra asks Zentype: `index_at(100, 50)`.
3. **Update State**: Zentype returns `index: 12`. Zenthra updates its widget state.
4. **Render**: Zenthra tells Zentype to `draw` with the cursor at `12`.

---

## 🚀 The Roadmap to Zenthra v0.1

To support the first Zenthra widgets (Labels, Buttons, and Inputs), Zentype is focusing on:
- [x] **Premium Backgrounds** (Current Achievement)
- [ ] **Native Hit-Testing** (Next Milestone)
- [ ] **I-Beam Cursor Rendering**
- [ ] **Selection Range Highlights**

*Zentype is the foundation. Zenthra is the masterpiece.*
