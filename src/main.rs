extern crate sdl2;

use std::ffi::CStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use sdl2::video::GLProfile;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

pub fn main() {
    let sdl_context = sdl2::init()
        .expect("Failed to initialize SDL2");
    let hint = unsafe { CStr::from_ptr(sdl2::sys::SDL_HINT_EMSCRIPTEN_KEYBOARD_ELEMENT.as_ptr() as *const _) }
        .to_str()
        .unwrap();
    sdl2::hint::set(hint, "#canvas");
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
    let gl_context = window.gl_create_context();
    match gl_context {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s).cast::<std::ffi::c_void>());

    let mut event_pump = sdl_context.event_pump()
        .expect("Failed to retrieve event pump");

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
                gl::ClearColor(1f32, 0f32, 0f32, 1f32);
                gl::Clear(gl::COLOR_BUFFER_BIT);
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
