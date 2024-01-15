use shader::ShaderId;

use ::{gl, gl::RawHandle};
use gl_raw_handle_derive::RawHandle;

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct ProgramId(gl::GLuint);

pub const NO_PROGRAM: ProgramId = ProgramId(0);

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct UniformLocation(gl::GLint);

#[must_use]
pub fn create_program() -> ProgramId {
    let id = unsafe { gl::CreateProgram() };
    ProgramId(id)
}

pub fn delete_program(program_id: &mut ProgramId) {
    unsafe { gl::DeleteProgram(program_id.raw_handle()) };
    program_id.0 = 0;
}

pub fn attach_shader(program_id: ProgramId, shader_id: ShaderId) {
    unsafe { gl::AttachShader(program_id.raw_handle(), shader_id.raw_handle()) };
}

pub fn detach_shader(program_id: ProgramId, shader_id: ShaderId) {
    unsafe { gl::DetachShader(program_id.raw_handle(), shader_id.raw_handle()) };
}

pub fn link_program(program_id: ProgramId) {
    unsafe { gl::LinkProgram(program_id.raw_handle()) };
}

#[must_use]
pub fn program_link_status(program_id: ProgramId) -> bool {
    let mut success: gl::GLint = 0;
    unsafe { gl::GetProgramiv(program_id.raw_handle(), gl::LINK_STATUS, &mut success) };
    return success != 0;
}

#[must_use]
pub fn program_info_log(program_id: ProgramId) -> Option<String> {
    let mut log_len: gl::GLint = 0;

    unsafe { gl::GetProgramiv(program_id.raw_handle(), gl::INFO_LOG_LENGTH, &mut log_len) };

    if log_len == 0 {
        None
    } else {
        // log_len includes null termination character, which we do not require
        let mut info_log_buffer = Vec::with_capacity(log_len as usize);
        let mut written_length: gl::GLsizei = 0;
        let info_log_buffer_ptr = info_log_buffer.spare_capacity_mut().as_ptr() as *mut gl::GLchar;
        let info_log = unsafe {
            gl::GetProgramInfoLog(program_id.raw_handle(), log_len, &mut written_length, info_log_buffer_ptr);
            info_log_buffer.set_len(std::cmp::max(written_length, log_len) as usize);
            let written_length = written_length as usize;
            String::from_raw_parts(info_log_buffer.as_mut_ptr(), written_length, written_length)
        };

        Some(info_log)
    }
}

pub fn use_program(program_id: ProgramId) {
    unsafe { gl::UseProgram(program_id.raw_handle()); }
}

pub fn uniform_location<T: Into<String>>(program_id: ProgramId, name: T) -> UniformLocation {
    let name: std::ffi::CString = std::ffi::CString::new(name.into())
        .expect("Null character found in uniform name!");
    let id = unsafe { gl::GetUniformLocation(program_id.raw_handle(), name.as_ptr()) };
    UniformLocation(id)
}

pub trait UniformValue {
    fn upload(&self, uniform_location: UniformLocation);
}

impl UniformValue for f32 {
    fn upload(&self, uniform_location: UniformLocation) {
        unsafe { gl::Uniform1f(uniform_location.raw_handle(), *self) };
    }
}

impl UniformValue for i32 {
    fn upload(&self, uniform_location: UniformLocation) {
        unsafe { gl::Uniform1i(uniform_location.raw_handle(), *self) };
    }
}

pub fn uniform<T: UniformValue>(uniform_location: UniformLocation, uniform_value: T) {
    uniform_value.upload(uniform_location);
}

pub trait UniformMatrix4Value {
    fn upload(&self, transposed: bool, uniform_location: UniformLocation);
}

impl UniformMatrix4Value for &[f32] {
    fn upload(&self, transposed: bool, uniform_location: UniformLocation) {
        unsafe { gl::UniformMatrix4fv(uniform_location.raw_handle(), 1, transposed as _, self.as_ptr()) };
    }
}

pub fn uniform_matrix4<T: UniformMatrix4Value>(uniform_location: UniformLocation, transposed: bool, uniform_value: T) {
    uniform_value.upload(transposed, uniform_location);
}
