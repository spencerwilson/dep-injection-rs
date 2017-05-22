from ext_module import ffi, lib
print 'hello'

class Client(object):
    def __init__(self):
        client_handle = ffi.new_handle(self)
        self._handle = client_handle  # Keep handle alive at least as long as Client instance

        self.rust_client = lib.make_client(
            self._handle,
            cb_logger_log,
        )

    def add(self, a, b):
        return lib.add(self.rust_client, a, b)

    def log(self, message):
        print 'Python, default logger:', message

@ffi.def_extern()
def cb_logger_log(client_handle, message):
    client = ffi.from_handle(client_handle)
    client.log(message)
