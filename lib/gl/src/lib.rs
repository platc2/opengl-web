extern crate gl_raw_handle_derive;

pub use sys::load_with;

pub mod shader;
pub mod program;
pub mod buffer;
pub mod vertex_array;
pub mod vertex_attrib;
pub mod rendering;
pub mod state;
pub mod capabilities;

mod gl {
    pub use sys::*;
    pub use sys::types::*;
}

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub trait RawHandle<T> {
        unsafe fn raw_handle(&self) -> T;
    }
}

