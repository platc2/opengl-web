extern crate anyhow;
extern crate sdl2;

#[cfg(target_os = "emscripten")]
use std::ffi::CStr;
use std::path::Path;

use anyhow::Result;
use sdl2::mouse::MouseButton;
use sdl2::video::GLProfile;

use crate::key_codes::KeyCodes;
use crate::mouse_buttons::MouseButtons;
use crate::program::Program;
use crate::shader::{Kind, Shader};

mod gl {
    pub use gl::buffer::*;
    pub use gl::load_with;
    pub use gl::program::*;
    pub use gl::rendering::*;
    pub use gl::state::*;
    pub use gl::sys;
    pub use gl::vertex_array::*;
    pub use gl::vertex_attrib::*;
}

#[cfg(target_os = "emscripten")]
pub mod emscripten;
mod resources;
mod shader;
mod program;
mod imgui_wrapper;
mod texture;
mod key_codes;
mod mouse_buttons;

pub fn main() -> Result<()> {
    #[cfg(target_os = "emscripten")]
        let hint = unsafe { CStr::from_ptr(sdl2::sys::SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT.as_ptr() as *const _) }
        .to_str()
        .map(|hint| sdl2::hint::set(hint, "#canvas"))
        .expect("Failed to set emscripten keyboard element for SDL");
    #[cfg(target_os = "emscripten")]
    sdl2::hint::set("SDL_EMSCRIPTEN_ASYNCIFY", "1");
    let sdl_context = sdl2::init()
        .expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video()
        .expect("Failed to initialize SDL video subsystem");
    let _gl_attr = {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::GLES);
        gl_attr.set_context_minor_version(0);
        gl_attr.set_context_major_version(3);
        gl_attr
    };
    let window = video_subsystem.window("My Window", 500, 500)
        .opengl()
        .build()
        .expect("Failed to create window!");
    let _gl_context = window.gl_create_context()
        .expect("Failed to create OpenGL context");
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<std::ffi::c_void>());

    let mut event_pump = sdl_context.event_pump()
        .expect("Failed to retrieve event pump");

    let resource = resources::Resources::from_relative_exe_path(Path::new("assets"))?;

    let vertex_buffer = initialize_vertices();
    let vertex_shader = {
        let src = resource.load_string("vertex.glsl")?;
        Shader::from_source(src.as_str(), Kind::Vertex)?
    };
    let fragmnet_shader = {
        let src = resource.load_string("fragment.glsl")?;
        Shader::from_source(src.as_str(), Kind::Fragment)?
    };

    let program = Program::from_shaders(&[
        &vertex_shader, &fragmnet_shader])?;

    gl::viewport((0, 0), (500, 500));

    let vao = gl::gen_vertex_array();
    gl::bind_vertex_array(vao);
    gl::bind_buffer(gl::ARRAY_BUFFER, vertex_buffer);
    gl::enable_vertex_attrib_array(0);
    gl::enable_vertex_attrib_array(1);

    gl::vertex_attrib_pointer(0, gl::SIZE_2, gl::FLOAT, false, 5 * std::mem::size_of::<f32>(), 0);
    gl::vertex_attrib_pointer(1, gl::SIZE_3, gl::FLOAT, false, 5 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>());
    gl::bind_vertex_array(gl::NO_VERTEX_ARRAY);

    let mut imgui_context = imgui_wrapper::Imgui::init();

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);
    let mut chars: Vec<char> = Vec::new();
    let mut gamma = 1f32;

    let mut main_loop = || {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (
                        // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                        i16::try_from(x).unwrap_or(0),
                        i16::try_from(y).unwrap_or(0),
                    );
                }
                Event::MouseButtonDown { mouse_btn, .. } => mouse_buttons[mouse_btn] = true,
                Event::MouseButtonUp { mouse_btn, .. } => mouse_buttons[mouse_btn] = false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    key_codes[keycode] = true;

                    let keycode = keycode as u32;
                    if (32..512).contains(&keycode) {
                        chars.push(char::from_u32(keycode).unwrap());
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => key_codes[keycode] = false,
                Event::Quit { .. } => return false,
                _ => {}
            }
        }

        imgui_context.prepare(
            [500f32, 500f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [
                mouse_buttons[MouseButton::Left],
                mouse_buttons[MouseButton::Right],
            ],
            &mut chars,
        );

        gl::clear_color(0xFF000000);
        gl::clear(gl::COLOR);

        gl::bind_vertex_array(vao);
        program.set_used();

        let uniform_location = gl::uniform_location(program.id(), "gamma");
        gl::uniform(uniform_location, gamma);
        gl::draw_arrays(gl::TRIANGLES, 0, 3);

        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.slider("Gamma", 0.5f32, 2.5f32, &mut gamma);
                    if ui.button("Reset (1.0)") {
                        gamma = 1f32;
                    }
                    ui.same_line();
                    if ui.button("Reset (2.2)") {
                        gamma = 2.2f32;
                    }
                });
        });

        #[cfg(not(target_os = "emscripten"))]
        window.gl_swap_window();

        true
    };

    /*
        #[cfg(target_os = "emscripten")]
        {
            use emscripten::emscripten;
            emscripten::set_main_loop_callback(main_loop);
            {
                let _ = imgui_context;
            }
            Ok(())
        }
    */

    loop {
        if !main_loop() {
            break;
        }

        #[cfg(target_os = "emscripten")]
        emscripten::emscripten::sleep(16);
    }

    Ok(())
}

fn initialize_vertices() -> gl::BufferId {
    let vertices = vec![
        -0.5f32, -0.5f32,
        1f32, 0f32, 0f32,
        0.5f32, -0.5f32,
        0f32, 1f32, 0f32,
        0f32, 0.5f32,
        0f32, 0f32, 1f32,
    ];
    let buffer = gl::gen_buffer();
    gl::bind_buffer(gl::ARRAY_BUFFER, buffer);
    gl::buffer_data(gl::ARRAY_BUFFER, vertices.as_slice(), gl::STREAM_DRAW);

    buffer
}

