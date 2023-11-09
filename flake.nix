{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";
  inputs.parts.inputs.nixpkgs-lib.follows = "nixpkgs";

  outputs = inputs@{ parts, nci, ... }:
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" ];
      imports = [ nci.flakeModule ];
      perSystem = { pkgs, config, ... }:
        let
          # TODO: change this to your crate's name
          crateName = "todo";
          # shorthand for accessing this crate's outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          crateOutputs = config.nci.outputs.${crateName};

          libraries = with pkgs; [ ];
        in {
          # declare projects
          # relPath is the relative path of a project to the flake root
          # TODO: change this to your crate's path
          nci.projects.${crateName}.path = ./.;
          #        nci.projects.outputs.${crateName}.packages = [pkgs.hello];
          # configure crates
          nci.crates.${crateName} = {
            # export crate (packages and devshell) in flake outputs
            # alternatively you can access the outputs and export them yourself (see below)
            # export = true;
            # look at documentation for more options
            # overrides = {
            # add-inupts.overrideAttrs = old: {
            # buildInputs = with pkgs; [ entr gtk4 cmake ];

            # shellHook = ''
            # export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
            # export LD_LIBRARY_PATH=${
            # pkgs.lib.makeLibraryPath libraries
            # }:$LD_LIBRARY_PATH
            # export PKG_CONFIG_PATH="${pkgs.pkg-config}"; 
            # '';
            # ${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            #            RUSTIC = "rgv";
            #       buildInputs = [pkgs.hello];
            #              nativeBuildInputs = [pkgs.pkg-config pkgs.gobject-introspection];
            # };
            # };
            #          runtimeLibs = [pkgs.pkg-config];
          };
          # export the crate devshell as the default devshell
          devShells.default = crateOutputs.devShell;
          # export the release package of the crate as default package
          packages.default = crateOutputs.packages.release;
        };
    };
}
