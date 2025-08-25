# Background/Text Split Refactor

Goal: Allow filters to affect only the background while keeping text crisp.

## Proposed Steps
1. Allocate an off-screen **Background Render Target (BG-RT)** matching window size and format.
2. Add config knob `filters_target = "frame" | "background"`.
3. When targeting background:
   - Render background layers and quads into BG-RT.
   - Run filter chain on BG-RT, outputting to the swap-chain texture.
   - Draw glyphs/cursor/selection in a second pass with `LoadOp::Load`.
4. Resize logic re-creates BG-RT on window resizes and HiDPI changes.
5. Composite uses premultiplied alpha to preserve text pixels.
