{
  system,
  pkgs,
  lib ? pkgs.lib,
  stdenv ? pkgs.stdenv,
  crane,
  fenix,
  ...
}: let
  # fenix: rustup replacement for reproducible builds
  toolchain = fenix.${system}.fromToolchainFile {
    file = ./rust-toolchain.toml;
    sha256 = "sha256-6eN/GKzjVSjEhGO9FhWObkRFaE1Jf+uqMSdQnb8lcB4=";
  };
  # crane: cargo and artifacts manager
  craneLib = crane.${system}.overrideToolchain toolchain;

  nativeBuildInputs = with pkgs; [
    esbuild
    worker-build
    wasm-pack
    wasm-bindgen-cli
    binaryen
  ];

  buildInputs = with pkgs; [
    openssl
    pkg-config
    # autoPatchelfHook
  ]
  ++ lib.optionals stdenv.buildPlatform.isDarwin [
    pkgs.libiconv
  ];
  # ++ lib.optionals stdenv.buildPlatform.isLinux [
  #   pkgs.libxkbcommon.dev
  # ];

  worker = craneLib.buildPackage {
    doCheck = false;
    src = craneLib.cleanCargoSource (craneLib.path ./.);
    buildPhaseCargoCommand = "HOME=$(mktemp -d fake-homeXXXX) worker-build --release --mode no-install";

    installPhaseCommand = ''
      cp -r ./build $out
    '';

    nativeBuildInputs = with pkgs; nativeBuildInputs ++ [
      esbuild
    ];

    inherit buildInputs;
  };
in
{
  # `nix build`
  packages.default = worker;

  # `nix develop`
  devShells.default = craneLib.devShell {
    buildInputs = 
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
        # pkgs-stable.wrangler
        (wrangler.overrideAttrs (final: prev: {
            installPhase = ''
    runHook preInstall
    mkdir -p $out/bin $out/lib $out/lib/packages/wrangler
    rm -rf node_modules/typescript node_modules/eslint node_modules/prettier node_modules/bin node_modules/.bin node_modules/**/bin node_modules/**/.bin
    cp -r node_modules $out/lib
    cp -r packages/miniflare $out/lib/packages
    cp -r packages/workers-tsconfig $out/lib/packages
    cp -r packages/wrangler/node_modules $out/lib/packages/wrangler
    cp -r packages/wrangler/templates $out/lib/packages/wrangler
    cp -r packages/wrangler/wrangler-dist $out/lib/packages/wrangler
    rm -rf $out/lib/**/bin $out/lib/**/.bin
    cp -r packages/wrangler/bin $out/lib/packages/wrangler
    NODE_PATH_ARRAY=( "$out/lib/node_modules" "$out/lib/packages/wrangler/node_modules" )
    makeWrapper ${lib.getExe nodejs} $out/bin/wrangler \
      --inherit-argv0 \
      --prefix-each NODE_PATH : "$${NODE_PATH_ARRAY[@]}" \
      --add-flags $out/lib/packages/wrangler/bin/wrangler.js \
      --set-default SSL_CERT_FILE "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" # https://github.com/cloudflare/workers-sdk/issues/3264
    runHook postInstall
            '';
        }))
      ]);
  };
}
