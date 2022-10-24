
open Alien_ffi

let () =
  let open Alcotest in
  let id () =
    let a = Arena.alloc () in Arena.free a
  in
  run "suite-name"
    [
      ( "test-a",
        [
          test_case "First" `Quick id;
        ] )
    ]
