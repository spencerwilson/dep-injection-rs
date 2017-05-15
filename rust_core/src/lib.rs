extern crate libc;

use libc::uint32_t;

struct Client {
    logger: 
}

impl Client {
    #[no_mangle]
    pub extern "C" fn add(&self, a: uint32_t, b: uint32_t) -> uint32_t {
        let result = a + b;

        // call logger

        
        a + b
    }

    /// Take a pointer to struct w/ fields of value fn* sufficient to make a Logger
    pub make_logger(ptr) -> Logger {

    }
}
