extern crate sdl2;

use std::ffi::CStr;
use std::io::Read;
use sdl2::video::GLProfile;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

pub fn main() {
    let sdl_context = sdl2::init()
        .expect("Failed to initialize SDL2");
    #[cfg(target_os = "emscripten")]
        let hint = unsafe { CStr::from_ptr(sdl2::sys::SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT.as_ptr() as *const _) }
        .to_str()
        .map(|hint| sdl2::hint::set(hint, "#canvas"))
        .expect("Failed to set emscripten keyboard element for SDL");
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

    let mut vertex_buffer = initialize_vertices();
    let mut program = load_program(
        load_shader(load_string("assets/vertex.glsl"), gl::VERTEX_SHADER),
        load_shader(load_string("assets/fragment.glsl"), gl::FRAGMENT_SHADER));
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);

        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei, (0 * std::mem::size_of::<f32>()) as *const _);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei, (2 * std::mem::size_of::<f32>()) as *const _);

        gl::UseProgram(program);
    }

    let mut main_loop = || {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => std::process::exit(1),
                sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                    println!("Key down: {:?}", keycode);
                }
                _ => {}
            }

            unsafe {
                gl::ClearColor(0f32, 0f32, 0f32, 1f32);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            #[cfg(not(target_os = "emscripten"))]
            window.gl_swap_window();
        }
    };

    #[cfg(target_os = "emscripten")]
    {
        use emscripten::emscripten;
        emscripten::set_main_loop_callback(main_loop)
    }

    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop();
    }
}

fn load_string(file_name: &str) -> std::ffi::CString {
    let mut file = std::fs::File::open(file_name)
        .unwrap();
    let file_len = file.metadata().unwrap().len() as usize;
    let mut buffer = Vec::with_capacity(file_len + 1);
    file.read_to_end(&mut buffer).unwrap();
    unsafe { std::ffi::CString::from_vec_unchecked(buffer) }
}

fn load_shader(source: std::ffi::CString, shader_type: gl::types::GLenum) -> gl::types::GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    unsafe {
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        // GL_INFO_LOG_LENGTH contains a positive number or 0 if no information is available
        let error_string_length = usize::try_from(len).unwrap_or(0);
        let mut error_string = String::with_capacity(error_string_length);
        error_string.extend([' '].iter().cycle().take(error_string_length));

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error_string.as_mut_ptr().cast());
        }

        println!("{}", error_string);
    }

    shader
}

fn load_program(vertex_shader: gl::types::GLuint, fragment_shader: gl::types::GLuint) -> gl::types::GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
    }

    program
}

fn initialize_vertices() -> gl::types::GLuint {
    let vertices = vec![
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32, 0.5f32, -0.5f32, 0f32, 1f32, 0f32, 0f32, 0.5f32, 0f32,
        0f32, 1f32,
    ];

    let size = std::mem::size_of::<f32>() * vertices.len();
    let mut buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer);
        gl::BufferData(gl::ARRAY_BUFFER, size as gl::types::GLsizeiptr, vertices.as_ptr().cast::<std::ffi::c_void>(), gl::STREAM_DRAW);
    };

    buffer
}

