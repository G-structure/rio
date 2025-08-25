#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BackdropSource {
    #[default]
    None,
    Os,
    Video,
    Scene3D,
}

pub trait BackdropProvider {
    fn begin_frame(&mut self, rect: PhysicalRect) -> Option<wgpu::TextureView>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PhysicalRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl PhysicalRect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "android")))]
pub use linux::OsBackdropProvider;
#[cfg(target_os = "macos")]
pub use macos::OsBackdropProvider;
#[cfg(target_os = "windows")]
pub use windows::OsBackdropProvider;
