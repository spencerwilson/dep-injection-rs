#!/usr/bin/env python

# This script runs CFFI in the (API level, out-of-line) mode,
# which produces a built Python C-extension module. This mode
# incurs minimal runtime cost but requires the most up-front work.
#
# Prerequisites: a C compiler is installed on your system.

from cffi import FFI

ffibuilder = FFI()

ffibuilder.cdef("""
    extern "Python" void cb_logger_log(void *, char *);

    void make_client(void *, void *);
    int add(void *, int, int);  // Do something, logging as a side effect
""")

ffibuilder.set_source("ext_module", 
    """
        void make_client(void *, void *);
        int add(void *, int, int);
    """,
    libraries=['rust_core'],
    # the following has no impact
    include_dirs=['/Users/ssw/code/di-rs/'])

if __name__ == "__main__":
    ffibuilder.compile(verbose=True)
