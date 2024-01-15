extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

const BINDNGS_OUTPUT_FILE: &'static str = "bindings.rs";

type ApiVersion = (u8, u8);

fn main() {
    let out_dir = env::var("OUT_DIR")
        .expect("Couldn't find build directory from 'OUT_DIR' environment variable!");
    let mut file_gl = File::create(&Path::new(&out_dir).join(BINDNGS_OUTPUT_FILE))
        .expect("Failed to create gl bindings file!");
    let (api, version) = get_api_and_version();
    Registry::new(api, version, Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file_gl)
        .expect("Failed to write gl bindings!");
}

fn get_api_and_version() -> (Api, ApiVersion) {
    if cfg!(target_os = "emscripten") {
        (Api::Gles2, (3, 0))
    } else {
        (Api::Gl, (4, 5))
    }
}
