
This is a set of functions for interfacing between OCaml and Rust with the sole purpose of never copying any data between the two.

The idea is not to provide a safe interface which tracks GC roots but rather to ensure that data is never copied in order to avoid potential attacks whilst binding cryptographic primitives.
