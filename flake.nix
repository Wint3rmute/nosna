{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs;
          lib.makeLibraryPath [
            libGL
            libxkbcommon
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ];

      in
      {
        defaultPackage = naersk-lib.buildPackage {
          root = ./.;
          nativeBuildInputs = with pkgs; [
            pkgs.pkg-config
            pkgs.alsa-lib
            pkgs.xorg.libXcursor
            pkgs.xorg.libXrandr
            pkgs.xorg.libXi
            pkgs.makeWrapper

            pkgs.libGL
            pkgs.libGLU
          ];
          LD_LIBRARY_PATH = libPath;
          postInstall = ''
            wrapProgram "$out/bin/nosna" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

          buildInputs = with pkgs; [ xorg.libxcb ];
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };

        # devShell = with pkgs;
        #   mkShell {
        #     nativeBuildInputs = [
        #       alsa-lib
        #       xorg.libXcursor
        #       xorg.libXi
        #       xorg.libXrandr
        #       xorg.libX11
        #     ];

        #     libPath = with pkgs;
        #       lib.makeLibraryPath [
        #         libGL
        #         libGLU
        #         libxkbcommon
        #         wayland
        #         alsa-lib
        #       ];

        #     buildInputs = [
        #       libGL
        #       libGLU
        #       pkg-config

        #       cargo
        #       rustc
        #       rustfmt
        #       pre-commit
        #       rustPackages.clippy
        #     ];
        #     RUST_SRC_PATH = rustPlatform.rustLibSrc;
        #   };
      });
}
