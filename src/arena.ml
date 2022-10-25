
(* A typed arena is allocated not as an opaque type but as a Bigstring.t *)

type caml_external_memory = Bigstring.t

(* A bigarray arena is a region of memory containing Vec<u8> in Rust
   for representing bigarrays.
 *)

type t = caml_external_memory

external alloc : unit -> t = "arena_alloc"
external free : t -> unit = "arena_free"

