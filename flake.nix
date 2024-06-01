{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };


    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, advisory-db, crane }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-darwin" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };

        craneLib = (crane.mkLib nixpkgs.legacyPackages.${system}).overrideToolchain
          fenix.packages.${system}.stable.toolchain;

        version = "0.1.0-alpha";

        src = pkgs.lib.cleanSourceWith {
          src = ./.;

          filter = path: type:
            (craneLib.filterCargoSources path type)
          ;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        commonArgs = {
          inherit version;
          inherit src;
          name = "slack_http";
          pname = "slack_http";
          strictDeps = true;
          doCheck = false;

          buildInputs = [
            pkgs.openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.CoreFoundation
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        slack_http = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          inherit slack_http;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets";
          });

          formatting = craneLib.cargoFmt (commonArgs // {
            inherit src;
            name = "slack_http";
          });

          audit = craneLib.cargoAudit {
            inherit src advisory-db;
            name = "slack_http";
          };

          # FIXME: WHY
          # deny-license = craneLib.cargoDeny {
          #   inherit src;
          #   name = "slack_http";
          # };
        };

        packages = {
          default = slack_http;
          inherit slack_http;

          # NOTE: Many things
          # 1. Do not push this to a binary cache. This is just meant for
          # testing purposes.
          # 2. Move to `checks` attribute when `--impure` allows getEnv to
          # read env vars.
          # 3. Run this with `nix build .#packages.aarch64-darwin.slack_http_test --impure`
          # cause the the env variables won't be read otherwise.
          slack_http_test = craneLib.cargoNextest (commonArgs // {
            inherit cargoArtifacts;
            doCheck = true;

            SLACK_USER_ACCESS_TOKEN = builtins.getEnv "SLACK_USER_ACCESS_TOKEN";
            SLACK_BOT_ACCESS_TOKEN = builtins.getEnv "SLACK_BOT_ACCESS_TOKEN";
            SLACK_TEAM_ID = builtins.getEnv "SLACK_TEAM_ID";

            partitions = 1;
            partitionType = "count";
          });
        };

        devShells = {
          default = craneLib.devShell {
            inputsFrom = [ slack_http ];

            shellHook = ''
              set -a
              source env.sh
              set +a
            '';

            packages = with pkgs; [
              cargo-watch
              nixpkgs-fmt
              nil
              libiconv
            ];
          };
        };
      });
}
