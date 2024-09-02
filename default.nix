inputs @ {
  pkgs,
  lib ? pkgs.lib,
  stdenv ? pkgs.stdenv,
  crane,
  fenix,
  flake-utils,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-3jVIIf5XPnUU1CRaTyAiO0XHVbJl12MSx3eucTXCjtE=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.overrideToolchain toolchain;

  nativeBuildInputs = with pkgs; [
    esbuild
    worker-build
    wasm-pack
    wasm-bindgen-cli
    binaryen
  ];

  buildInputs = with pkgs;
    [
      openssl
      pkg-config
    ]
    ++ lib.optionals stdenv.buildPlatform.isDarwin [
      pkgs.libiconv
    ];

  cargoToml = path:
    craneLib.crateNameFromCargoToml {
      src = craneLib.cleanCargoSource path;
    };

  commonArgs = {
    inherit buildInputs nativeBuildInputs;

    pname = "worker";
    strictDeps = true;
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
  };

  cargoArtifacts = craneLib.buildDepsOnly commonArgs;

  worker = craneLib.buildPackage (commonArgs
    // {
      inherit cargoArtifacts;

      pname = "worker";
      inherit (cargoToml ./crates/backend) version;

      buildPhaseCargoCommand = ''
        cd crates/backend

        HOME=$(mktemp -d fake-homeXXXX)
        worker-build --release --mode no-install
      '';
      installPhaseCommand = "cp -r build $out";
    });

  devShellBuildInputs =
    nativeBuildInputs
    ++ buildInputs
    ++ (with pkgs; [
      cargo-make
      taplo

      # deno
      nodejs
      nodePackages.pnpm
      # nodePackages.prettier
      # nodePackages.typescript-language-server
      # nodePackages.svelte-language-server
      (import ./nix/wrangler.nix inputs)
    ]);
in {
  # `nix run .#zellij`
  apps.zellij = flake-utils.lib.mkApp {
    drv = pkgs.writeShellApplication {
      name = "run-zellij";

      runtimeInputs = with pkgs;
        devShellBuildInputs
        ++ [
          zellij
        ];

      text = ''
        zellij --layout ${./nix/zellij/layout.kdl}
      '';
    };
  };

  # `nix build`
  packages.default = worker;

  # `nix develop`
  devShells = {
    default = craneLib.devShell {
      buildInputs =
        devShellBuildInputs;
    };
  };
}
