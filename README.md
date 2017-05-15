# Dependency injection from Python into Rust

A demo of calling a Python callback from Rust.

End goal is a Rust-backed Python module `sdk` that creates SDK clients which
can be configured at construction-time with custom implementations of internal components
(e.g., logging, HTTP requesting, etc.) defined in Python.

The following is an example with a Logger interface.

```py
import sdk

class Logger(object):
    def log(msg):
        print('got:', msg)

client = sdk.Client(logger=Logger)  # Instantiate an SDK client

result = client.add(1, 2)           # Should log 'got: computed 1 + 2 = 3'
assert result == 3
```