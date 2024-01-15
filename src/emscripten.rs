#[cfg(target_os = "emscripten")]
pub mod emscripten {
    use std::cell::RefCell;
    use std::os::raw::{c_float, c_int, c_void, c_uint};
    use std::ptr::null_mut;

    #[allow(non_camel_case_types)]
    type em_callback_func = unsafe extern fn();

    extern {
        pub fn emscripten_set_main_loop(func: em_callback_func, fps: c_int, simulate_infinite_loop: c_int);
        pub fn emscripten_cancel_main_loop();
        pub fn emscripten_get_now() -> c_float;
        pub fn emscripten_sleep(ms: c_uint);
    }

    thread_local!(static MAIN_LOOP_CALLBACK: RefCell<*mut c_void> = RefCell::new(null_mut()));

    pub fn sleep(ms: u32) {
        unsafe { emscripten_sleep(ms as c_uint) };
    }

    pub fn set_main_loop_callback<F>(callback: F) where F: FnMut() {
        MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
        });

        unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

        unsafe extern "C" fn wrapper<F>() where F: FnMut() {
            MAIN_LOOP_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)();
            });
        }
    }
}
