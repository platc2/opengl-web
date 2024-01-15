use thiserror::Error;

use gl::sys::RawHandle;

use crate::program::Error::ProgramLink;
use crate::shader::Shader;

mod gl {
    pub use gl::program::*;
    pub use gl::sys;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Program failed to link: {0}")]
    ProgramLink(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Program {
    id: gl::ProgramId,
}

impl Program {
    /// # Errors
    /// - Program failed to link
    pub fn from_shaders(shaders: &[&Shader]) -> Result<Self> {
        let id = gl::create_program();

        for shader in shaders {
            gl::attach_shader(id, shader.id());
        }
        gl::link_program(id);
        for shader in shaders {
            gl::detach_shader(id, shader.id());
        }

        let link_successful = gl::program_link_status(id);
        let info_log = gl::program_info_log(id);
        if link_successful {
            if let Some(info_log) = info_log { println!("Program linked successfully: {}", info_log); }
            Ok(Program { id })
        } else {
            Err(ProgramLink(info_log.unwrap_or(String::from("Unknown error"))))
        }
    }

    pub fn set_used(&self) {
        gl::use_program(self.id);
    }

    #[must_use]
    pub fn id(&self) -> gl::ProgramId {
        self.id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl::delete_program(&mut self.id);
    }
}
