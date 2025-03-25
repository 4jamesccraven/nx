{lib, pkgs}:

with pkgs;
rustPlatform.buildRustPackage {
  pname = "nx";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    license = lib.licenses.mit;
    mainProgram = "nx";
  };
}
