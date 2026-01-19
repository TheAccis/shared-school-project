{
	description = "Rust dev shell with wgpu (rust-src included)";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
		flake-utils.url = "github:numtide/flake-utils";
		rust-overlay.url = "github:oxalica/rust-overlay";
	};

	outputs = { self, nixpkgs, flake-utils, rust-overlay }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				overlays = [ rust-overlay.overlays.default ];
				pkgs = import nixpkgs { inherit system overlays; };

				rust = pkgs.rust-bin.stable.latest.default.override {
					extensions = [ "rust-src" "rustfmt" "clippy" ];
					targets = [ "wasm32-unknown-unknown" ];
				};

				libs = with pkgs; 
				[
					# build tools
          wasm-tools
					cmake
					ninja
					git

					# X11
					xorg.libX11
					xorg.libXcursor
					xorg.libXrandr
					xorg.libXi
					xorg.libXinerama
					xorg.libXrender
					xorg.libXpresent

					# XCB
					xorg.xrandr
					xorg.xdpyinfo
					xorg.libxcb
					xorg.xcbutil
					xorg.xcbutilwm
					xorg.xcbutilimage
					xorg.xcbutilkeysyms
					xorg.xcbutilrenderutil

					# Keyboard
					libxkbcommon

					# Wayland
					wayland
					wayland-protocols

					# Graphics
					vulkan-loader
					mesa
					libGL

					# Sound
					alsa-lib
				];

			in {
				devShells.default = pkgs.mkShell {
					buildInputs = with pkgs; [
						# Rust
						rust
						rust-analyzer
						pkg-config
						lldb
					] ++ libs;

					nativeBuildInputs = [ pkgs.pkg-config ];

					shellHook = ''
						export CMAKE_POLICY_VERSION_MINIMUM=3.31

						export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
							pkgs.lib.makeLibraryPath libs
						}"

						echo "🦀 Rust development environment activated!"
						echo "🦀 Rust: $(rustc --version)"
					'';
				};
			});
}