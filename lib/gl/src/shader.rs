use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ShaderKind(gl::GLenum);

pub const VERTEX_SHADER: ShaderKind = ShaderKind(gl::VERTEX_SHADER);
pub const FRAGMENT_SHADER: ShaderKind = ShaderKind(gl::FRAGMENT_SHADER);
#[cfg(not(target_os = "emscripten"))]
pub const GEOMETRY_SHADER: ShaderKind = ShaderKind(gl::GEOMETRY_SHADER);
#[cfg(not(target_os = "emscripten"))]
pub const TESSELLATION_CONTROL_SHADER: ShaderKind = ShaderKind(gl::TESS_CONTROL_SHADER);
#[cfg(not(target_os = "emscripten"))]
pub const TESSELLATION_EVALUATION_SHADER: ShaderKind = ShaderKind(gl::TESS_EVALUATION_SHADER);
#[cfg(not(target_os = "emscripten"))]
pub const COMPUTE_SHADER: ShaderKind = ShaderKind(gl::COMPUTE_SHADER);

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ShaderId(gl::GLuint);

#[must_use]
pub fn create_shader(shader_kind: ShaderKind) -> ShaderId {
    let id = unsafe { gl::CreateShader(shader_kind.raw_handle()) };
    ShaderId(id)
}

pub fn delete_shader(shader_id: &mut ShaderId) {
    unsafe { gl::DeleteShader(shader_id.raw_handle()) };
    shader_id.0 = 0;
}

pub fn shader_source(shader_id: ShaderId, source: &str) {
    unsafe {
        gl::ShaderSource(
            shader_id.raw_handle(),
            1,
            &(source.as_ptr().cast()),
            &(source.len() as gl::GLint))
    };
}

pub fn compile_shader(shader_id: ShaderId) {
    unsafe { gl::CompileShader(shader_id.raw_handle()) };
}

#[must_use]
pub fn shader_compile_status(shader_id: ShaderId) -> bool {
    let mut success: gl::GLint = 0;
    unsafe { gl::GetShaderiv(shader_id.raw_handle(), gl::COMPILE_STATUS, &mut success) };
    return success != 0;
}

#[must_use]
pub fn shader_info_log(shader_id: ShaderId) -> Option<String> {
    let mut log_len: gl::GLint = 0;

    unsafe { gl::GetShaderiv(shader_id.raw_handle(), gl::INFO_LOG_LENGTH, &mut log_len) };

    if log_len == 0 {
        None
    } else {
        // log_len includes null termination character, which we do not require
        let mut info_log_buffer = Vec::with_capacity(log_len as usize);
        let mut written_length: gl::GLsizei = 0;
        let info_log_buffer_ptr = info_log_buffer.spare_capacity_mut().as_ptr() as *mut gl::GLchar;
        let info_log = unsafe {
            gl::GetShaderInfoLog(shader_id.raw_handle(), log_len, &mut written_length, info_log_buffer_ptr);
            info_log_buffer.set_len(std::cmp::max(written_length, log_len) as usize);
            let written_length = written_length as usize;
            String::from_raw_parts(info_log_buffer.as_mut_ptr(), written_length, written_length)
        };

        Some(info_log)
    }
}
