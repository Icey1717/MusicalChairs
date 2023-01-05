extern crate web_sys;

// A macro to provide `log!(..)`-style syntax for `console.log` logging.
#[cfg(target_family = "wasm")]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

#[cfg(target_family = "windows")]
macro_rules! log {
    ( $( $t:tt )* ) => {
        println!{$($t)*}
    }
}

pub(crate) use log;
