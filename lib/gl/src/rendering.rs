use std::ops::BitOr;

use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ClearMask(gl::GLenum);

pub const COLOR: ClearMask = ClearMask(gl::COLOR_BUFFER_BIT);
pub const DEPTH: ClearMask = ClearMask(gl::DEPTH_BUFFER_BIT);
pub const STENCIL: ClearMask = ClearMask(gl::STENCIL_BUFFER_BIT);

impl BitOr for ClearMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output { Self(unsafe { self.raw_handle() | rhs.raw_handle() }) }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct DrawMode(gl::GLenum);

pub const POINTS: DrawMode = DrawMode(gl::POINTS);
pub const LINE_STRIP: DrawMode = DrawMode(gl::LINE_STRIP);
pub const LINE_LOOP: DrawMode = DrawMode(gl::LINE_LOOP);
pub const LINES: DrawMode = DrawMode(gl::LINES);
pub const TRIANGLE_STRIP: DrawMode = DrawMode(gl::TRIANGLE_STRIP);
pub const TRIANGLE_FAN: DrawMode = DrawMode(gl::TRIANGLE_FAN);
pub const TRIANGLES: DrawMode = DrawMode(gl::TRIANGLES);

pub fn clear(clear_mask: ClearMask) {
    unsafe { gl::Clear(clear_mask.raw_handle()) };
}

pub fn clear_color(color_argb: u32) {
    let a = ((color_argb >> 24) & 0xFF) as f32 / 255f32;
    let r = ((color_argb >> 16) & 0xFF) as f32 / 255f32;
    let g = ((color_argb >> 8) & 0xFF) as f32 / 255f32;
    let b = ((color_argb >> 0) & 0xFF) as f32 / 255f32;
    unsafe { gl::ClearColor(r, g, b, a) };
}

pub fn draw_arrays(draw_mode: DrawMode, start_index: usize, count: usize) {
    unsafe { gl::DrawArrays(draw_mode.raw_handle(), start_index as _, count as _) };
}
