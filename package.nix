{ lib, pkgs }:

with pkgs;
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = "nx";
  version = manifest.version;

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    installShellFiles
    makeBinaryWrapper
  ];

  postInstall = ''
    installShellCompletion --cmd nx \
      --bash <(COMPLETE=bash $out/bin/nx) \
      --zsh  <(COMPLETE=zsh $out/bin/nx) \
      --fish <(COMPLETE=fish $out/bin/nx) \
  '';

  postFixup = ''
    wrapProgram $out/bin/nx \
      --prefix PATH : ${lib.makeBinPath [ nh ]}
  '';

  meta = {
    license = lib.licenses.gpl3;
    mainProgram = "nx";
  };
}
