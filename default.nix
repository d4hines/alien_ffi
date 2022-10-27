{ pkgs ? import <nixpkgs>, ... }:
{
  libalien_ffi_c = pkgs.stdenv.mkDerivation {
    name = "libalien_ffi_c";
    version = "0.0.1";
    src = ./src/libalien_ffi_c/.;
    buildInputs = [ pkgs.ocaml-ng.ocamlPackages_5_00.ocaml ];
    buildPhase = "make";
    installPhase = ''
      mkdir -p $out/lib
      cp libalien_ffi_c.a $out/lib/libalien_ffi_c.a
      cp libalien_ffi_c.so $out/lib/libalien_ffi_c.so
    '';
  };

  libalien_ffi_rs = pkgs.rustPlatform.buildRustPackage {
    pname = "libalien_ffi_rs";
    version = "0.0.1";
    src = ./.;
    cargoBuildFlags = "-p libalien_ffi_rs";
    
    cargoLock = {
      lockFile = ./Cargo.lock;
    };
    
    nativeBuildInputs = [ pkgs.pkg-config ];
  };

  alien_ffi = ocamlPackages: with ocamlPackages; buildDunePackage {
    pname = "alien_ffi";
    version = "0.0.1";

    src = ./src;

    propagatedBuildInputs = [
      bigstring
    ];
  };
}
