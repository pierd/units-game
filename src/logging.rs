use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn js_log(s: &str);
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        #[allow(unused_unsafe)]
        unsafe {
            $crate::logging::js_log(&format!($($arg)*));
        }
    }}
}
