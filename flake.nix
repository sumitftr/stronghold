{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, ... } @inputs: let
    systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];
    
    perSystemOutputs = system: let

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };
      
      rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
        extensions = [ "rust-src" ];
        targets = [ "wasm32-unknown-unknown" ];
      };
      
      rustBuildInputs = (with pkgs; [ openssl libiconv pkg-config ])
        ++ pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [
          pkg-config openssl glib gtk3 libsoup_3 webkitgtk_4_1 xdotool cairo pango gdk-pixbuf atk wasm-bindgen-cli_0_2_108
        ])
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
          SystemConfiguration IOKit Carbon WebKit Security Cocoa
        ]);
      
    in {
      devShells.default = pkgs.mkShell {
        name = "chessreed";
        buildInputs = rustBuildInputs ++ [ pkgs.stdenv.cc.cc.lib ];
        nativeBuildInputs = with pkgs; [
          rustToolchain
          wasm-bindgen-cli_0_2_108
          dioxus-cli
          tailwindcss_4
        ];
        
        shellHook = pkgs.lib.optionalString pkgs.stdenv.isLinux ''
          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath (rustBuildInputs ++ [ pkgs.stdenv.cc.cc.lib ])}:$LD_LIBRARY_PATH
          export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules
          
          # Critical: This tells cargo to use autoPatchelfHook behavior
          export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=${pkgs.stdenv.cc}/bin/cc
          export CC=${pkgs.stdenv.cc}/bin/cc
          export CXX=${pkgs.stdenv.cc}/bin/c++
          
          sudo ${pkgs.iptables}/bin/iptables -I INPUT -p tcp --dport 8080 -j ACCEPT 2>/dev/null || true
        '' + pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
          export DYLD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath rustBuildInputs}:$DYLD_LIBRARY_PATH
        '';
        
        env.RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
      };
      formatter = pkgs.nixfmt-rfc-style;
    };
    
  in {
    devShells = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).devShells);
    formatter = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).formatter);
  };
}
