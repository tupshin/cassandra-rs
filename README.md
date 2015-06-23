# cassandra-rs

This is a (hopefully) maintained rust project that unsafely
exposes the cpp driver at https://github.com/datastax/cpp-driver/
in a somewhat-sane crate.

You can use it from cargo with

    [dependencies.cql_ffi]
    git = "https://github.com/tupshin/cassandra-rs"

Or just

    [dependencies]
    cassandra="*"

If you're compiling on a OS X:

    export DYLD_LIBRARY_PATH=/usr/local/lib64
    export LIBRARY_PATH=/usr/local/lib64
