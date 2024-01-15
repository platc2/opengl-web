use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct BufferTarget(gl::GLenum);

pub const ARRAY_BUFFER: BufferTarget = BufferTarget(gl::ARRAY_BUFFER);
pub const ELEMENT_ARRAY_BUFFER: BufferTarget = BufferTarget(gl::ELEMENT_ARRAY_BUFFER);

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct BufferUsage(gl::GLenum);

pub const STREAM_DRAW: BufferUsage = BufferUsage(gl::STREAM_DRAW);
pub const STATIC_DRAW: BufferUsage = BufferUsage(gl::STATIC_DRAW);
pub const STREAM_READ: BufferUsage = BufferUsage(gl::STREAM_READ);
pub const STATIC_READ: BufferUsage = BufferUsage(gl::STATIC_READ);
pub const STREAM_COPY: BufferUsage = BufferUsage(gl::STREAM_COPY);
pub const STATIC_COPY: BufferUsage = BufferUsage(gl::STATIC_COPY);

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct BufferId(gl::GLuint);

pub const NO_BUFFER: BufferId = BufferId(0);

#[must_use]
pub fn gen_buffers(count: usize) -> Vec<BufferId> {
    let mut raw_buffer_ids: Vec<gl::GLuint> = Vec::with_capacity(count);

    unsafe {
        let raw_buffer_ids_ptr = raw_buffer_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::GenBuffers(count as gl::GLsizei, raw_buffer_ids_ptr);
        raw_buffer_ids.set_len(count);
    }

    raw_buffer_ids.into_iter()
        .map(BufferId)
        .collect()
}

#[must_use]
pub fn gen_buffer() -> BufferId {
    let mut buffer_id: gl::GLuint = 0;
    unsafe { gl::GenBuffers(1, &mut buffer_id) };
    BufferId(buffer_id)
}

pub fn bind_buffer(target: BufferTarget, buffer_id: BufferId) {
    unsafe { gl::BindBuffer(target.raw_handle(), buffer_id.raw_handle()) };
}

pub fn buffer_data<T>(target: BufferTarget, data: &[T], usage: BufferUsage) {
    let size: gl::GLsizeiptr = (data.len() * std::mem::size_of::<T>()) as isize;
    unsafe {
        gl::BufferData(target.raw_handle(), size, data.as_ptr().cast(), usage.raw_handle());
    }
}

pub fn delete_buffers(buffer_ids: &mut [BufferId]) {
    let raw_buffer_ids = buffer_ids.into_iter()
        .map(|buffer_id| unsafe { buffer_id.raw_handle() })
        .collect::<Vec<_>>();
    unsafe {
        gl::DeleteBuffers(buffer_ids.len() as gl::GLsizei, raw_buffer_ids.as_ptr());
    }
    buffer_ids.into_iter()
        .for_each(|buffer_id| buffer_id.0 = 0);
}

pub fn delete_buffer(buffer_id: &mut BufferId) {
    unsafe { gl::DeleteBuffers(1, &buffer_id.raw_handle()) };
    buffer_id.0 = 0;
}
