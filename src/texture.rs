use std::ffi::c_void;

use gl::sys::types::{GLenum, GLint, GLsizei, GLuint};
use stb_image::image::LoadResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageLoadingError {
    #[error("Image data invalid: {0}")]
    InvalidImage(String),

    #[error("Image format unsupported")]
    UnsupportedFormat,

    #[error("Resource error: {0}")]
    Resource(#[from] crate::resources::Error),

    #[error("Image is too large")]
    TooLarge,
}

type Result<T> = std::result::Result<T, ImageLoadingError>;

pub struct Texture {
    handle: GLuint,
    width: usize,
    height: usize,
}

struct Image {
    gl_type: GLenum,
    ptr: *const c_void,
    width: GLsizei,
    height: GLsizei,
    depth: usize,
}

impl Image {
    /// # Errors
    /// - [`Error::TooLarge`]
    pub fn from_byte(image: &stb_image::image::Image<u8>) -> Result<Self> {
        Self::from_type(gl::sys::UNSIGNED_BYTE, image)
    }

    /// # Errors
    /// - [`Error::TooLarge`]
    pub fn from_float(image: &stb_image::image::Image<f32>) -> Result<Self> {
        Self::from_type(gl::sys::FLOAT, image)
    }

    fn from_type<ImageType>(
        gl_type: GLenum,
        image: &stb_image::image::Image<ImageType>,
    ) -> Result<Self> {
        Ok(Self {
            gl_type,
            ptr: image.data.as_ptr().cast::<c_void>(),
            width: Self::convert_dimension(image.width)?,
            height: Self::convert_dimension(image.height)?,
            depth: image.depth,
        })
    }

    fn convert_dimension(dimension: usize) -> Result<GLsizei> {
        GLsizei::try_from(dimension).map_err(|_| ImageLoadingError::TooLarge)
    }
}

impl Texture {
    pub fn from_raw_1(image_data: &[u8], width: usize, height: usize) -> Result<Self> {
        let mut handle: GLuint = 0;

        let gl_width = GLsizei::try_from(width).expect("Too wide");
        let gl_height = GLsizei::try_from(height).expect("Too high");

        // TODO - Figure out why glTextureParameteri requires Glint while these values are GLenum
        let gl_linear = unsafe { GLint::try_from(gl::sys::LINEAR).unwrap_unchecked() };

        unsafe {
            #[cfg(target_os = "emscripten")]
            gl::sys::GenTextures(1, &mut handle);
            #[cfg(not(target_os = "emscripten"))]
            gl::sys::CreateTextures(gl::sys::TEXTURE_2D, 1 as GLsizei, &mut handle);
            gl::sys::BindTexture(gl::sys::TEXTURE_2D, handle);

            #[cfg(target_os = "emscripten")]
            {
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MIN_FILTER, gl_linear);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_WRAP_S, gl::sys::CLAMP_TO_EDGE as GLint);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_WRAP_T, gl::sys::CLAMP_TO_EDGE as GLint);
            }
            #[cfg(not(target_os = "emscripten"))]
            {
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MIN_FILTER, gl_linear);
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_WRAP_S, gl::sys::CLAMP_TO_EDGE as GLint);
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_WRAP_T, gl::sys::CLAMP_TO_EDGE as GLint);
            }
        }

        unsafe {
            gl::sys::TexImage2D(
                gl::sys::TEXTURE_2D,
                0 as GLint,
                gl::sys::R8 as GLint,
                gl_width,
                gl_height,
                0 as GLint,
                gl::sys::RED,
                gl::sys::UNSIGNED_BYTE,
                image_data.as_ptr().cast::<c_void>(),
            );
            #[cfg(target_os = "emscripten")]
            gl::sys::GenerateMipmap(gl::sys::TEXTURE_2D);
            #[cfg(not(target_os = "emscripten"))]
            gl::sys::GenerateTextureMipmap(handle);
        }
        Ok(Self {
            handle,
            width,
            height,
        })
    }

    /// # Errors
    pub fn from_raw(image_data: &[u8], width: usize, height: usize) -> Result<Self> {
        let mut handle: GLuint = 0;

        let gl_width = GLsizei::try_from(width).expect("Too wide");
        let gl_height = GLsizei::try_from(height).expect("Too high");

        // TODO - Figure out why glTextureParameteri requires Glint while these values are GLenum
        let gl_linear = unsafe { GLint::try_from(gl::sys::LINEAR).unwrap_unchecked() };
        let gl_rgba = unsafe { GLint::try_from(gl::sys::RGBA32F).unwrap_unchecked() };

        unsafe {
//            #[cfg(target_os = "emscripten")]
            gl::sys::GenTextures(1, &mut handle);
//            #[cfg(not(target_os = "emscripten"))]
//            gl::CreateTextures(gl::TEXTURE_2D, 1 as GLsizei, &mut handle);

            gl::sys::BindTexture(gl::sys::TEXTURE_2D, handle);

//            #[cfg(target_os = "emscripten")]
            {
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MIN_FILTER, gl_linear);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
            }
/*
            #[cfg(not(target_os = "emscripten"))]
            {
                gl::TextureParameteri(handle, gl::TEXTURE_MIN_FILTER, gl_linear);
                gl::TextureParameteri(handle, gl::TEXTURE_MAG_FILTER, gl_linear);
            }
*/
        }

        unsafe {
            gl::sys::TexImage2D(
                gl::sys::TEXTURE_2D,
                0 as GLint,
                gl::sys::RGBA as GLint,
                gl_width,
                gl_height,
                0 as GLint,
                gl::sys::RGBA,
                gl::sys::UNSIGNED_BYTE,
                image_data.as_ptr().cast::<c_void>(),
            );
//            #[cfg(target_os = "emscripten")]
//            gl::GenerateMipmap(gl::TEXTURE_2D);
//            #[cfg(not(target_os = "emscripten"))]
//            gl::GenerateTextureMipmap(handle);
        }
        Ok(Self {
            handle,
            width,
            height,
        })
    }
    /// # Errors
    /// - [`Error::InvalidImage`]
    /// - [`Error::UnsupportedFormat`]
    pub fn from(image_data: &mut [u8]) -> Result<Self> {
        let mut handle: GLuint = 0;

        // TODO - Figure out why glTextureParameteri requires Glint while these values are GLenum
        let gl_linear = unsafe { GLint::try_from(gl::sys::LINEAR).unwrap_unchecked() };
        let gl_rgba = unsafe { GLint::try_from(gl::sys::RGBA32F).unwrap_unchecked() };

        unsafe {
            #[cfg(target_os = "emscripten")]
            gl::sys::GenTextures(1, &mut handle);
            #[cfg(not(target_os = "emscripten"))]
            gl::sys::CreateTextures(gl::sys::TEXTURE_2D, 1 as GLsizei, &mut handle);
            gl::sys::BindTexture(gl::sys::TEXTURE_2D, handle);

            #[cfg(target_os = "emscripten")]
            {
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MIN_FILTER, gl::sys::LINEAR_MIPMAP_LINEAR as GLint);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
            }
            #[cfg(not(target_os = "emscripten"))]
            {
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MIN_FILTER, gl::sys::LINEAR_MIPMAP_LINEAR as GLint);
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
            }
        }

        // TODO - Figure out how to inline stb_image into match expression without the value being dropped too early
        let stb_image = stb_image::image::load_from_memory(image_data);
        let image_data = match &stb_image {
            LoadResult::Error(error) => Err(ImageLoadingError::InvalidImage(error.to_string())),
            LoadResult::ImageU8(image_data) => Ok(Image::from_byte(image_data)?),
            LoadResult::ImageF32(image_data) => Ok(Image::from_float(image_data)?),
        }?;

        let format = format_from_depth(image_data.depth)?;

        unsafe {
            gl::sys::TexImage2D(
                gl::sys::TEXTURE_2D,
                0 as GLint,
                gl_rgba,
                image_data.width,
                image_data.height,
                0 as GLint,
                format,
                image_data.gl_type,
                image_data.ptr,
            );
            #[cfg(target_os = "emscripten")]
            gl::sys::GenerateMipmap(gl::sys::TEXTURE_2D);
            #[cfg(not(target_os = "emscripten"))]
            gl::sys::GenerateTextureMipmap(handle);
        }
        // We don't require to check width & height as they've been validated above
        #[allow(clippy::cast_sign_loss)]
        Ok(Self {
            handle,
            width: image_data.width as usize,
            height: image_data.height as usize,
        })
    }

    #[must_use]
    pub fn blank(width: usize, height: usize) -> Self {
        let mut handle: GLuint = 0;

        let gl_width = GLsizei::try_from(width).expect("Width too large");
        let gl_height = GLsizei::try_from(height).expect("Height too large");

        // TODO - Figure out why glTextureParameteri requires Glint while these values are GLenum
        let gl_linear = unsafe { GLint::try_from(gl::sys::LINEAR).unwrap_unchecked() };
        let gl_rgba = unsafe { GLint::try_from(gl::sys::RGBA32F).unwrap_unchecked() };

        unsafe {
            #[cfg(target_os = "emscripten")]
            gl::sys::GenTextures(1, &mut handle);
            #[cfg(not(target_os = "emscripten"))]
            gl::sys::CreateTextures(gl::sys::TEXTURE_2D, 1 as GLsizei, &mut handle);
            gl::sys::BindTexture(gl::sys::TEXTURE_2D, handle);

            gl::sys::TexImage2D(
                gl::sys::TEXTURE_2D,
                0 as GLint,
                gl_rgba,
                gl_width,
                gl_height,
                0 as GLint,
                gl::sys::RGBA,
                gl::sys::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            #[cfg(target_os = "emscripten")]
            {
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MIN_FILTER, gl_linear);
                gl::sys::TexParameteri(gl::sys::TEXTURE_2D, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
            }
            #[cfg(not(target_os = "emscripten"))]
            {
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MIN_FILTER, gl_linear);
                gl::sys::TextureParameteri(handle, gl::sys::TEXTURE_MAG_FILTER, gl_linear);
            }
        }

        Self {
            handle,
            width,
            height,
        }
    }

    #[must_use]
    pub const fn handle(&self) -> GLuint {
        self.handle
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }
}

const fn format_from_depth(depth: usize) -> Result<GLenum> {
    match depth {
        1 => Ok(gl::sys::RED),
        2 => Ok(gl::sys::RG),
        3 => Ok(gl::sys::RGB),
        4 => Ok(gl::sys::RGBA),
        _ => Err(ImageLoadingError::UnsupportedFormat),
    }
}
