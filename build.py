#!/usr/bin/env python

# This script runs CFFI in the (API level, out-of-line) mode,
# which produces a built Python C-extension module. This mode
# incurs minimal runtime cost but requires the most up-front work.
#
# Prerequisites: a C compiler is installed on your system.

from cffi import FFI

ffibuilder = FFI()

ffibuilder.cdef("""
    extern "Python" void log_msg(char *);

    int add(int, int);  // Do something, calling `log` as a side effect
""")

ffibuilder.set_source("ext_module", 
    r"""
        int add(int, int);
    """,
    libraries=[])

if __name__ == "__main__":
    ffibuilder.compile(verbose=True)
