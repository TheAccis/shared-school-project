{
	description = "Python 3.12 dev shell";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; };
				python = pkgs.python312.withPackages (ps: with ps; [
					scikit-learn
					matplotlib
					pandas
					pygame
					numpy
				]);
			in {
				devShells.default = pkgs.mkShell {
					packages = [
						python
					];
				};
			}
		);
}