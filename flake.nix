{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    workerd = {
      url = "github:getchoo/workerd-docker";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
  # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        bundle = import ./. {
          inherit system flake-utils;
          pkgs = nixpkgs.legacyPackages.${system};
          crane = inputs.crane.lib;
          fenix = inputs.fenix.packages;
          workerd = inputs.workerd.packages.${system}.workerd;
        };
      in {
        inherit (bundle) packages apps devShells;
      }
    );
}
