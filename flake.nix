{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
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
          ];
          # Todo: move to a dev shell or something once
          # I figure out how to do it without breaking openGL
          buildInputs = [
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.clippy
            # pkgs.rustdoc
          ];
          LD_LIBRARY_PATH = libPath;
          postInstall = ''
            wrapProgram "$out/bin/nosna" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';
        };

        defaultApp = utils.lib.mkApp { drv = self.defaultPackage."${system}"; };

      });
}
