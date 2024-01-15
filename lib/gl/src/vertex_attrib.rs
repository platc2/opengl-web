use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ComponentSize(gl::GLint);

pub const SIZE_1: ComponentSize = ComponentSize(1);
pub const SIZE_2: ComponentSize = ComponentSize(2);
pub const SIZE_3: ComponentSize = ComponentSize(3);
pub const SIZE_4: ComponentSize = ComponentSize(4);

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ComponentType(gl::GLenum);

pub const FLOAT: ComponentType = ComponentType(gl::FLOAT);
pub const BYTE: ComponentType = ComponentType(gl::BYTE);
pub const UNSIGNED_BYTE: ComponentType = ComponentType(gl::UNSIGNED_BYTE);
pub const SHORT: ComponentType = ComponentType(gl::SHORT);

pub fn enable_vertex_attrib_array(index: usize) {
    unsafe { gl::EnableVertexAttribArray(index as _) };
}

pub fn disable_vertex_attrib_array(index: usize) {
    unsafe { gl::DisableVertexAttribArray(index as _) };
}

pub fn vertex_attrib_pointer(index: usize, size: ComponentSize, value_type: ComponentType, normalized: bool, stride: usize, offset: usize) {
    unsafe {
        gl::VertexAttribPointer(
            index as _,
            size.raw_handle(),
            value_type.raw_handle(),
            if normalized { gl::TRUE } else { gl::FALSE },
            stride as _,
            offset as *const _,
        );
    }
}
