# SPDX-FileCopyrightText: 2024-2025 László Vaskó <opensource@vlaci.email>
#
# SPDX-License-Identifier: EUPL-1.2

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    git-hooks = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      git-hooks,
      ...
    }:

    let
      supportedSystems = [
        "x86_64-linux"
        "x86_64-darwin"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
        }
      );

      pre-commit-check = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        git-hooks.lib.${system}.run {
          src = ./.;
          package = pkgs.prek;
          hooks = {
            check-added-large-files.enable = true;
            end-of-file-fixer.enable = true;
            check-yaml.enable = true;
            check-toml.enable = true;
            nixfmt.enable = true;
            statix.enable = true;
            deadnix.enable = true;
            cargo-check.enable = true;
            clippy = {
              enable = true;
              packageOverrides.cargo = pkgs.cargo;
              packageOverrides.clippy = pkgs.clippy;
            };
            rustfmt.enable = true;
            reuse.enable = true;
          };
        }
      );
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            inherit (pre-commit-check.${system}) shellHook;
            packages =
              with pkgs;
              [
                cargo
                rustc
                rustfmt
                clippy
                janet
              ]
              ++ pre-commit-check.${system}.enabledPackages;
          };
        }
      );
      formatter = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          inherit (pre-commit-check.${system}.config) package configFile;
          script = ''
            ${pkgs.lib.getExe package} run --all-files --config ${configFile}
          '';
        in
        pkgs.writeShellScriptBin "pre-commit-run" script
      );
    };
}
