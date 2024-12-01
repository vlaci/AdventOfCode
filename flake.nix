# SPDX-FileCopyrightText: 2024 László Vaskó <vlaci@fastmail.com>
#
# SPDX-License-Identifier: EUPL-1.2

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };
  outputs =
    {
      self,
      nixpkgs,
      devenv,
      ...
    }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system} = {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
        devenv-test = self.devShells.${system}.default.config.test;
      };

      devShells.${system}.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [ (import ./devenv.nix) ];
      };
    };
}
