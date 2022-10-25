(* Ensures performing a full GC after using the FFI. *)
let with_opaques f =
  f (); Gc.full_major ()

(* Ensures an arena is free'd. *)
let with_arena f =
  let arena = Arena.alloc () in
  f arena;
  Arena.free arena

let with_full f =
  with_opaques (fun () -> with_arena f)
