from ext_module import ffi, lib

class Client(object):
    def __init__(self, **kwargs):
        # Keep handle alive at least as long as Client instance
        self.handle = ffi.new_handle(self)
        print "Python: In Client.__init__", self.handle

        # Configure logger, etc.
        self.log = kwargs.get("logger", self.default_logger)

        self.rust_client = lib.make_client(
            self.handle,
            lib.cb_logger_log,
        )
        print "Python: make_client returned", self.rust_client

    def add(self, a, b):
        print "Python: In Client.add; calling lib.add with rust_client", self.rust_client
        return lib.add(self.rust_client, a, b)

    def default_logger(self, message):
        print "DefaultPrefix:", message

@ffi.def_extern()
def cb_logger_log(client_handle, message):
    message = ffi.string(message)
    print "Python: In static callback, msg: '%s'" % message

    client = ffi.from_handle(client_handle)
    client.log(message)
