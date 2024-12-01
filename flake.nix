{
  description = "Leptos Template";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.${system}.nixfmt-rfc-style;

        # Development Shell.
        # $> nix develop
        devShells = {
          # https://v2.tauri.app/start/prerequisites/#linux
          default = pkgs.mkShell rec {
            buildInputs = with pkgs; [
              at-spi2-atk
              atkmm
              cairo
              curl
              ffmpeg
              gdk-pixbuf
              glib
              gobject-introspection
              gobject-introspection.dev
              gst_all_1.gstreamer
              gst_all_1.gst-libav
              gst_all_1.gst-vaapi
              gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good
              gst_all_1.gst-plugins-bad
              gst_all_1.gst-plugins-ugly
              gtk3
              harfbuzz
              librsvg
              libsoup_3
              openssl
              pkg-config
              pango
              webkitgtk_4_1
              webkitgtk_4_1.dev
            ];

            RUST_BACKTRACE = 1;

            # https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

            shellHook = ''
              cargo install create-tauri-app --locked
              cargo install tauri-cli --version '^2.0.0' --locked
              cargo install trunk --locked

              cargo install leptosfmt

              export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath buildInputs}:$LD_LIBRARY_PATH
            '';
          };
        };
      }
    );
}
