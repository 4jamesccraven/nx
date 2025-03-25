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

  preFixp = ''
    mkdir completions
    cp $out/_nx completions
    cp $out/nx.bash completions
    cp $out/nx.fish completions

    installShellCompletion completions/*
  '';

  meta = {
    license = lib.licenses.mit;
    mainProgram = "nx";
  };
}
