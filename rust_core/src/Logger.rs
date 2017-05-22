use libc::{c_char, c_void};
use std;
use std::convert::From;

/// A trait for types that can record diagnostic messages.
pub trait Logger {
    /// Logs the given message.
    fn log(&self, msg: &str);
}

/// A Rust interface to Loggers defined in foreign languages.
pub struct ExternLogger {
    /// A handle of some kind to a Client instance in the extern language.
    /// This is received at construction-time from extern code, and is passed
    /// back to extern code when callbacks are called. This way, Rust
    /// conveys which extern instance the callback is related to.
    client_handle: c_void,

    // TODO compare w/ sending string across FFI boundary blog
    // Can we send a &str vs *const c_char
    log: Option<extern "C" fn(ch: &c_void, msg: &str)>,
}

impl ExternLogger {
    /// Constructs an ExternLogger from fn pointers received from extern code.
    pub fn new(ch: c_void, log: fn()) -> Self {
        ExternLogger {
            client_handle: ch,

            // Transmute to an Option<fn> to handle receiving a null ptr. Works because
            // Option<&T>::None has the same bit-level representation as a null *T.
            log: unsafe { std::mem::transmute(log) },
        }
    }
}

impl Logger for ExternLogger {
    fn log(&self, msg: &str) {
        match self.log {
            Some(l) => l(&self.client_handle, msg),
            None => { /* A null fn ptr was received from the extern interface. No-op. */ },
        };
    }
}
