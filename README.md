
# alien_ffi

## Introduction

`alien_ffi` is an OCaml FFI providing a set of features for using external memory objects allocated in `rust` from `ocaml` in order to reduce copying between the two. 

## Building

The project can be built by using:
```
$ nix build .
```

Which will produce a set of artifacts in `result/lib`.
