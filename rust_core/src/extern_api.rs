use libc::c_void;

use {Client, Plugins};
use logger;

/// Get ch, fn ptrs to static callbacks, make client, return ptr to struct
#[no_mangle]
pub extern "C" fn make_client<T: logger::Logger>(ch: c_void, cb_logger_log: fn()) -> *const Client<T> {
    let plugins = Plugins {
        logger: logger::ExternLogger::new(ch, cb_logger_log),
    };

    let client = Box::new(Client::new(plugins));
    Box::into_raw(client) as *const Client<T>
}

#[no_mangle]
pub extern "C" fn add<T: logger::Logger>(client: &Client<T>, a: u32, b: u32) -> u32 {
    client.add(a, b)
}
