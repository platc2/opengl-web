use gl;
use gl::RawHandle;

use gl_raw_handle_derive::RawHandle;

pub fn viewport(pos: (usize, usize), size: (usize, usize)) {
    unsafe { gl::Viewport(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _) };
}

pub fn scissor(pos: (usize, usize), size: (usize, usize)) {
    unsafe { gl::Scissor(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _) }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct BlendEquation(gl::GLenum);

pub const FUNC_ADD: BlendEquation = BlendEquation(gl::FUNC_ADD);
pub const FUNC_SUBTRACT: BlendEquation = BlendEquation(gl::FUNC_SUBTRACT);
pub const FUNC_REVERSE_SUBTRACT: BlendEquation = BlendEquation(gl::FUNC_REVERSE_SUBTRACT);

pub fn blend_equation(blend_equation: BlendEquation) {
    unsafe { gl::BlendEquation(blend_equation.raw_handle()) };
}

pub fn blend_equation_separate(blend_equation_rgb: BlendEquation, blend_equation_alpha: BlendEquation) {
    unsafe { gl::BlendEquationSeparate(blend_equation_rgb.raw_handle(), blend_equation_alpha.raw_handle()) };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, RawHandle)]
pub struct BlendFuncFactor(gl::GLenum);

pub const ZERO: BlendFuncFactor = BlendFuncFactor(gl::ZERO);
pub const ONE: BlendFuncFactor = BlendFuncFactor(gl::ONE);
pub const SRC_COLOR: BlendFuncFactor = BlendFuncFactor(gl::SRC_COLOR);
pub const ONE_MINUS_SRC_COLOR: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_SRC_COLOR);
pub const DST_COLOR: BlendFuncFactor = BlendFuncFactor(gl::DST_COLOR);
pub const ONE_MINUS_DST_COLOR: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_DST_COLOR);
pub const SRC_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::SRC_ALPHA);
pub const ONE_MINUS_SRC_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_SRC_ALPHA);
pub const DST_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::DST_ALPHA);
pub const ONE_MINUS_DST_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_DST_ALPHA);
pub const CONSTANT_COLOR: BlendFuncFactor = BlendFuncFactor(gl::CONSTANT_COLOR);
pub const ONE_MINUS_CONSTANT_COLOR: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_CONSTANT_COLOR);
pub const CONSTANT_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::CONSTANT_ALPHA);
pub const ONE_MINUS_CONSTANT_ALPHA: BlendFuncFactor = BlendFuncFactor(gl::ONE_MINUS_CONSTANT_ALPHA);
// TODO - This value must not be used as dfactor
pub const SRC_ALPHA_SATURATE: BlendFuncFactor = BlendFuncFactor(gl::SRC_ALPHA_SATURATE);

pub fn blend_func_separate(
    blend_func_factor_source_rgb: BlendFuncFactor,
    blend_func_factor_destination_rgb: BlendFuncFactor,
    blend_func_factor_source_alpha: BlendFuncFactor,
    blend_func_factor_destination_alpha: BlendFuncFactor,
) {
    unsafe {
        gl::BlendFuncSeparate(
            blend_func_factor_source_rgb.raw_handle(),
            blend_func_factor_destination_rgb.raw_handle(),
            blend_func_factor_source_alpha.raw_handle(),
            blend_func_factor_destination_alpha.raw_handle(),
        )
    };
}
