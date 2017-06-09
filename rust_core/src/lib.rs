extern crate libc;

pub mod extern_api;
pub mod logger;

use logger::Logger;

/// An SDK client.
///
/// Contains incredible patent-pending tech such as "integer addition".
pub struct Client<L> where
    L: Logger
{
    logger: L,
}

impl<L> Client<L> where L: Logger, {
    fn new(logger: L) -> Client<L> {
        println!("Rust: Making a client");

        Client {
            logger,
        }
    }

    fn add(&self, a: u32, b: u32) -> u32 {
        let result = a + b;
        self.logger.log(&format!("Rust computed {} + {} = {}", a, b, result));
        result
    }
}
