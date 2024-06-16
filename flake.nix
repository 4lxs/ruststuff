{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";

    # rust
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
    just-flake.url = "github:juspay/just-flake";
    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
  };

  outputs =
    { crane
    , advisory-db
    , fenix
    , ...
    } @ inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.just-flake.flakeModule
        inputs.pre-commit-hooks-nix.flakeModule
      ];
      perSystem =
        { config
        , self'
        , pkgs
        , lib
        , system
        , ...
        }:
        let
          craneLib = crane.mkLib pkgs;
          craneLibLLvmTools =
            craneLib.overrideToolchain
              (fenix.packages.${system}.complete.withComponents [
                "cargo"
                "llvm-tools"
                "rustc"
              ]);

          mkCrateOutputs = srcpath: name:
            let
              src = craneLib.cleanCargoSource srcpath;

              commonArgs = {
                inherit src;
                strictDeps = true;

                buildInputs = lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
              };

              cargoArtifacts = craneLib.buildDepsOnly commonArgs;

              package = craneLib.buildPackage (commonArgs
                // {
                inherit cargoArtifacts;
              });
            in
            {
              checks = {
                "${name}" = package;

                "${name}-clippy" = craneLib.cargoClippy (commonArgs
                  // {
                  inherit cargoArtifacts;
                  cargoClippyExtraArgs = "--all-targets -- --deny warnings";
                });

                "${name}-doc" = craneLib.cargoDoc (commonArgs
                  // {
                  inherit cargoArtifacts;
                });

                "${name}-fmt" = craneLib.cargoFmt {
                  inherit src;
                };

                "${name}-audit" = craneLib.cargoAudit {
                  inherit src advisory-db;
                };

                "${name}-deny" = craneLib.cargoDeny {
                  inherit src;
                };

                "${name}-nextest" = craneLib.cargoNextest (commonArgs
                  // {
                  inherit cargoArtifacts;
                  partitions = 1;
                  partitionType = "count";
                });
              };

              packages =
                {
                  "${name}" = package;
                }
                // lib.optionalAttrs (!pkgs.stdenv.isDarwin) {
                  "${name}-llvm-coverage" = craneLibLLvmTools.cargoLlvmCov (commonArgs
                  // {
                    inherit cargoArtifacts;
                  });
                };
            };

          projs = {
            json_parser = mkCrateOutputs ./json_parser "json_parser";
            ds = mkCrateOutputs ./ds "ds";
          };
        in
        {
          checks = lib.mkMerge (lib.mapAttrsToList (name: proj: proj.checks) projs);

          packages =
            {
              default = projs.json_parser.packages.json_parser;
            }
            // lib.mkMerge (lib.mapAttrsToList (name: proj: proj.packages) projs);

          just-flake.features = {
            treefmt.enable = true;
            rust.enable = true;
            convco.enable = true;
          };

          treefmt.config = {
            projectRootFile = "flake.nix";
            flakeCheck = false; # pre-commit-hooks.nix checks this
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };

          pre-commit = {
            check.enable = true;
            settings = {
              hooks = {
                treefmt.enable = true;
                convco.enable = true;
              };
            };
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
              config.just-flake.outputs.devShell
              config.pre-commit.devShell
              (craneLib.devShell { inherit (self') checks; })
            ];
            packages = [
              pkgs.cargo-watch
              config.pre-commit.settings.tools.convco
              pkgs.alejandra
            ];
          };
        };
    };
}
