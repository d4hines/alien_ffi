
let bench f =
  let t1 = Unix.gettimeofday () in
  let () = f () in
  let t2 = Unix.gettimeofday () in
  t2 -. t1

let bench msg ~items ~prepare run =
  let runs = 10 in
  let value = prepare () in
  let results =
    List.init runs (fun n ->
        let time = bench (fun () -> run value) in
        Format.eprintf "%s(%d): %.3f\n%!" msg n time;
        time)
  in
  let total = List.fold_left (fun acc time -> acc +. time) 0.0 results in
  let average = total /. Int.to_float runs in
  let per_second = Int.to_float items /. average in
  Format.eprintf "%s: %.3f\n%!" msg average;
  Format.eprintf "%s: %.3f/s\n%!" msg per_second
