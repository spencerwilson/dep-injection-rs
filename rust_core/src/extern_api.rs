//! Extern functions enabling foreign languages to use the `Client` struct.

use libc::c_void;

use Client;
use logger::ExternLogger;

/// Constructs a `Client` with internal functionality defined by the given function pointers.
///
/// `ch` is a value stored within the `Client` instance that is always included among the
/// arguments passed to the extern callbacks. This is useful when one needs to preserve a
/// reference of some kind to a client instance in the extern language.
#[no_mangle]
pub extern "C" fn make_client(ch: *const c_void, cb_logger_log: fn()) -> *const Client<ExternLogger> {
    let logger = ExternLogger::new(ch, cb_logger_log);
    let client = Box::new(Client::new(logger));
    // TODO: check if std::mem::forget needs to be used in some way here
    Box::into_raw(client) as *const Client<ExternLogger>
}

/// Invokes the `add` method of the `Client` instance pointed to by the given pointer.
#[no_mangle]
pub extern "C" fn add(client_ptr: *const Client<ExternLogger>, a: u32, b: u32) -> u32 {
    println!("Rust: In add");
    let client = unsafe { &*client_ptr };
    client.add(a, b)
}
