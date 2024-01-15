use std::any::Any;
use std::ffi::c_void;

use imgui::{FontAtlas, TextureId};
#[cfg(not(target_os = "emscripten"))]
use imgui::BackendFlags;

use gl::sys::RawHandle;

use crate::program::Program;
use crate::shader;
use crate::shader::Shader;
use crate::texture::Texture;

mod gl {
    pub use gl::buffer::*;
    pub use gl::capabilities::*;
    pub use gl::program::*;
    pub use gl::state::*;
    pub use gl::sys;
    pub use gl::vertex_array::*;
    pub use gl::vertex_attrib::*;
}

const IMGUI_VERTEX_SHADER_SOURCE: &str = include_str!("imgui.vert");
const IMGUI_FRAGMENT_SHADER_SOURCE: &str = include_str!("imgui.frag");

#[derive(Debug)]
pub struct Imgui {
    context: imgui::Context,
    program: Program,
    proj_matrix_uniform_location: gl::UniformLocation,
    texture_uniform_location: gl::UniformLocation,
    vao: gl::VertexArrayId,
    _vbo: gl::BufferId,
    element_buffer_object: gl::BufferId,
}

type WindowDimension = [f32; 2];
type MousePos = [f32; 2];
type MouseButtonState = [bool; 2];

impl Imgui {
    #[must_use]
    pub fn init() -> Self {
        let mut context = imgui::Context::create();
        #[cfg(not(target_os = "emscripten"))]
        {
            context.io_mut().backend_flags = BackendFlags::RENDERER_HAS_VTX_OFFSET;
        }

        let _font_texture = generate_font_texture_from_atlas(context.fonts());
        let program = create_program();
        let vertex_buffer_object = gl::gen_buffer();
        let element_buffer_object = gl::gen_buffer();
        gl::bind_buffer(gl::ARRAY_BUFFER, vertex_buffer_object);
        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_object);
        let vao = gl::gen_vertex_array();

        let vtx_size = std::mem::size_of::<imgui::DrawVert>();
        program.set_used();
        gl::bind_vertex_array(vao);
        gl::enable_vertex_attrib_array(0);
        gl::enable_vertex_attrib_array(1);
        gl::enable_vertex_attrib_array(2);
        gl::vertex_attrib_pointer(0, gl::SIZE_2, gl::FLOAT, false, vtx_size, 0);
        gl::vertex_attrib_pointer(1, gl::SIZE_2, gl::FLOAT, false, vtx_size, 2 * std::mem::size_of::<f32>());
        gl::vertex_attrib_pointer(2, gl::SIZE_3, gl::UNSIGNED_BYTE, true, vtx_size, 4 * std::mem::size_of::<f32>());
        gl::bind_vertex_array(gl::NO_VERTEX_ARRAY);
        gl::bind_buffer(gl::ARRAY_BUFFER, gl::NO_BUFFER);
        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, gl::NO_BUFFER);

        let proj_matrix_uniform_location = gl::uniform_location(program.id(), "ProjMtx");
        let texture_uniform_location = gl::uniform_location(program.id(), "Texture");

        Self {
            context,
            program,
            proj_matrix_uniform_location,
            texture_uniform_location,
            vao,
            _vbo: vertex_buffer_object,
            element_buffer_object,
        }
    }

    pub fn prepare(
        &mut self,
        window_dimension: WindowDimension,
        mouse_pos: MousePos,
        mouse_button_state: MouseButtonState,
        chars: &mut Vec<char>,
    ) {
        let io = self.context.io_mut();
        io.display_size = window_dimension;
        io.delta_time = 1f32 / 60f32;

        io.mouse_pos = mouse_pos;
        io.mouse_down[0] = mouse_button_state[0];
        io.mouse_down[1] = mouse_button_state[1];

        for char in chars.iter() {
            io.add_input_character(*char);
        }
        chars.truncate(0);
    }

    /// # Panics
    /// - Unimplemented draw command
    #[allow(clippy::too_many_lines)]
    pub fn render<F>(&mut self, mut callback: F)
        where
            F: FnMut(&imgui::Ui),
    {
        let ui = self.context.frame();
        callback(ui);
        let draw_data = self.context.render();

        let message = "ImGui Rendering";
        // Message length is guaranteed to not exceed 31bits
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        unsafe {
            /*
                        gl::PushDebugGroup(
                            gl::DEBUG_SOURCE_APPLICATION,
                            2 as GLuint,
                            message.len() as GLsizei,
                            message.as_ptr().cast(),
                        );
            */
        }

        gl::enable(gl::BLEND);
        gl::blend_equation(gl::FUNC_ADD);
        gl::blend_func_separate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        gl::disable(gl::CULL_FACE);
        gl::disable(gl::DEPTH_TEST);
        gl::disable(gl::STENCIL_TEST);
        gl::enable(gl::SCISSOR_TEST);
        #[cfg(not(target_os = "emscripten"))]
        unsafe { gl::sys::PolygonMode(gl::sys::FRONT_AND_BACK, gl::sys::FILL) };

        let [display_pos_x, display_pos_y] = draw_data.display_pos;
        let [display_size_w, display_size_h] = draw_data.display_size;
        let frame_buffer_width = display_pos_x + display_size_w;
        let frame_buffer_height = display_pos_y + display_size_h;
        gl::viewport(
            (display_pos_x as _, display_pos_y as _),
            (display_size_w as _, display_size_h as _));
        let ortho = nalgebra_glm::ortho(
            display_pos_x,
            display_pos_x + display_size_w,
            display_pos_y + display_size_h,
            display_pos_y,
            -1f32,
            1f32,
        );
        self.program.set_used();
        gl::uniform(self.texture_uniform_location, 0);
        gl::uniform_matrix4(self.proj_matrix_uniform_location, false, nalgebra_glm::value_ptr(&ortho));
        gl::bind_vertex_array(self.vao);
        gl::bind_buffer(gl::ARRAY_BUFFER, self._vbo);
        gl::bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object);
        unsafe {
            gl::sys::ActiveTexture(gl::sys::TEXTURE0);
        }
        for draw_list in draw_data.draw_lists() {
            let vtx_buffer = draw_list.vtx_buffer();
            let idx_buffer = draw_list.idx_buffer();

            gl::buffer_data(gl::ARRAY_BUFFER, vtx_buffer, gl::STREAM_DRAW);
            gl::buffer_data(gl::ELEMENT_ARRAY_BUFFER, idx_buffer, gl::STREAM_DRAW);

            for command in draw_list.commands() {
                match command {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        let clip_rect = cmd_params.clip_rect;
                        let clip_rect = [
                            clip_rect[0] - display_pos_x,
                            clip_rect[1] - display_pos_y,
                            clip_rect[2] - display_pos_x,
                            clip_rect[3] - display_pos_y,
                        ];

                        let vtx_offset = cmd_params.vtx_offset;
                        let idx_offset = cmd_params.idx_offset * std::mem::size_of::<imgui::DrawIdx>();
                        if clip_rect[0] < frame_buffer_width
                            && clip_rect[1] < frame_buffer_height
                            && clip_rect[2] >= 0f32
                            && clip_rect[3] >= 0f32
                        {
                            gl::scissor(
                                (clip_rect[0] as _, (frame_buffer_height - clip_rect[3]) as _),
                                ((clip_rect[2] - clip_rect[0]) as _, (clip_rect[3] - clip_rect[1]) as _));

                            unsafe {
                                gl::sys::BindTexture(
                                    gl::sys::TEXTURE_2D,
                                    gl::sys::types::GLuint::try_from(cmd_params.texture_id.id())
                                        .unwrap_unchecked(),
                                );
                            }
                            let gl_type = match std::mem::size_of::<imgui::DrawIdx>() {
                                2 => gl::sys::UNSIGNED_SHORT,
                                _ => gl::sys::UNSIGNED_INT,
                            };
                            #[cfg(not(target_os = "emscripten"))]
                            unsafe {
                                gl::sys::DrawElementsBaseVertex(
                                    gl::sys::TRIANGLES,
                                    gl::sys::types::GLsizei::try_from(count).unwrap_unchecked(),
                                    gl_type,
                                    idx_offset as *const c_void,
                                    vtx_offset as gl::sys::types::GLint,
                                );
                            }
                            #[cfg(target_os = "emscripten")]
                            unsafe {
                                gl::sys::DrawElements(
                                    gl::sys::TRIANGLES,
                                    gl::sys::types::GLsizei::try_from(count).unwrap_unchecked(),
                                    gl_type,
                                    idx_offset as *const c_void);
                            }
                        }
                    }
                    x => {
                        panic!("Unimplemented! {:?}", x.type_id());
                    }
                }
            }
        }

        gl::disable(gl::BLEND);
        gl::enable(gl::CULL_FACE);
        gl::disable(gl::SCISSOR_TEST);
        /*
                    gl::PopDebugGroup();
        */
    }
}

fn generate_font_texture_from_atlas(font_atlas: &mut FontAtlas) -> Texture {
    let font_atlas_texture = &mut font_atlas.build_rgba32_texture();
    let font_texture = Texture::from_raw(
        font_atlas_texture.data,
        font_atlas_texture.width as usize,
        font_atlas_texture.height as usize,
    )
        .expect("Failed to create font texture for Dear ImGui");
    font_atlas.tex_id = TextureId::new(font_texture.handle() as usize);
    font_texture
}

fn create_program() -> Program {
    let vertex_shader = Shader::from_source(IMGUI_VERTEX_SHADER_SOURCE, shader::Kind::Vertex)
        .expect("Failed to setup Dear ImGui vertex shader");
    let fragment_shader = Shader::from_source(IMGUI_FRAGMENT_SHADER_SOURCE, shader::Kind::Fragment)
        .expect("Failed to setup Dear ImGui fragment shader");
    Program::from_shaders(&[&vertex_shader, &fragment_shader])
        .expect("Failed to setup Dear ImGui program")
}
