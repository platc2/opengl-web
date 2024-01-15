use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct Capability(gl::GLenum);

pub const BLEND: Capability = Capability(gl::BLEND);
pub const CULL_FACE: Capability = Capability(gl::CULL_FACE);
pub const DEPTH_TEST: Capability = Capability(gl::DEPTH_TEST);
pub const STENCIL_TEST: Capability = Capability(gl::STENCIL_TEST);
pub const SCISSOR_TEST: Capability = Capability(gl::SCISSOR_TEST);

pub fn enable(capability: Capability) {
    unsafe { gl::Enable(capability.raw_handle()) };
}

pub fn disable(capability: Capability) {
    unsafe { gl::Disable(capability.raw_handle()) };
}
