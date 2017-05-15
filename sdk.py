from ext_module import ffi, lib

@ffi.def_extern()
def log_msg(msg):
    print 'got:', msg

def add(a, b):
    return lib.add(a, b)
