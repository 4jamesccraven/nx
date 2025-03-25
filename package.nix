{lib, pkgs}:

with pkgs;
rustPlatform.buildRustPackage {
  pname = "nx";
  version = "0.1.0";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [
    installShellFiles
  ];

  preFixup = ''
    mkdir completions
    $out/bin/nx completions bash > completions/nx.bash
    $out/bin/nx completions zsh > completions/nx.zsh
    $out/bin/nx completions fish > completions/nx.fish

    installShellCompletion completions/*
  '';

  meta = {
    license = lib.licenses.mit;
    mainProgram = "nx";
  };
}
