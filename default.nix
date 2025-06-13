{
  pkgs,
  lib ? pkgs.lib,
  stdenv ? pkgs.stdenv,
  crane,
  fenix,
  wrangler-fix,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-KUm16pHj+cRedf8vxs/Hd2YWxpOrWZ7UOrwhILdSJBU=";
  };

  # crane: cargo and artifacts manager
  craneLib = crane.overrideToolchain toolchain;

  nativeBuildInputs = with pkgs; [
    worker-build
    wasm-pack
    wasm-bindgen-cli
    binaryen
  ];

  buildInputs = with pkgs;
    [
      openssl
      pkg-config
      autoPatchelfHook
    ]
    ++ lib.optionals stdenv.buildPlatform.isDarwin [
      pkgs.libiconv
    ];

  cargoToml = path:
    craneLib.crateNameFromCargoToml {
      src = craneLib.cleanCargoSource path;
    };

  worker = craneLib.buildPackage {
    pname = "worker";
    inherit (cargoToml ./crates/backend) version;
    doCheck = false;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.unions [
        ./Cargo.toml
        ./Cargo.lock
        ./crates/backend
        ./crates/models
        ./crates/shared
      ];
    };
    buildPhaseCargoCommand = ''
      cd crates/backend
      HOME=$(mktemp -d fake-homeXXXX) worker-build --release --mode no-install
      cd ../..
    '';

    # Custom build command is provided, so this should be enabled
    doNotPostBuildInstallCargoBinaries = true;

    installPhaseCommand = ''
      cp -r ./crates/backend/build/ $out
    '';

    nativeBuildInputs = with pkgs; [esbuild] ++ nativeBuildInputs;

    inherit buildInputs;
  };
in {
  # `nix build .#backend`
  packages.backend = worker;

  # `nix develop`
  devShells = {
    default = craneLib.devShell {
      buildInputs =
        nativeBuildInputs
        ++ buildInputs
        ++ (with pkgs; [
          toolchain

          cargo-make
          taplo

          nodejs
          nodePackages.pnpm
          wrangler-fix.wrangler
        ]);
    };
  };
}
