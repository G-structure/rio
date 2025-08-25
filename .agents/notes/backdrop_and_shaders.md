# Backdrop Capture & Shader Considerations

## OS Backdrop Sampling
- **macOS:** Use ScreenCaptureKit or `CGWindowListCreateImage` to grab the region behind the window. Frames can be shared via `IOSurface` and uploaded to a wgpu texture. Requires Screen Recording permission.
- **Windows:** Leverage DXGI Desktop Duplication to obtain monitor images and share textures with the D3D12 backend. Capture at ≤60 Hz to minimize latency.
- **Linux/Wayland:** Direct backdrop access is restricted. PipeWire portal can capture the screen with user consent but cannot isolate the exact window area. Fallback to compositor blur or static wallpaper.

## Shader Capabilities
- RetroArch filters run as a full-frame post-process; they cannot currently exclude text or sample the OS backdrop without pipeline changes.
- To refract the backdrop behind text, introduce a pre-pass that combines BG-RT with an OS/video texture before the filter chain.
- 3D or video backgrounds can render into BG-RT in Pass 1, keeping glyphs unfiltered in Pass 2.
- Filters can change pixel colors (including alpha) of Rio's own frame buffer but have no access to the desktop behind a transparent window, so true glass-like refraction requires an explicit OS backdrop provider.
- Background-only effects need a rendering split; today the chain operates on the fully composed frame.
- RetroArch filters are 2‑D post-processing shaders; they cannot render standalone 3‑D scenes without feeding pre-rendered textures.

## Rio Shader Support
- `renderer.filters` lists shader presets. Built‑ins (`newpixiecrt`, `fubax_vr`) are embedded and unpacked at runtime; other entries are loaded from user-specified `.slangp` files.
- Filters are applied via `FiltersBrush::render` and currently require non‑OpenGL backends (wgpu Metal/Vulkan/DX12).
