{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs-unstable";
    };

    workerd = {
      url = "github:getchoo/workerd-docker";
      inputs.nixpkgs.follows = "nixpkgs-unstable";
    };
  };

  outputs = {
    self,
    nixpkgs,
    nixpkgs-unstable,
    flake-utils,
    ...
  } @ inputs:
  # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        bundle = import ./. rec {
          inherit system flake-utils;
          pkgs = nixpkgs-unstable.legacyPackages.${system};
          pkgs-stable = nixpkgs.legacyPackages.${system};
          crane = inputs.crane.lib;
          fenix = inputs.fenix.packages;
          workerd = pkgs.callPackage ./workerd.nix {};
        };
      in {
        inherit (bundle) packages apps devShells;
      }
    );
}
