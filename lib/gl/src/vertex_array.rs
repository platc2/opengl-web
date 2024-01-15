use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct VertexArrayId(gl::GLuint);

pub const NO_VERTEX_ARRAY: VertexArrayId = VertexArrayId(0);

#[must_use]
pub fn gen_vertex_arrays(count: usize) -> Vec<VertexArrayId> {
    let mut raw_vertex_array_ids: Vec<gl::GLuint> = Vec::with_capacity(count);

    unsafe {
        let raw_vertex_array_ids_ptr = raw_vertex_array_ids.spare_capacity_mut().as_mut_ptr().cast();
        gl::GenVertexArrays(count as gl::GLsizei, raw_vertex_array_ids_ptr);
        raw_vertex_array_ids.set_len(count);
    }

    raw_vertex_array_ids.into_iter()
        .map(VertexArrayId)
        .collect()
}

#[must_use]
pub fn gen_vertex_array() -> VertexArrayId {
    let mut vertex_array_id: gl::GLuint = 0;
    unsafe { gl::GenVertexArrays(1, &mut vertex_array_id); };
    VertexArrayId(vertex_array_id)
}

pub fn bind_vertex_array(vertex_array_id: VertexArrayId) {
    unsafe { gl::BindVertexArray(vertex_array_id.raw_handle()) };
}
