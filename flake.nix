{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs:
    with inputs; let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      toolchain = fenix.packages.${system}.stable.toolchain;
    in {
      devShells.x86_64-linux = {
        default = pkgs.mkShell {
          # Libraries
          buildInputs = [];
          # Additional tooling
          packages = with pkgs; [toolchain cargo-release];
        };
      };
    };
}
