{
  pkgs,

  lib ? pkgs.lib,

  nodejs ? pkgs.nodejs,
  ...
}:
pkgs.wrangler.overrideAttrs (final: prev: {
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
})
