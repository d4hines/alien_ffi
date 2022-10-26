{
  description = "A flake for building `alien_ffi`";

  # Use existing `deku` cache for faster resolution of dependencies.
  nixConfig = {
    extra-substituters = "https://anmonteiro.nix-cache.workers.dev";
    extra-trusted-public-keys = "ocaml.nix-cache.com-1:/xI2h2+56rwFfKyyFVbkJSeGqSIYMC/Je+7XXqGKDIY=";
  };

  # Use the same `nixpkgs` as `deku` for `OCaml5`.
  inputs.nixpkgs.url = github:nix-ocaml/nix-overlays;
  # Use the same `rust-overlay` as `ZeroFX`.
  inputs.rust-overlay.url = github:oxalica/rust-overlay;
  inputs.flake-utils.follows = "rust-overlay/flake-utils";

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        code = pkgs.callPackage ./. { inherit nixpkgs system rust-overlay; };
      in rec {
        packages = {
          libalien_ffi_c = code.libalien_ffi_c;
          libalien_ffi_rs = code.libalien_ffi_rs;
          all = pkgs.symlinkJoin {
            name = "all";
            paths = with code; [ libalien_ffi_c libalien_ffi_rs ];
          };
          default = packages.all;
        };
      }
    );
}
