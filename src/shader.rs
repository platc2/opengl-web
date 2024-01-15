use thiserror::Error;

use gl::sys::RawHandle;
pub use shader_kind::*;

use crate::resources::Resources;
use crate::shader::Error::ShaderCompilation;

mod gl {
    pub use gl::shader::*;
    pub use gl::sys;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Shader type could not be guessed from file extension: {0}")]
    UnsupportedFileExtension(String),

    #[error("Resource error: {0}")]
    Resource(#[from] crate::resources::Error),

    #[error("UTF-8 Error: {0}")]
    Utf8Encoding(#[from] core::str::Utf8Error),

    #[error("Shader failed to compile: {0}")]
    ShaderCompilation(String),

    #[error("Shader type is not supported: {0:?}")]
    ShaderTypeNotSupported(Kind),
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(target_os = "emscripten")]
mod shader_kind {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Kind {
        Vertex,
        Fragment,
    }

    impl Kind {
        pub fn gl_type(&self) -> gl::shader::ShaderKind {
            match self {
                Kind::Vertex => gl::shader::VERTEX_SHADER,
                Kind::Fragment => gl::shader::FRAGMENT_SHADER,
            }
        }
    }
}

#[cfg(not(target_os = "emscripten"))]
mod shader_kind {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Kind {
        Vertex,
        Fragment,
        Geometry,
        TessellationControl,
        TessellationEvaluation,
        Compute,
    }

    impl Kind {
        pub fn gl_type(&self) -> gl::shader::ShaderKind {
            match self {
                Kind::Vertex => gl::shader::VERTEX_SHADER,
                Kind::Fragment => gl::shader::FRAGMENT_SHADER,
                Kind::Geometry => gl::shader::GEOMETRY_SHADER,
                Kind::TessellationControl => gl::shader::TESSELLATION_CONTROL_SHADER,
                Kind::TessellationEvaluation => gl::shader::TESSELLATION_EVALUATION_SHADER,
                Kind::Compute => gl::shader::COMPUTE_SHADER,
            }
        }
    }
}

pub struct Shader {
    id: gl::ShaderId,
    kind: Kind,
}

impl Shader {
    /// # Errors
    /// - Shader compilation error
    pub fn from_res(res: &Resources, name: &str) -> Result<Self> {
        const POSSIBLE_EXT: [(&str, Kind); 2] =
            [(".vert", Kind::Vertex), (".frag", Kind::Fragment)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::UnsupportedFileExtension(String::from(name)))?;
        let source = res.load_cstring(name)?;

        Self::from_source(source.to_str()?, shader_kind)
    }

    /// # Errors
    /// - Shader compilation error
    pub fn from_source(source: &str, kind: Kind) -> Result<Self> {
        let gl_type = kind.gl_type();

//        let source = &CString::new(source).expect("Shader source contains invalid characters");
        let id = shader_from_source(source, gl_type)?;
        Ok(Self { id, kind })
    }

    #[must_use]
    pub fn id(&self) -> gl::ShaderId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }
}

fn shader_from_source(source: &str, kind: gl::ShaderKind) -> Result<gl::ShaderId> {
    let id = gl::create_shader(kind);

    gl::shader_source(id, source);
    gl::compile_shader(id);

    let compilation_successful = gl::shader_compile_status(id);
    let info_log = gl::shader_info_log(id);
    if compilation_successful {
        if let Some(info_log) = info_log { println!("Shader compiled successfully: {}", info_log); }
        Ok(id)
    } else {
        Err(ShaderCompilation(info_log.unwrap_or(String::from("Unknown error"))))
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::delete_shader(&mut self.id);
    }
}
