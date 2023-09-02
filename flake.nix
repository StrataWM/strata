{
  description = "strata - A cutting-edge, robust and sleek Wayland compositor with batteries included. ";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    rust.url = "github:oxalica/rust-overlay";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    parts,
    crane,
    rust,
    nix-filter,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["aarch64-linux" "x86_64-linux"];

      perSystem = {
        self',
        lib,
        system,
        ...
      }: let
        pkgs = nixpkgs.legacyPackages.${system}.extend rust.overlays.default;
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = crane.lib.${system}.overrideToolchain rust-toolchain;
        craneArgs = {
          pname = "strata";
          version = self.rev or "dirty";

          src = nix-filter.lib.filter {
            root = ./.;
            include = [
              ./src
              ./Cargo.toml
              ./Cargo.lock
              ./resources
            ];
            exclude = [
              ./lua
            ];
          };
          cmakeFlags = ["PREFIX=~/.local"];

          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            autoPatchelfHook
          ];

          buildInputs = with pkgs; [
            fontconfig
            stdenv.cc.cc.lib
            wayland
            libdrm
            libGL
            libffi
            libinput
            libseat
            libwacom
            libxkbcommon
            mesa # For libgbm
            systemd # For libudev
          ];
          runtimeDependencies = with pkgs; [
            libGL
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly craneArgs;
        strata = craneLib.buildPackage (craneArgs // {inherit cargoArtifacts;});
      in {
        apps.strata = {
          type = "app";
          program = lib.getExe self'.packages.default;
        };

        checks.strata = strata;
        packages.default = strata;

        devShells.default = pkgs.mkShell {
          # Should there be packages here or use Nix purely for CI?
          LD_LIBRARY_PATH = lib.makeLibraryPath (__concatMap (d: d.runtimeDependencies) (__attrValues self'.checks));
          # Basic Developing Components using `nix develop`
          buildInputs = with pkgs; [
            rust-bin.nightly.latest.default
            rust-analyzer
            rustfmt
            pkg-config
            cmake
            autoPatchelfHook
            fontconfig
            stdenv.cc.cc.lib
            wayland
            libdrm
            libGL
            libffi
            libinput
            libseat
            libwacom
            libxkbcommon
            mesa # For libgbm
            systemd # For libudev
          ];
        };
      };
    };
}
