trait Logger {
    /// Logs the given message.
    fn log(msg: &str);
}

/// A Rust interface to Loggers defined in foreign languages.
struct ExternLogger {
    // TODO: Transmute to an Option<fn(msg: *const c_char)>; Option to handle nullability
    // Works because Option<&T>::None has the same byte-level representation as a null *T
    log: Option<extern "C" fn(msg: *const c_char)>,
}

impl Logger for ExternLogger {
    fn log(&self, msg: &str) {
        match self.log {
            Some(l) => l(msg),
            None => { /* no-op */ },
        };
    }
}
