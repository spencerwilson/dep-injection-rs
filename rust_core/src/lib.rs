extern crate libc;

mod extern_api;
mod logger;

/// An SDK client.
///
/// Contains incredible patent-pending tech such as "integer addition".
pub struct Client<T: logger::Logger> {
    logger: T,
}

/// The pluggable bits of a client.
pub struct Plugins<T> where
    T: logger::Logger,
{
    logger: T,
}

impl<T: logger::Logger> Client<T> {
    fn new(plugins: Plugins<T>) -> Self {
        Self {
            logger: plugins.logger,
        }
    }

    fn add(&self, a: u32, b: u32) -> u32 {
        let result = a + b;
        self.logger.log(&format!("Rust computed {} + {} = {}", a, b, result));
        a + b
    }
}
