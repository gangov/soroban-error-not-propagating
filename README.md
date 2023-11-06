## Problem description

Escalating panic messages between contracts seems to always return `HostError: Error(Value, InvalidInput)`
All the tests inside `sandbox/src/test.rs` fail, while in `token/srs/test.rs` work just fine.

## How to reproduce

To reproduce the issue, go into `sandbox` directory and run `make test`.

