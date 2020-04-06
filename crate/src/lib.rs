#![recursion_limit = "512"]
mod app;
mod error;
mod evaluator;
mod parser;
mod result;
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
extern crate test_case;

use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run() {
    set_panic_hook();
    yew::start_app::<app::App>()
}
