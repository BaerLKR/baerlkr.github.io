{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nixpkgs-for-wasm-bindgen.url = "github:NixOS/nixpkgs/4e6868b1aa3766ab1de169922bb3826143941973";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    nixpkgs-for-wasm-bindgen,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      inherit (pkgs) lib;
      rustToolchainFor = p:
        p.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
        };
      craneLib = ((crane.mkLib pkgs).overrideToolchain rustToolchainFor).overrideScope (_final: _prev: {
        inherit (import nixpkgs-for-wasm-bindgen {inherit system;}) wasm-bindgen-cli;
      });
      src = lib.cleanSourceWith {
        src = ./.;
        filter = path: type:
          (lib.hasSuffix "\.html" path)
          || (lib.hasSuffix "\.css" path)
          || (lib.hasInfix "/assets/" path)
          || (craneLib.filterCargoSources path type);
      };
      commonArgs = {
        inherit src;
        strictDeps = true;
        CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
        buildInputs = with pkgs;
          [
            openssl
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          doCheck = false;
        });
      my-app = craneLib.buildTrunkPackage (commonArgs
        // {
          inherit cargoArtifacts;
          wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
            version = "0.2.93";
            hash = "sha256-DDdu5mM3gneraM85pAepBXWn3TMofarVR4NbjMdz3r0=";
            cargoHash = "sha256-birrg+XABBHHKJxfTKAMSlmTVYLmnmqMDfRnmG6g/YQ=";
          };
        });
      serve-app = pkgs.writeShellScriptBin "serve-app" ''
        ${pkgs.python3Minimal}/bin/python3 -m http.server --directory ${my-app} 8000
      '';
    in {
      checks = {
        inherit my-app;
        my-app-clippy = craneLib.cargoClippy (commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });
        my-app-fmt = craneLib.cargoFmt {
          inherit src;
        };
      };
      packages.default = my-app;
      apps.default = flake-utils.lib.mkApp {
        drv = serve-app;
      };
      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
        packages = [
          pkgs.trunk
          pkgs.rust-analyzer
        ];
      };
    });
}
