# Render Pipeline Overview

## Sugarloaf Render Stages
1. **Acquire frame & clear** – `Sugarloaf::render` gets the swap‑chain frame and begins a pass that clears or loads it (`LoadOp::Clear`/`LoadOp::Load`, `StoreOp::Store`). *Texture*: surface frame (`frame.texture`, format `ctx.format`).
2. **Background image/layers** – `LayerBrush::prepare` loads any background or overlay textures and `LayerBrush::render` draws them to the frame. *Reads*: image atlas (`Rgba16Float` or `Rgba8Unorm`). *Writes*: surface frame.
3. **Rects/quads (cursor & selection)** – `QuadBrush::render` emits colored quads from `SugarState.quads`, covering cell backgrounds, selection rectangles and cursor shapes. *Reads*: uniform/vertex buffers only. *Writes*: surface frame.
4. **Glyphs** – `RichTextBrush::render` blends glyph atlas data on top of quads. *Reads*: mask atlas (`R8Unorm`) and color atlas (`Rgba8Unorm`). *Writes*: surface frame.
5. **Post‑process filters** – when configured, `FiltersBrush::render` copies the completed frame to a temporary texture and runs the RetroArch filter chain; the result overwrites the original frame. *Reads*: copy of frame texture. *Writes*: frame texture via intermediate textures (`ctx.format`).
6. **Present** – command buffer is submitted and the frame is presented to the OS compositor.

## Filter Chain Flow
- Config field `renderer.filters` is deserialized into `Vec<Filter>` in `rio-backend`.
- The frontend calls `sugarloaf.update_filters(config.renderer.filters.as_slice())` on startup and on config reloads.
- `Sugarloaf::update_filters` allocates a `FiltersBrush` and forwards the filter list.
- `FiltersBrush::update_filters` loads built‑ins or preset paths and prepares intermediate textures.
- Each frame `Sugarloaf::render` invokes `FiltersBrush::render` with the same texture as source and destination, so the chain processes the fully composed frame rather than individual layers.

## Texture/format diagram & load/store ops
```
Surface frame (frame.texture, format = ctx.format)
  LoadOp: Clear/Load → StoreOp: Store
    ↑ writes:
        LayerBrush (reads: image atlas – Rgba16Float/Rgba8Unorm)
        QuadBrush   (no texture inputs)
        RichTextBrush (reads: mask R8Unorm, color Rgba8Unorm)
  └─optional filters─┐
        copy to "Filters Source Texture" (ctx.format)
        filter chain uses intermediate textures (ctx.format)
  └──── result copied back to frame.texture ────┘
Present → OS compositor
```

- Surface format is chosen at startup; non‑mac platforms deliberately avoid sRGB formats, while macOS selects `Bgra8UnormSrgb` for an sRGB colorspace.
- The surface is configured once with that format and `PresentMode::Fifo`.
- sRGB conversion occurs only when the surface format is `*Srgb` (macOS `Colorspace::Srgb`); atlas and glyph textures remain in linear `Rgba16Float/Rgba8Unorm/R8Unorm`, so no additional sRGB conversions happen on those paths.
